import { Router, Route } from 'preact-router';
import { Home } from './routes/home';
import { Sandbox } from './routes/sandbox';
import { Room } from './routes/room';

const NotFound = () => {
  return <span>{'Not Found'}</span>;
};

const App = () => (
  <main>
    <Router>
      <Route path="/" component={Home} />
      <Route path="/room/:roomId" component={Room} />
      <Route path="/sandbox" component={Sandbox} />
      <Route default component={NotFound} />
    </Router>
  </main>
);

export default App;
