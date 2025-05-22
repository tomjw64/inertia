import { useState } from 'preact/hooks';
import { Fdg } from '../../components/fdg';
import { FdgControls } from '../../components/fdg-controls';

export const FdgSandbox = () => {
  const [numNodes, setNumNodes] = useState<number>(100);
  const [numEdges, setNumEdges] = useState<number>(100);

  return (
    <>
      <Fdg numNodes={numNodes} numEdges={numEdges} />
      <FdgControls
        numNodes={numNodes}
        setNumNodes={setNumNodes}
        numEdges={numEdges}
        setNumEdges={setNumEdges}
      />
    </>
  );
};
