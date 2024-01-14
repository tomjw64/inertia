import { Router, Route } from 'preact-router';
import { Home } from './routes/home';
import { Sandbox } from './routes/sandbox';
import { Room } from './routes/room';
import { StarfieldSandbox } from './routes/starfield-sandbox';
import { FlexCenter } from './components/flex-center';
import { ThemedPanel } from './components/themed-panel';
import { Starfield } from './components/starfield';
import { BoardExplorer } from './routes/board-explorer';

const NotFound = () => {
  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <div style={{ height: '100vh', width: '100vw' }}>
        <FlexCenter expand>
          <ThemedPanel>Nothing here.</ThemedPanel>
        </FlexCenter>
      </div>
    </>
  );
};

const App = () => (
  <main style={{ minHeight: '100svh' }}>
    <Router>
      <Route path="/" component={Home} />
      <Route path="/room/:roomId" component={Room} />
      <Route path="/explore" component={BoardExplorer} />
      <Route path="/sandbox" component={Sandbox} />
      <Route path="/starfield-sandbox" component={StarfieldSandbox} />
      <Route default component={NotFound} />
    </Router>
  </main>
);

export default App;
