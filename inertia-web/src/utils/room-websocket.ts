import { FromClientMessage, ToClientMessage } from 'inertia-core';
import ReconnectingWebSocket from 'reconnecting-websocket';
import { WS_URL } from './backend';

export class RoomWebSocket {
  private inner: ReconnectingWebSocket;

  constructor() {
    this.inner = new ReconnectingWebSocket(WS_URL, [], {
      maxReconnectionDelay: 2000,
      minReconnectionDelay: 1000,
      reconnectionDelayGrowFactor: 1.1,
      connectionTimeout: 3000,
    });
  }

  onOpen(handler: () => void) {
    this.inner.addEventListener('open', handler);
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
