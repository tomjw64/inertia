import { useRef, useEffect, useMemo, useCallback } from 'preact/hooks';
import style from './style.module.scss';
import { debounce } from 'lodash';
import {
  BufferGeometry,
  Color,
  CylinderGeometry,
  Fog,
  FogExp2,
  Frustum,
  Matrix4,
  Mesh,
  MeshBasicMaterial,
  PerspectiveCamera,
  Scene,
  SphereGeometry,
  Vector3,
  WebGLRenderer,
  WebGLRenderTarget,
} from 'three';
import {
  BlendShader,
  CopyShader,
  EffectComposer,
  FXAAShader,
  GammaCorrectionShader,
  RenderPass,
  SavePass,
  ShaderPass,
} from 'three/addons';
import { useLazyEffectRef } from '../../utils/hooks/use-lazy-ref';
import { mergeGeometries } from 'three/examples/jsm/utils/BufferGeometryUtils.js';

const getColorString = (r: number, g: number, b: number) =>
  `#${((r << 16) | (g << 8) | b).toString(16)}`;

const getCanvasHeight = () =>
  Math.floor(window.innerHeight * window.devicePixelRatio);

const getCanvasWidth = () =>
  Math.floor(window.innerWidth * window.devicePixelRatio);

const getCanvasAspectRatio = () => getCanvasWidth() / getCanvasHeight();

const Z_LIMIT = 1000;

const STAR_R = 0xff;
const STAR_G = 0xff;
const STAR_B = 0xff;

const STAR_COLOR = getColorString(STAR_R, STAR_B, STAR_G);

const RADIAL_GEOMETRY_SEGMENTS = 16;
const STAR_GEOMETRY = new CylinderGeometry(
  1,
  1,
  1,
  RADIAL_GEOMETRY_SEGMENTS,
).rotateX(Math.PI / 2);
const STAR_MATERIAL = new MeshBasicMaterial({ color: STAR_COLOR });

