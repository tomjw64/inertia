import { useRef, useEffect, useMemo } from 'preact/hooks';
import style from './style.module.scss';
import { debounce } from 'lodash';
import {
  Color,
  Fog,
  Frustum,
  Matrix4,
  Mesh,
  MeshBasicMaterial,
  PerspectiveCamera,
  Scene,
  SphereGeometry,
  Vector3,
  WebGLRenderer,
} from 'three';

const getColorString = (r: number, g: number, b: number) =>
  `#${((r << 16) | (g << 8) | b).toString(16)}`;

const getCanvasHeight = () =>
  Math.floor(window.innerHeight * window.devicePixelRatio);

const getCanvasWidth = () =>
  Math.floor(window.innerWidth * window.devicePixelRatio);

const getCanvasAspectRatio = () => getCanvasWidth() / getCanvasHeight();

const Z_LIMIT = 2000;

const STAR_R = 0xff;
const STAR_G = 0xff;
const STAR_B = 0xff;

const STAR_COLOR = getColorString(STAR_R, STAR_B, STAR_G);

const STAR_GEOMETRY = new SphereGeometry(1, 16, 16);
const STAR_MATERIAL = new MeshBasicMaterial({ color: STAR_COLOR });

export const Starfield = ({
  numStars,
  speed,
}: {
  numStars: number;
  speed: number;
}) => {
  // numStars = 0;
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const stars = useRef<
    {
      mesh: Mesh;
      relativeCoords: { x: number; y: number };
    }[]
  >([]);

  const camera = useRef<PerspectiveCamera | null>(null);
  const scene = useRef<Scene | null>(null);
  const renderer = useRef<WebGLRenderer | null>(null);

  const animationFrame = useRef<number>(0);
  const _lastAnimationFrameTimestamp = useRef<DOMHighResTimeStamp | null>(null);

  const matchCanvasToWindowSize = useMemo(
    () =>
      debounce(() => {
        camera.current!.aspect = getCanvasAspectRatio();
        camera.current!.updateProjectionMatrix();
        renderer.current!.setSize(getCanvasWidth(), getCanvasHeight());
      }),
    [],
  );

  useEffect(() => {
    if (camera.current) {
      return;
    }
    camera.current = new PerspectiveCamera(
      50,
      getCanvasAspectRatio(),
      50,
      Z_LIMIT,
    );
  }, []);

  useEffect(() => {
    if (scene.current) {
      return;
    }
    scene.current = new Scene();
    scene.current.background = new Color(0x373b55);
    scene.current.fog = new Fog(0x373b55, Z_LIMIT * 0.6, Z_LIMIT);
  }, []);

  useEffect(() => {
    if (renderer.current) {
      return;
    }
    renderer.current = new WebGLRenderer({
      canvas: canvasRef.current!,
      antialias: true,
    });
  }, []);

  useEffect(() => {
    matchCanvasToWindowSize();
    window.addEventListener('resize', matchCanvasToWindowSize);
    return () => {
      window.removeEventListener('resize', matchCanvasToWindowSize);
    };
  }, [matchCanvasToWindowSize]);

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
        const topLeft = new Vector3().set(-1, 1, 1).unproject(camera.current!);
        const bottomRight = new Vector3()
          .set(1, -1, 1)
          .unproject(camera.current!);
        mesh.position.x =
          relativeCoords.x * (bottomRight.x - topLeft.x) + topLeft.x;
        mesh.position.y =
          relativeCoords.y * (bottomRight.y - topLeft.y) + topLeft.y;
        mesh.position.z = -1 * Z_LIMIT * Math.random();
        stars.current.push({
          mesh,
          relativeCoords,
        });
        scene.current!.add(mesh);
      }
    }
  }, [numStars]);

  useEffect(() => {
    const animateMotion = (_timestamp: number) => {
      for (const { mesh, relativeCoords } of stars.current) {
        mesh.position.z += speed;
        const frustum = new Frustum();
        frustum.setFromProjectionMatrix(
          new Matrix4().multiplyMatrices(
            camera.current!.projectionMatrix,
            camera.current!.matrixWorldInverse,
          ),
        );
        if (mesh.position.z > 0 || !frustum.containsPoint(mesh.position)) {
          relativeCoords.x = Math.random();
          relativeCoords.y = Math.random();
          const topLeft = new Vector3()
            .set(-1, 1, 1)
            .unproject(camera.current!);
          const bottomRight = new Vector3()
            .set(1, -1, 1)
            .unproject(camera.current!);
          mesh.position.x =
            relativeCoords.x * (bottomRight.x - topLeft.x) + topLeft.x;
          mesh.position.y =
            relativeCoords.y * (bottomRight.y - topLeft.y) + topLeft.y;
          mesh.position.z = -1 * Z_LIMIT;
        }
      }
      renderer.current!.render(scene.current!, camera.current!);

      animationFrame.current = window.requestAnimationFrame(animateMotion);
    };

    animateMotion(0);

    return () => {
      if (animationFrame.current != 0) {
        window.cancelAnimationFrame(animationFrame.current);
      }
    };
  }, [numStars, speed]);

  return (
    <div className={style.background}>
      <canvas className={style.canvas} ref={canvasRef} />;
    </div>
  );
};
