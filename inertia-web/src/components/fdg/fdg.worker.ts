import {
  Color,
  Mesh,
  PerspectiveCamera,
  Scene,
  SphereGeometry,
  Vector3,
  WebGLRenderer,
  EventDispatcher,
  BaseEvent,
  AmbientLight,
  DirectionalLight,
  ArrowHelper,
  LineBasicMaterial,
  SpriteMaterial,
  Sprite,
  CanvasTexture,
  MeshToonMaterial,
} from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls';
import {
  EventMessage,
  InitGraphMessage,
  MakeProxyMessage,
  ProxyEvent,
  ResizeMessage,
  StartMessage,
  WorkerMessage,
} from './worker-message';
import { GraphEdge, GraphNode } from 'inertia-core';

const NODE_MATERIALS: Record<string, MeshToonMaterial> = {};
const getNodeMaterial = (color: string): MeshToonMaterial => {
  if (!NODE_MATERIALS[color]) {
    NODE_MATERIALS[color] = new MeshToonMaterial({ color });
  }
  return NODE_MATERIALS[color];
};
const NODE_RADIUS = 5;
const NODE_GEOMETRY = new SphereGeometry(NODE_RADIUS, 32, 32);

type DomEventLike = {
  preventDefault?: () => void;
  stopPropagation?: () => void;
};

class ElementProxyReceiver extends EventDispatcher<Record<string, unknown>> {
  ownerDocument = this;
  document = {};
  style = {};
  width = 0;
  height = 0;
  left = 0;
  top = 0;

  constructor() {
    super();
  }

  get clientWidth() {
    return this.width;
  }
  get clientHeight() {
    return this.height;
  }
  getBoundingClientRect() {
    return {
      left: this.left,
      top: this.top,
      width: this.width,
      height: this.height,
      right: this.left + this.width,
      bottom: this.top + this.height,
    };
  }

  handleEvent(event: BaseEvent & DomEventLike) {
    event.preventDefault = () => {};
    event.stopPropagation = () => {};
    this.dispatchEvent(event);
  }

  getRootNode() {
    return this;
  }

  setPointerCapture() {}
  releasePointerCapture() {}

  focus() {}
}

class ProxyManager {
  private targets: { [key: string]: ElementProxyReceiver } = {};
  constructor() {}

  makeProxy(id: string) {
    const proxy = new ElementProxyReceiver();
    this.targets[id] = proxy;
  }

  getProxy(id: string) {
    return this.targets[id];
  }

  handleEvent(id: string, event: ProxyEvent) {
    this.targets[id]?.handleEvent(event);
  }
}

class ForceGraphAnimation {
  private camera: PerspectiveCamera;
  private scene: Scene;
  private renderer: WebGLRenderer;
  private controls: OrbitControls;

  constructor(
    private canvas: OffscreenCanvas,
    private inputElement: ElementProxyReceiver,
    private canvasWidth: number,
    private canvasHeight: number,
  ) {
    this.camera = this.createCamera();
    this.scene = this.createScene();
    this.renderer = this.createRenderer();
    this.controls = this.createOrbitControls();
  }

