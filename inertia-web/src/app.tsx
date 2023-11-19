import { Router, Route } from 'preact-router';
import { Home } from './routes/home';
import { Sandbox } from './routes/sandbox';
import { Room } from './routes/room';
import { StarfieldSandbox } from './routes/starfield-sandbox';

const NotFound = () => {
  return <span>{'Not Found'}</span>;
};

const App = () => (
  <main style={{ minHeight: '100vh' }}>
    <Router>
      <Route path="/" component={Home} />
      <Route path="/room/:roomId" component={Room} />
      <Route path="/sandbox" component={Sandbox} />
      <Route path="/starfield-sandbox" component={StarfieldSandbox} />
      <Route default component={NotFound} />
    </Router>
  </main>
);

export default App;
