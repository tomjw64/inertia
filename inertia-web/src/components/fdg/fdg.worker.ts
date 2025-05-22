import {
  Color,
  Mesh,
  MeshStandardMaterial,
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

const NODE_MATERIAL = new MeshStandardMaterial({ color: 0x44bbff });
const NODE_RADIUS = 4;
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

  initGraph(numNodes: number, numEdges: number) {
    this.scene.children = this.scene.children.filter((child) => {
      const isMeshOrLine =
        child.type === 'Mesh' || child.type === 'ArrowHelper';
      if (isMeshOrLine) {
        this.scene.remove(child);
      }
      return !isMeshOrLine;
    });

    const nodes: { id: number; x: number; y: number; z: number }[] = [];
    for (let i = 0; i < numNodes; i++) {
      nodes.push({
        id: i,
        x: Math.random() * 1000 - 500,
        y: Math.random() * 1000 - 500,
        z: Math.random() * 1000 - 500,
      });
    }

    const edges = [];
    for (let i = 0; i < numEdges; i++) {
      edges.push({
        source: Math.floor(Math.random() * numNodes),
        target: Math.floor(Math.random() * numNodes),
      });
    }

    nodes.forEach((nodeData) => {
      const nodeMesh = new Mesh(NODE_GEOMETRY, NODE_MATERIAL);
      nodeMesh.position.set(nodeData.x, nodeData.y, nodeData.z);
      this.scene.add(nodeMesh);
    });

    edges.forEach((edgeData) => {
      const sourceNode = nodes.find((n) => n.id === edgeData.source);
      const targetNode = nodes.find((n) => n.id === edgeData.target);

      if (sourceNode && targetNode) {
        const start = new Vector3(sourceNode.x, sourceNode.y, sourceNode.z);
        const end = new Vector3(targetNode.x, targetNode.y, targetNode.z);
        const direction = new Vector3().subVectors(end, start).normalize();
        const length = start.distanceTo(end);

        const arrowStart = start
          .clone()
          .add(direction.clone().multiplyScalar(NODE_RADIUS));
        const arrowLength = length - NODE_RADIUS * 2 - 1;

        if (arrowLength > 0) {
          const arrow = new ArrowHelper(
            direction,
            arrowStart,
            arrowLength,
            0x888888,
            6,
            4,
          );
          arrow.line.material = new LineBasicMaterial({ color: 0xaaaaaa });
          arrow.cone.material = new LineBasicMaterial({ color: 0xaaaaaa });
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
  fdg?.initGraph(data.numNodes, data.numEdges);
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