  handleResize(width: number, height: number) {
    this.canvasWidth = width;
    this.canvasHeight = height;

    this.inputElement.width = width;
    this.inputElement.height = height;

    this.camera.aspect = this.getCanvasAspectRatio();
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height, false);
  }

  animate() {
    const animateMotion = (_timestamp: number) => {
      this.controls.update();
      this.render();
      requestAnimationFrame(animateMotion);
    };
    animateMotion(0);
  }

  initGraph(nodes: GraphNode[], edges: GraphEdge[]) {
    this.scene.children = this.scene.children.filter((child) => {
      const isMeshOrLine =
        child.type === 'Mesh' || child.type === 'ArrowHelper';
      if (isMeshOrLine) {
        this.scene.remove(child);
      }
      return !isMeshOrLine;
    });

    nodes.forEach((node, index) => {
      const nodeMesh = new Mesh(NODE_GEOMETRY, getNodeMaterial(node.color));
      nodeMesh.position.set(node.position.x, node.position.y, node.position.z);
      this.scene.add(nodeMesh);

      if (index < 256) {
        const canvasWidth = 200;
        const canvasHeight = 100;
        const canvas = new OffscreenCanvas(canvasWidth, canvasHeight);
        const context = canvas.getContext('2d');

        if (context) {
          const fontSize = 32;
          context.font = `Bold ${fontSize}px Arial`;
          context.fillStyle = 'rgba(255, 255, 255, 0.8)';
          context.textAlign = 'center';
          context.textBaseline = 'middle';

          const text = index.toString();
          context.fillText(text, canvasWidth / 2, canvasHeight / 2);
        }

        const texture = new CanvasTexture(canvas);
        const spriteMaterial = new SpriteMaterial({
          map: texture,
        });
        const sprite = new Sprite(spriteMaterial);
        sprite.onBeforeRender = function (renderer, scene, camera) {
          const objectPosition = new Vector3();
          nodeMesh.getWorldPosition(objectPosition);

          const cameraPosition = new Vector3();
          camera.getWorldPosition(cameraPosition);

          const directionToCamera = new Vector3();
          directionToCamera
            .subVectors(cameraPosition, objectPosition)
            .normalize();

          const basePosition = new Vector3(
            node.position.x,
            node.position.y,
            node.position.z,
          );

          basePosition.addScaledVector(directionToCamera, NODE_RADIUS + 0.5);

          sprite.position.copy(basePosition);
          sprite.updateMatrixWorld();
        };

        const spriteScale = 10;
        sprite.scale.set(
          spriteScale * (canvasWidth / canvasHeight),
          spriteScale,
          1,
        );

        sprite.position.set(
          node.position.x,
          node.position.y,
          node.position.z + NODE_RADIUS * 0.5,
        );
        this.scene.add(sprite);
      }
    });

    edges.forEach((edge) => {
      const sourceNode = nodes[edge.source];
      const targetNode = nodes[edge.target];

      if (sourceNode && targetNode) {
        const start = new Vector3(
          sourceNode.position.x,
          sourceNode.position.y,
          sourceNode.position.z,
        );
        const end = new Vector3(
          targetNode.position.x,
          targetNode.position.y,
          targetNode.position.z,
        );
        const direction = new Vector3().subVectors(end, start).normalize();
        const length = start.distanceTo(end);

        const arrowStart = start
          .clone()
          .add(direction.clone().multiplyScalar(NODE_RADIUS));
        const arrowLength = length - NODE_RADIUS * 2 - 0.8;

        if (arrowLength > 0) {
          const arrow = new ArrowHelper(
            direction,
            arrowStart,
            arrowLength,
            0x888888,
            5,
            3,
          );
          arrow.line.material = new LineBasicMaterial({ color: 0x888888 });
          arrow.cone.material = new MeshToonMaterial({
            color: 0x888888,
          });
          this.scene.add(arrow);
        }
      }
    });
  }

  private createCamera() {
    return new PerspectiveCamera(50, this.getCanvasAspectRatio(), 50, 10000);
  }

  private getCanvasAspectRatio() {
    return this.canvasWidth / this.canvasHeight;
  }

  private createScene() {
    const scene = new Scene();
    scene.background = new Color(0x373b55);
    scene.add(new AmbientLight(0x606060));
    const directionalLight = new DirectionalLight(0xffffff, 1);
    directionalLight.position.set(1, 1, 1).normalize();
    scene.add(directionalLight);
    return scene;
  }

  private createRenderer() {
    return new WebGLRenderer({
      canvas: this.canvas,
      antialias: true,
    });
  }

  private createOrbitControls() {
    const controls = new OrbitControls(
      this.camera,
      this.inputElement as unknown as HTMLElement,
    );
    controls.target = new Vector3(0, 0, 0);
    this.camera.position.set(0, 0, 500);
    controls.update();
    return controls;
  }

  private render() {
    this.renderer.render(this.scene, this.camera);
  }
}

let fdg: ForceGraphAnimation | undefined;
const proxyManager = new ProxyManager();
let proxy: ElementProxyReceiver | undefined;

const start = (data: StartMessage['payload']) => {
  const { canvas, inputElementId, canvasWidth, canvasHeight } = data;
  proxy = proxyManager.getProxy(inputElementId)!;
  (self as unknown as { document: Document }).document = {} as Document; // HACK!
  fdg = new ForceGraphAnimation(canvas, proxy, canvasWidth, canvasHeight);
  fdg.animate();
};

const handleInitGraph = (data: InitGraphMessage['payload']) => {
  fdg?.initGraph(data.nodes, data.edges);
};

const makeProxy = (data: MakeProxyMessage['payload']) => {
  proxyManager.makeProxy(data.id);
};

const handleResize = (data: ResizeMessage['payload']) => {
  fdg?.handleResize(data.width, data.height);
};

const handleEvent = (data: EventMessage['payload']) => {
  proxyManager.handleEvent(data.id, data.event);
};

type Handlers = {
  [T in WorkerMessage['type']]: (
    payload: Extract<WorkerMessage, { type: T }>['payload'],
  ) => void;
};

const handlers: Handlers = {
  start,
  initGraph: handleInitGraph,
  makeProxy,
  resize: handleResize,
  event: handleEvent,
};

addEventListener('message', ({ data }: { data: WorkerMessage }) => {
  const { type, payload } = data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  handlers[type]?.(payload as any);
});
