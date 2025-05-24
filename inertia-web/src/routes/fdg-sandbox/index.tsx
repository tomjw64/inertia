import { generate_force_graph_from_position } from 'inertia-core';
import { Fdg } from '../../components/fdg';
import { useInitialUrlPositions } from '../../utils/url-params';
import { useState } from 'preact/hooks';
import { FdgControls } from '../../components/fdg-controls';

export const FdgSandbox = () => {
  const [t, setT] = useState(0);

  const positions = useInitialUrlPositions();
  const position = positions[0]?.position;
  if (!position) {
    return <Fdg nodes={[]} edges={[]} />;
  }

  const graph = generate_force_graph_from_position(position, t);

  return (
    <>
      <Fdg nodes={graph.nodes} edges={graph.edges} />
      <FdgControls t={t} setT={setT} />
    </>
  );
};
