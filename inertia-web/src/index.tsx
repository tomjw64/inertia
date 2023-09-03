import App from './app';
import { render } from 'preact';

const anchor = document.body.querySelector('#app');
if (anchor) {
  render(<App />, anchor);
} else {
  document.body.textContent = 'Could not render.';
}
