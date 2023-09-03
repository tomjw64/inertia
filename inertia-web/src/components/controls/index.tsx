import {
  ActorSquares,
  SolutionStep,
  WalledBoard,
  WalledBoardPosition,
} from 'inertia-core';
import { StateUpdater, useState } from 'preact/hooks';
import { JSX } from 'preact/jsx-runtime';

type ControlsProps = {
  setWalledBoard: StateUpdater<WalledBoard>;
  setInitialActorSquares: StateUpdater<ActorSquares>;
  initialActorSquares: ActorSquares;
  setActorSquares: StateUpdater<ActorSquares>;
  simulateMoveSequence: (sequence: SolutionStep[]) => Promise<void>;
  numStars: number;
  setNumStars: StateUpdater<number>;
  starSpeed: number;
  setStarSpeed: StateUpdater<number>;
};

export const Controls = ({
  setWalledBoard,
  setInitialActorSquares,
  initialActorSquares,
  setActorSquares,
  simulateMoveSequence,
  numStars,
  setNumStars,
  starSpeed,
  setStarSpeed,
}: ControlsProps) => {
  const [simulate, setSimulate] = useState('');

  const handleSubmitSimulate = async () => {
    simulateMoveSequence(JSON.parse(simulate));
    setSimulate('');
  };

  const handleSimulateChange = async (
    e: JSX.TargetedEvent<HTMLInputElement>
  ) => {
    setSimulate(e.currentTarget.value);
  };

  const handleNewBoard = async () => {
    const response = await fetch('http://0.0.0.0:8000/board/random');
    const result = (await response.json()) as WalledBoardPosition;
    setInitialActorSquares(result['actor_squares']);
    setWalledBoard(result['walled_board']);
  };

  const handleResetBoard = async () => {
    setActorSquares(initialActorSquares);
  };

  const handleStarSpeedChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setStarSpeed(parseInt(e.currentTarget.value));
  };

  const handleNumStarsChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setNumStars(parseInt(e.currentTarget.value));
  };

  return (
    <div>
      <button onClick={handleNewBoard}>New Board</button>
      <input value={simulate} onChange={handleSimulateChange} />
      <button onClick={handleSubmitSimulate}>Simulate</button>
      <button onClick={handleResetBoard}>Reset Board</button>
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
    </div>
  );
};
