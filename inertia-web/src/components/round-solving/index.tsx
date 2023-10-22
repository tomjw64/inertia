import { RoundSolving as RoundSolvingState } from 'inertia-core';
import { CountdownPanel } from '../countdown-panel';

export const RoundSolving = ({
  state,
  initialCountdownTimeLeft,
}: {
  state: RoundSolvingState;
  initialCountdownTimeLeft: number;
}) => {
  return (
    <div>
      <span>{JSON.stringify({ state })}</span>
      <CountdownPanel initialCountdownTimeLeft={initialCountdownTimeLeft} />
    </div>
  );
};
