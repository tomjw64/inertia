import { set_panic_hook } from 'inertia-core';
import { ErrorBoundary, LocationProvider, Route, Router } from 'preact-iso';
import { FlexCenter } from './components/flex-center';
import { Starfield } from './components/starfield';
import { ThemedPanel } from './components/themed-panel';
import { BoardEditor } from './routes/board-editor';
import { BoardExplorer } from './routes/board-explorer';
import { Home } from './routes/home';
import { Room } from './routes/room';
import { StarfieldSandbox } from './routes/starfield-sandbox';

set_panic_hook();

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
    <LocationProvider>
      <ErrorBoundary>
        <Router>
          <Route path="/" component={Home} />
          <Route path="/room/:roomId" component={Room} />
          <Route path="/explore" component={BoardExplorer} />
          <Route path="/edit" component={BoardEditor} />
          <Route path="/daily" component={() => <></>} />
          <Route path="/puzzle" component={() => <></>} />
          <Route path="/starfield-sandbox" component={StarfieldSandbox} />
          <Route default component={NotFound} />
        </Router>
      </ErrorBoundary>
    </LocationProvider>
  </main>
);

export default App;
