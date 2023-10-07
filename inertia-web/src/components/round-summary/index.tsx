import { RoundSummary as RoundSummaryState } from 'inertia-core';

export const RoundSummary = ({ data }: { data: RoundSummaryState }) => {
  return (
    <div>
      <span>{JSON.stringify({ data })}</span>
      <button>Start Game</button>
    </div>
  );
};
