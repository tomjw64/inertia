import { FromClientMessage, ToClientMessage } from 'inertia-core';

const WS_CONNECTION_URL = 'ws://127.0.0.1:8001/ws';

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
