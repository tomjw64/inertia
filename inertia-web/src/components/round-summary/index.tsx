import { RoundSummary as RoundSummaryState } from 'inertia-core';

export const RoundSummary = ({
  state,
  onStartGame,
}: {
  state: RoundSummaryState;
  onStartGame: () => void;
}) => {
  return (
    <div>
      <span>{JSON.stringify({ state })}</span>
      <button onClick={onStartGame}>Start Game</button>
    </div>
  );
};
