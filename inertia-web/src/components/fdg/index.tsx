import { useRef, useEffect } from 'preact/hooks';
import style from './style.module.scss';
import { debounce } from 'lodash';
import { RenderWhen } from '../utils/RenderWhen';
import { useLazyRef } from '../../utils/hooks/use-lazy-ref';
import {
  EventMessage,
  InitGraphMessage,
  MakeProxyMessage,
  ProxyEvent,
  ResizeMessage,
  StartMessage,
} from './worker-message';
import { GraphEdge, GraphNode } from 'inertia-core';

const getCanvasWidth = () =>
  Math.floor(window.innerWidth * window.devicePixelRatio);

const getCanvasHeight = () =>
  Math.floor(window.innerHeight * window.devicePixelRatio);

type EventHandlers = Record<
  string,
  (event: Event, sendEvent: (data: ProxyEvent) => void) => void
>;

let nextProxyId = 0;
class ElementProxy {
  id: string;
  worker: Worker;

  constructor(
    element: HTMLCanvasElement,
    worker: Worker,
    eventHandlers: EventHandlers,
  ) {
    this.id = (nextProxyId++).toString();
    this.worker = worker;
    const sendEvent = (event: ProxyEvent) => {
      const message: EventMessage = {
        type: 'event',
        payload: {
          id: this.id,
          event: event,
        },
      };
      this.worker.postMessage(message);
    };

    const makeProxyMessage: MakeProxyMessage = {
      type: 'makeProxy',
      payload: {
        id: this.id,
      },
    };
    worker.postMessage(makeProxyMessage);
    for (const [eventName, handler] of Object.entries(eventHandlers)) {
      element.addEventListener(eventName, function (event) {
        handler(event, sendEvent);
      });
    }
  }
}

export const Fdg = ({
  nodes,
  edges,
}: {
  nodes: GraphNode[];
  edges: GraphEdge[];
}) => {
  return (
    <div className={style.background}>
      <RenderWhen when={nodes.length > 0}>
        <NonEmptyFdg nodes={nodes} edges={edges} />
      </RenderWhen>
    </div>
  );
};

export const NonEmptyFdg = ({
  nodes,
  edges,
}: {
  nodes: GraphNode[];
  edges: GraphEdge[];
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const workerRef = useLazyRef(
    () => new Worker(new URL('./fdg.worker.ts', import.meta.url)),
  );

  useEffect(() => {
    const canvas = canvasRef.current!;
    const canvasOffscreen = canvasRef.current!.transferControlToOffscreen();
    const worker = workerRef.current;

    const mouseEventHandler = makeSendPropertiesHandler<PointerEvent>([
      'ctrlKey',
      'metaKey',
      'shiftKey',
      'button',
      'pointerType',
      'clientX',
      'clientY',
      'pointerId',
      'pageX',
      'pageY',
    ]);
    const wheelEventHandlerImpl = makeSendPropertiesHandler<WheelEvent>([
      'deltaX',
      'deltaY',
    ]);
    const keydownEventHandler = makeSendPropertiesHandler<KeyboardEvent>([
      'ctrlKey',
      'metaKey',
      'shiftKey',
      'keyCode',
    ]);

    function wheelEventHandler(
      event: WheelEvent,
      sendFn: (data: ProxyEvent) => void,
    ) {
      event.preventDefault();
      wheelEventHandlerImpl(event, sendFn);
    }

    function preventDefaultHandler(event: Event) {
      event.preventDefault();
    }

    function copyProperties<T extends Event>(
      src: T,
      properties: (keyof T & string)[],
      dst: Record<string, unknown>,
    ) {
      for (const name of properties) {
        dst[name] = src[name];
      }
    }

    function makeSendPropertiesHandler<T extends Event>(
      properties: (keyof T & string)[],
    ) {
      return (event: T, sendFn: (data: ProxyEvent) => void) => {
        const data = { type: event.type };
        copyProperties(event, properties, data);
        sendFn(data);
      };
    }

    function touchEventHandler(
      event: TouchEvent,
      sendFn: (data: {
        type: string;
        touches: {
          pageX: number;
          pageY: number;
          clientX: number;
          clientY: number;
        }[];
      }) => void,
    ) {
      event.preventDefault();
      const touches: {
        pageX: number;
        pageY: number;
        clientX: number;
        clientY: number;
      }[] = [];
      const data = { type: event.type, touches };
      for (let i = 0; i < event.touches.length; ++i) {
        const touch = event.touches[i]!;
        touches.push({
          pageX: touch.pageX,
          pageY: touch.pageY,
          clientX: touch.clientX,
          clientY: touch.clientY,
        });
      }
      sendFn(data);
    }

    const orbitKeys: Record<string, boolean> = {
      ArrowLeft: true,
      ArrowUp: true,
      ArrowRight: true,
      ArrowDown: true,
    };
    function filteredKeydownEventHandler(
      event: KeyboardEvent,
      sendFn: (data: ProxyEvent) => void,
    ) {
      const { key } = event;
      if (orbitKeys[key]) {
        event.preventDefault();
        keydownEventHandler(event, sendFn);
      }
    }

    const eventHandlers = {
      contextmenu: preventDefaultHandler,
      mousedown: mouseEventHandler,
      mousemove: mouseEventHandler,
      mouseup: mouseEventHandler,
      pointerdown: mouseEventHandler,
      pointermove: mouseEventHandler,
      pointerup: mouseEventHandler,
      touchstart: touchEventHandler,
      touchmove: touchEventHandler,
      touchend: touchEventHandler,
      wheel: wheelEventHandler,
      keydown: filteredKeydownEventHandler,
    } as unknown as EventHandlers;
    const proxy = new ElementProxy(canvas, worker, eventHandlers);

    const startMessage: StartMessage = {
      type: 'start',
      payload: {
        canvas: canvasOffscreen,
        inputElementId: proxy.id,
        canvasWidth: getCanvasWidth(),
        canvasHeight: getCanvasHeight(),
        canvasId: proxy.id,
      },
    };
    worker.postMessage(startMessage, [canvasOffscreen]);

    return () => {
      worker.terminate();
    };
  }, [workerRef]);

  useEffect(() => {
    const handleResize = debounce(() => {
      const resizeMessage: ResizeMessage = {
        type: 'resize',
        payload: {
          width: getCanvasWidth(),
          height: getCanvasHeight(),
        },
      };
      workerRef.current.postMessage(resizeMessage);
    }, 200);
    setTimeout(() => {
      handleResize();
    }, 0);
    window.addEventListener('resize', handleResize);
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [workerRef]);

  useEffect(() => {
    const worker = workerRef.current;
    const initGraphMessage: InitGraphMessage = {
      type: 'initGraph',
      payload: {
        nodes,
        edges,
      },
    };
    worker.postMessage(initGraphMessage);
  }, [nodes, edges, workerRef]);

  return <canvas className={style.canvas} ref={canvasRef} />;
};
