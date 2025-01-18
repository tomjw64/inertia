import {
  Color,
  CylinderGeometry,
  FogExp2,
  Frustum,
  InstancedMesh,
  Matrix4,
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
const MAX_STAR_COUNT = 10_000;

class StarfieldAnimation {
  private scratchpadMatrix = new Matrix4();
  private scratchpadVector = new Vector3();

  private starMesh: InstancedMesh;
  private starRelativeCoords: {
    x: number;
    y: number;
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
    this.starMesh = new InstancedMesh(
      STAR_GEOMETRY,
      STAR_MATERIAL,
      MAX_STAR_COUNT,
    );
    this.starMesh.frustumCulled = false;
    this.starMesh.count = 0;
    this.starRelativeCoords = [];
    this.speed = 0;

    this.camera = this.createCamera();
    this.frustum = this.getFrustumFromCurrentCamera();
    this.corners = this.getCornersFromCurrentCamera();
    this.scene = this.createScene();
    this.renderer = this.createRenderer();

    this.scene.add(this.starMesh);
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

  private beginScratchpadForStarAt(index: number) {
    const matrix = this.scratchpadMatrix;
    const position = this.scratchpadVector;
    this.starMesh.getMatrixAt(index, matrix);
    position.setFromMatrixPosition(matrix);
  }

  private setScratchpadXyForStarAt(index: number) {
    const relativeCoords = this.starRelativeCoords[index]!;
    const { topLeft, bottomRight } = this.corners;
    this.scratchpadVector.setX(
      relativeCoords.x * (bottomRight.x - topLeft.x) + topLeft.x,
    );
    this.scratchpadVector.setY(
      relativeCoords.y * (bottomRight.y - topLeft.y) + topLeft.y,
    );
  }

  private scaleScratchPadZScaleToSpeed() {
    this.scratchpadMatrix.makeScale(1, 1, this.speed);
  }

  private commitScratchpadForStarAt(index: number) {
    this.scratchpadMatrix.setPosition(this.scratchpadVector);
    this.starMesh.setMatrixAt(index, this.scratchpadMatrix);
  }

  handleResize(width: number, height: number) {
    this.canvasWidth = width;
    this.canvasHeight = height;

    this.camera.aspect = this.getCanvasAspectRatio();
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height, false);
    this.frustum = this.getFrustumFromCurrentCamera();
    this.corners = this.getCornersFromCurrentCamera();

    for (let index = 0; index < this.starRelativeCoords.length; index++) {
      this.beginScratchpadForStarAt(index);
      this.setScratchpadXyForStarAt(index);
      this.commitScratchpadForStarAt(index);
    }
  }

  setSpeed(speed: number) {
    this.speed = speed;
    for (let index = 0; index < this.starRelativeCoords.length; index++) {
      this.beginScratchpadForStarAt(index);
      this.scaleScratchPadZScaleToSpeed();
      this.commitScratchpadForStarAt(index);
    }
  }

  setNumStars(numStars: number) {
    if (this.starRelativeCoords.length > numStars) {
      this.starMesh.count = numStars;
      this.starRelativeCoords.length = numStars;
    } else if (this.starRelativeCoords.length < numStars) {
      const resultingCount = Math.min(numStars, MAX_STAR_COUNT);
      this.starMesh.count = resultingCount;

      for (
        let index = this.starRelativeCoords.length;
        index < resultingCount;
        index++
      ) {
        const relativeCoords = {
          x: Math.random(),
          y: Math.random(),
        };
        this.starRelativeCoords.push(relativeCoords);
        this.beginScratchpadForStarAt(index);
        this.setScratchpadXyForStarAt(index);
        this.scratchpadVector.setZ(-1 * Z_LIMIT * Math.random());
        this.scaleScratchPadZScaleToSpeed();
        this.commitScratchpadForStarAt(index);
      }
      this.starMesh.instanceMatrix.needsUpdate = true;
    }
  }

  animate() {
    const animateMotion = (_timestamp: number) => {
      for (let index = 0; index < this.starRelativeCoords.length; index++) {
        this.beginScratchpadForStarAt(index);
        if (
          this.scratchpadVector.z > 0 ||
          !this.frustum.containsPoint(this.scratchpadVector)
        ) {
          this.starRelativeCoords[index]!.x = Math.random();
          this.starRelativeCoords[index]!.y = Math.random();

          this.setScratchpadXyForStarAt(index);
          this.scratchpadVector.setZ(-1 * Z_LIMIT);
        }
        this.scratchpadVector.setZ(this.scratchpadVector.z + this.speed);
        this.commitScratchpadForStarAt(index);
      }
      this.starMesh.instanceMatrix.needsUpdate = true;
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