export const Starfield = ({
  numStars,
  speed,
}: {
  numStars: number;
  speed: number;
}) => {
  // numStars = 0;
  const canvas = useRef<HTMLCanvasElement>(null);

  const stars = useRef<
    {
      mesh: Mesh;
      relativeCoords: { x: number; y: number };
    }[]
  >([]);

  const animationFrame = useRef<number>(0);

  const camera = useLazyEffectRef<PerspectiveCamera>(
    () => new PerspectiveCamera(50, getCanvasAspectRatio(), 50, Z_LIMIT),
  );
  const getFrustumFromCamera = useCallback(() => {
    const frustum = new Frustum();
    frustum.setFromProjectionMatrix(
      new Matrix4().multiplyMatrices(
        camera.current!.projectionMatrix,
        camera.current!.matrixWorldInverse,
      ),
    );
    return frustum;
  }, [camera]);
  const frustum = useLazyEffectRef<Frustum>(getFrustumFromCamera);
  const getCornersFromCamera = useCallback(() => {
    return {
      topLeft: new Vector3().set(-1, 1, 1).unproject(camera.current!),
      bottomRight: new Vector3().set(1, -1, 1).unproject(camera.current!),
    };
  }, [camera]);
  const corners = useLazyEffectRef<{
    topLeft: Vector3;
    bottomRight: Vector3;
  }>(getCornersFromCamera);
  const scene = useLazyEffectRef<Scene | null>(() => {
    const scene = new Scene();
    scene.background = new Color(0x373b55);
    scene.fog = new FogExp2(0x373b55, 0.0025);
    return scene;
  });
  const renderer = useLazyEffectRef<WebGLRenderer | null>(() => {
    const renderer = new WebGLRenderer({
      canvas: canvas.current!,
    });
    return renderer;
  });
  const composer = useLazyEffectRef<EffectComposer | null>(() => {
    const composer = new EffectComposer(
      renderer.current!,
      new WebGLRenderTarget(getCanvasWidth(), getCanvasHeight()),
    );

    const renderPass = new RenderPass(scene.current!, camera.current!);

    // const savePassTarget = new WebGLRenderTarget(
    //   getCanvasWidth(),
    //   getCanvasHeight(),
    // );
    // const savePass = new SavePass(savePassTarget);
    // const blendPass = new ShaderPass(BlendShader, 'tDiffuse1');
    // blendPass.uniforms['tDiffuse2']!.value = savePass.renderTarget.texture;
    // blendPass.uniforms['mixRatio']!.value = 0.2;
    const gammaCorrectionPass = new ShaderPass(GammaCorrectionShader);
    // const outputPass = new ShaderPass(CopyShader);
    const fxaaPass = new ShaderPass(FXAAShader);
    fxaaPass.material.uniforms['resolution']!.value.x = 1 / getCanvasWidth();
    fxaaPass.material.uniforms['resolution']!.value.y = 1 / getCanvasHeight();

    composer.addPass(renderPass);
    composer.addPass(gammaCorrectionPass);
    // composer.addPass(blendPass);
    // composer.addPass(savePass);
    // composer.addPass(outputPass);
    composer.addPass(fxaaPass);
    return composer;
  });

  const recalculateMeshXY = useCallback(
    (mesh: Mesh, relativeCoords: { x: number; y: number }) => {
      const { topLeft, bottomRight } = corners.current!;
      mesh.position.x =
        relativeCoords.x * (bottomRight.x - topLeft.x) + topLeft.x;
      mesh.position.y =
        relativeCoords.y * (bottomRight.y - topLeft.y) + topLeft.y;
    },
    [corners],
  );

  const handleResize = useMemo(
    () =>
      debounce(() => {
        camera.current!.aspect = getCanvasAspectRatio();
        camera.current!.updateProjectionMatrix();
        renderer.current!.setSize(getCanvasWidth(), getCanvasHeight());
        composer.current!.setSize(getCanvasWidth(), getCanvasHeight());
        frustum.current = getFrustumFromCamera();
        corners.current = getCornersFromCamera();

        for (const { mesh, relativeCoords } of stars.current) {
          recalculateMeshXY(mesh, relativeCoords);
        }
      }, 200),
    [
      camera,
      composer,
      corners,
      frustum,
      getCornersFromCamera,
      getFrustumFromCamera,
      recalculateMeshXY,
      renderer,
    ],
  );

  useEffect(() => {
    handleResize();
    window.addEventListener('resize', handleResize);
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [handleResize]);

  useEffect(() => {
    for (const { mesh } of stars.current) {
      mesh.scale.z = speed;
    }
  }, [speed]);

  useEffect(() => {
    if (stars.current.length > numStars) {
      for (let i = numStars; i < stars.current.length; i++) {
        scene.current!.remove(stars.current[i]!.mesh);
      }
      stars.current.length = numStars;
    } else if (stars.current.length < numStars) {
      for (let i = stars.current.length; i < numStars; i++) {
        const mesh = new Mesh(STAR_GEOMETRY, STAR_MATERIAL);
        const relativeCoords = {
          x: Math.random(),
          y: Math.random(),
        };
        recalculateMeshXY(mesh, relativeCoords);
        mesh.scale.z = speed;
        mesh.position.z = -1 * Z_LIMIT * Math.random();
        stars.current.push({
          mesh,
          relativeCoords,
        });
        scene.current!.add(mesh);
      }
    }
  }, [numStars, recalculateMeshXY, scene, speed]);

  useEffect(() => {
    const animateMotion = (_timestamp: number) => {
      for (const { mesh, relativeCoords } of stars.current) {
        mesh.position.z += speed;

        if (
          mesh.position.z > 0 ||
          !frustum.current!.containsPoint(mesh.position)
        ) {
          relativeCoords.x = Math.random();
          relativeCoords.y = Math.random();
          recalculateMeshXY(mesh, relativeCoords);
          mesh.position.z = -1 * Z_LIMIT;
        }
      }
      composer.current!.render();

      animationFrame.current = window.requestAnimationFrame(animateMotion);
    };

    animateMotion(0);

    return () => {
      if (animationFrame.current != 0) {
        window.cancelAnimationFrame(animationFrame.current);
      }
    };
  }, [
    camera,
    composer,
    frustum,
    numStars,
    recalculateMeshXY,
    renderer,
    scene,
    speed,
  ]);

  return (
    <div className={style.background}>
      <canvas className={style.canvas} ref={canvas} />;
    </div>
  );
};
