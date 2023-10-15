import { useComputed } from '@preact/signals';
import { useCountdown } from '../../utils/use-countdown';

export const Countdown = ({
  initialCountdownTimeLeft,
  granularity,
}: {
  initialCountdownTimeLeft: number;
  granularity: number;
}) => {
  const timeLeftMillis = useCountdown(initialCountdownTimeLeft, granularity);
  const timeLeftFormatted = useComputed(() =>
    (timeLeftMillis.value / 1000).toFixed(2)
  );

  if (timeLeftMillis == null) {
    return <></>;
  }

  return <div>{timeLeftFormatted}</div>;
};
