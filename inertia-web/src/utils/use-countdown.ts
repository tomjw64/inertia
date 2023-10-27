import { Signal, useSignal } from '@preact/signals';
import { useEffect } from 'preact/hooks';

export const useCountdown = (
  timeLeftMillis: number,
  paused?: boolean
): Signal<number> => {
  const stopTime = Date.now() + timeLeftMillis;
  const timeLeft = useSignal(timeLeftMillis);

  useEffect(() => {
    if (paused) {
      return;
    }

    const interval = setInterval(() => {
      timeLeft.value = Math.max(0, stopTime - Date.now());
    }, 10);

    return () => {
      clearInterval(interval);
    };
  }, [timeLeftMillis, paused, timeLeft, stopTime]);

  return timeLeft;
};
