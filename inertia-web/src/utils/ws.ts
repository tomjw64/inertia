import { FromClientMessage, ToClientMessage } from 'inertia-core';

const BACKEND_HOST = process.env.BACKEND_HOST;
const WS_CONNECTION_URL = `ws://${BACKEND_HOST}/ws`;

export class RoomWebSocket {
  private inner: WebSocket;

  constructor() {
    this.inner = new WebSocket(WS_CONNECTION_URL);
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
