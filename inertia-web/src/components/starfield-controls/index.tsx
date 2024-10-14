import { JSX } from 'preact/jsx-runtime';
import { StateSetter } from '../../utils/types';
import { FlexCenter } from '../flex-center';

type StarfieldControlsProps = {
  numStars: number;
  setNumStars: StateSetter<number>;
  starSpeed: number;
  setStarSpeed: StateSetter<number>;
  useGpu: boolean;
  setUseGpu: StateSetter<boolean>;
};

export const StarfieldControls = ({
  numStars,
  setNumStars,
  starSpeed,
  setStarSpeed,
  useGpu,
  setUseGpu,
}: StarfieldControlsProps) => {
  const handleStarSpeedChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setStarSpeed(parseInt(e.currentTarget.value));
  };

  const handleNumStarsChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setNumStars(parseInt(e.currentTarget.value));
  };

  const handleToggleUseGpu = () => {
    setUseGpu(!useGpu);
  };

  return (
    <div>
      <FlexCenter>
        <label>Star Speed</label>
        <input
          type="range"
          min="0"
          max="20"
          value={starSpeed}
          step="1"
          onChange={handleStarSpeedChange}
        />
        <div>[{starSpeed}]</div>
      </FlexCenter>
      <FlexCenter>
        <label>Number of Stars</label>
        <input
          type="range"
          min="0"
          max="10000"
          value={numStars}
          step="100"
          onChange={handleNumStarsChange}
        />
        <div>[{numStars}]</div>
      </FlexCenter>
      <FlexCenter>
        <button onClick={handleToggleUseGpu}>
          {useGpu ? 'Use canvas' : 'Use gpu'}
        </button>
      </FlexCenter>
    </div>
  );
};
