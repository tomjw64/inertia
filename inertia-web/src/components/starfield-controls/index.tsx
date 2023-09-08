import { StateUpdater } from 'preact/hooks';
import { JSX } from 'preact/jsx-runtime';

type StarfieldControlsProps = {
  numStars: number;
  setNumStars: StateUpdater<number>;
  starSpeed: number;
  setStarSpeed: StateUpdater<number>;
};

export const StarfieldControls = ({
  numStars,
  setNumStars,
  starSpeed,
  setStarSpeed,
}: StarfieldControlsProps) => {
  const handleStarSpeedChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setStarSpeed(parseInt(e.currentTarget.value));
  };

  const handleNumStarsChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setNumStars(parseInt(e.currentTarget.value));
  };

  const toggleHyperdrive = () => {};

  return (
    <div>
      <div>
        <label>Star Speed</label>
        <input
          type="range"
          min="1"
          max="20"
          value={starSpeed}
          step="1"
          onChange={handleStarSpeedChange}
        />
      </div>
      <div>
        <label>Number of Stars</label>
        <input
          type="range"
          min="0"
          max="2000"
          value={numStars}
          step="100"
          onChange={handleNumStarsChange}
        />
      </div>
      <div>
        <button onClick={toggleHyperdrive}>Toggle Hyperdrive</button>
      </div>
    </div>
  );
};
