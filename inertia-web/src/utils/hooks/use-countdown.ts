import { useComputed, useSignal } from '@preact/signals';
import { useStopwatch } from './use-stopwatch';
import { useCallback } from 'preact/hooks';

export const useCountdown = ({
  timeMillis,
  paused,
}: {
  timeMillis: number;
  paused?: boolean;
}) => {
  const { reset: resetStopwatch, timeMillis: timePastMillis } = useStopwatch({
    paused,
  });
  const initialTimeLeftMillis = useSignal(timeMillis);

  const reset = useCallback(
    (timeMillis: number) => {
      resetStopwatch();
      initialTimeLeftMillis.value = timeMillis;
    },
    [resetStopwatch, initialTimeLeftMillis],
  );

  return {
    timeLeftMillis: useComputed(() =>
      Math.max(0, initialTimeLeftMillis.value - timePastMillis.value),
    ),
    reset,
  };
};
