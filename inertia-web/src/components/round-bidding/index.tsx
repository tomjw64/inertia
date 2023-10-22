import { RoundBidding as RoundBiddingState } from 'inertia-core';
import { CountdownPanel } from '../countdown-panel';

export const RoundBidding = ({
  state,
  initialCountdownTimeLeft,
}: {
  state: RoundBiddingState;
  initialCountdownTimeLeft: number;
}) => {
  return (
    <div>
      <span>{JSON.stringify({ state })}</span>
      <CountdownPanel initialCountdownTimeLeft={initialCountdownTimeLeft} />
    </div>
  );
};
