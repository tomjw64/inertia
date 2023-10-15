import { RoundStart as RoundStartState } from 'inertia-core';
import { Countdown } from '../countdown';

export const RoundStart = ({
  state,
  initialCountdownTimeLeft,
}: {
  state: RoundStartState;
  initialCountdownTimeLeft: number;
}) => {
  return (
    <div>
      <span>{JSON.stringify({ state })}</span>
      <Countdown
        initialCountdownTimeLeft={initialCountdownTimeLeft}
        granularity={10}
      />
    </div>
  );
};
