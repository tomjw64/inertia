import { GraphEdge, GraphNode } from 'inertia-core';
import { BaseEvent } from 'three';

export type WorkerMessage =
  | StartMessage
  | MakeProxyMessage
  | ResizeMessage
  | EventMessage
  | InitGraphMessage;

export type StartMessage = {
  type: 'start';
  payload: {
    canvas: OffscreenCanvas;
    inputElementId: string;
    canvasWidth: number;
    canvasHeight: number;
    canvasId: string;
  };
};

export type InitGraphMessage = {
  type: 'initGraph';
  payload: {
    nodes: GraphNode[];
    edges: GraphEdge[];
  };
};

export type MakeProxyMessage = {
  type: 'makeProxy';
  payload: { id: string };
};

export type ResizeMessage = {
  type: 'resize';
  payload: { width: number; height: number };
};

export type ProxyEvent = BaseEvent & Record<string, unknown>;

export type EventMessage = {
  type: 'event';
  payload: {
    id: string;
    event: ProxyEvent;
  };
};
