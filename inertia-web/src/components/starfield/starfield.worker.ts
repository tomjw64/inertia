import {
  Color,
  CylinderGeometry,
  FogExp2,
  Frustum,
  Matrix4,
  Mesh,
  MeshBasicMaterial,
  PerspectiveCamera,
  Scene,
  Vector3,
  WebGLRenderer,
} from 'three';

const getColorString = (r: number, g: number, b: number) =>
  `#${((r << 16) | (g << 8) | b).toString(16)}`;

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

class StarfieldAnimation {
  private stars: {
    mesh: Mesh;
    relativeCoords: { x: number; y: number };
  }[];
  private speed: number;

  private camera: PerspectiveCamera;
  private frustum: Frustum;
  private corners: {
    topLeft: Vector3;
    bottomRight: Vector3;
  };
  private scene: Scene;
  private renderer: WebGLRenderer;

  constructor(
    private canvas: HTMLCanvasElement,
    private canvasWidth: number,
    private canvasHeight: number,
  ) {
    this.stars = [];
    this.speed = 0;

    this.camera = this.createCamera();
    this.frustum = this.getFrustumFromCurrentCamera();
    this.corners = this.getCornersFromCurrentCamera();
    this.scene = this.createScene();
    this.renderer = this.createRenderer();
  }

  private getCanvasAspectRatio() {
    return this.canvasWidth / this.canvasHeight;
  }

  private createCamera() {
    return new PerspectiveCamera(50, this.getCanvasAspectRatio(), 50, Z_LIMIT);
  }

  private createScene() {
    const scene = new Scene();
    scene.background = new Color(0x373b55);
    scene.fog = new FogExp2(0x373b55, 0.0025);
    return scene;
  }

  private createRenderer() {
    return new WebGLRenderer({
      canvas: this.canvas,
      antialias: true,
    });
  }

  private render() {
    this.renderer.render(this.scene, this.camera);
  }

  private getFrustumFromCurrentCamera() {
    const frustum = new Frustum();
    frustum.setFromProjectionMatrix(
      new Matrix4().multiplyMatrices(
        this.camera.projectionMatrix,
        this.camera.matrixWorldInverse,
      ),
    );
    return frustum;
  }

  private getCornersFromCurrentCamera() {
    return {
      topLeft: new Vector3().set(-1, 1, 1).unproject(this.camera),
      bottomRight: new Vector3().set(1, -1, 1).unproject(this.camera),
    };
  }

  private recalculateMeshXY(
    mesh: Mesh,
    relativeCoords: { x: number; y: number },
  ) {
    const { topLeft, bottomRight } = this.corners;
    mesh.position.x =
      relativeCoords.x * (bottomRight.x - topLeft.x) + topLeft.x;
    mesh.position.y =
      relativeCoords.y * (bottomRight.y - topLeft.y) + topLeft.y;
  }

  handleResize(width: number, height: number) {
    this.canvasWidth = width;
    this.canvasHeight = height;

    this.camera.aspect = this.getCanvasAspectRatio();
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height, false);
    this.frustum = this.getFrustumFromCurrentCamera();
    this.corners = this.getCornersFromCurrentCamera();

    for (const { mesh, relativeCoords } of this.stars) {
      this.recalculateMeshXY(mesh, relativeCoords);
    }
  }

  setSpeed(speed: number) {
    this.speed = speed;
    for (const { mesh } of this.stars) {
      mesh.scale.z = speed;
    }
  }

  setNumStars(numStars: number) {
    if (this.stars.length > numStars) {
      for (let i = numStars; i < this.stars.length; i++) {
        this.scene.remove(this.stars[i]!.mesh);
      }
      this.stars.length = numStars;
    } else if (this.stars.length < numStars) {
      for (let i = this.stars.length; i < numStars; i++) {
        const mesh = new Mesh(STAR_GEOMETRY, STAR_MATERIAL);
        const relativeCoords = {
          x: Math.random(),
          y: Math.random(),
        };
        this.recalculateMeshXY(mesh, relativeCoords);
        mesh.scale.z = this.speed;
        mesh.position.z = -1 * Z_LIMIT * Math.random();
        this.stars.push({
          mesh,
          relativeCoords,
        });
        this.scene.add(mesh);
      }
    }
  }

  animate() {
    const animateMotion = (_timestamp: number) => {
      for (const { mesh, relativeCoords } of this.stars) {
        mesh.position.z += this.speed;

        if (mesh.position.z > 0 || !this.frustum.containsPoint(mesh.position)) {
          relativeCoords.x = Math.random();
          relativeCoords.y = Math.random();
          this.recalculateMeshXY(mesh, relativeCoords);
          mesh.position.z = -1 * Z_LIMIT;
        }
      }
      this.render();

      requestAnimationFrame(animateMotion);
    };

    animateMotion(0);
  }
}

type MessageData = {
  canvas?: HTMLCanvasElement;
  canvasWidth: number;
  canvasHeight: number;
  numStars?: number;
  speed?: number;
};

let starfield: StarfieldAnimation | undefined;
addEventListener('message', (msg) => {
  const { canvas, canvasWidth, canvasHeight, numStars, speed } =
    msg.data as MessageData;

  if (canvas && canvasWidth != null && canvasHeight != null) {
    starfield = new StarfieldAnimation(canvas, canvasWidth, canvasHeight);
    starfield.animate();
  }
  if (canvasWidth != null && canvasHeight != null) {
    starfield?.handleResize(canvasWidth, canvasHeight);
  }
  if (speed != null) {
    starfield?.setSpeed(speed);
  }
  if (numStars != null) {
    starfield?.setNumStars(numStars);
  }
});
