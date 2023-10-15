import { Signal, useSignal } from '@preact/signals';
import { useEffect } from 'preact/hooks';

export const useCountdown = (
  initialTimeLeftMillis: number,
  granularity: number
): Signal<number> => {
  const stopTime = Date.now() + initialTimeLeftMillis;
  const timeLeft = useSignal(0);

  useEffect(() => {
    const interval = setInterval(() => {
      timeLeft.value = Math.max(0, stopTime - Date.now());
    }, granularity);

    return () => {
      clearInterval(interval);
    };
  }, [initialTimeLeftMillis]);

  return timeLeft;
};
