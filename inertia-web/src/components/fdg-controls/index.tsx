import { JSX } from 'preact';
import { StateSetter } from '../../utils/types';
import { FlexCenter } from '../flex-center';
import style from './style.module.scss';

type FdgControlsProps = {
  numNodes: number;
  setNumNodes: StateSetter<number>;
  numEdges: number;
  setNumEdges: StateSetter<number>;
};

export const FdgControls = ({
  numNodes,
  setNumNodes,
  numEdges,
  setNumEdges,
}: FdgControlsProps) => {
  const handleNumNodesChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setNumNodes(parseInt(e.currentTarget.value));
  };

  const handleNumEdgesChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setNumEdges(parseInt(e.currentTarget.value));
  };

  return (
    <div className={style.controls}>
      <FlexCenter>
        <label>Number of Nodes</label>
        <input
          type="range"
          min="0"
          max="1024"
          value={numNodes}
          step="1"
          onChange={handleNumNodesChange}
        />
        <div>[{numNodes}]</div>
      </FlexCenter>
      <FlexCenter>
        <label>Number of Edges</label>
        <input
          type="range"
          min="0"
          max="4096"
          value={numEdges}
          step="1"
          onChange={handleNumEdgesChange}
        />
        <div>[{numEdges}]</div>
      </FlexCenter>
    </div>
  );
};
