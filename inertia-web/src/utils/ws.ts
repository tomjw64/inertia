import { FromClientMessage, ToClientMessage } from 'inertia-core';

const WS_PORT = process.env.WS_PORT === 'auto' ? '' : `:${process.env.WS_PORT}`;
const WS_RELATIVE_PROTOCOL =
  window.location.protocol === 'https:' ? 'wss:' : 'ws:';
const WS_URL = `${WS_RELATIVE_PROTOCOL}//${window.location.hostname}${WS_PORT}/ws`;

console.log(WS_URL);

export class RoomWebSocket {
  private inner: WebSocket;

  constructor() {
    this.inner = new WebSocket(WS_URL);
  }

  onOpen(handler: () => void) {
    this.inner.addEventListener('open', (_event: Event) => {
      handler();
    });
  }

  onMessage(handler: (msg: ToClientMessage) => void) {
    this.inner.addEventListener('message', (event: MessageEvent<string>) => {
      const msg = JSON.parse(event.data) as ToClientMessage;
      handler(msg);
    });
  }

  send(msg: FromClientMessage) {
    this.inner.send(JSON.stringify(msg));
  }

  close() {
    this.inner.close();
  }
}
