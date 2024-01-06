const BACKEND_PORT =
  process.env.BACKEND_PORT === 'auto' ? '' : `:${process.env.BACKEND_PORT}`;
const WS_RELATIVE_PROTOCOL =
  window.location.protocol === 'https:' ? 'wss:' : 'ws:';

export const WS_URL = `${WS_RELATIVE_PROTOCOL}//${window.location.hostname}${BACKEND_PORT}/ws`;

export const getBackendUrl = (relative: string) => {
  const path = relative.startsWith('/') ? relative : `/${relative}`;
  return `${window.location.protocol}//${window.location.hostname}${BACKEND_PORT}${path}`;
};
