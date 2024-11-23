import { useSignal } from '@preact/signals';
import { useCallback, useEffect, useRef } from 'preact/hooks';

export const useStopwatch = ({ paused }: { paused?: boolean } = {}) => {
  const timeMillis = useSignal(0);
  const startTime = useRef(Date.now());
  const timeAcc = useRef(0);

  const reset = useCallback(() => {
    timeMillis.value = 0;
    timeAcc.current = 0;
    startTime.current = Date.now();
  }, [timeMillis]);

  useEffect(() => {
    if (paused) {
      timeAcc.current += Date.now() - startTime.current;
    } else {
      startTime.current = Date.now();
    }
  }, [paused]);

  useEffect(() => {
    if (paused) {
      return;
    }

    const interval = setInterval(() => {
      timeMillis.value = Math.max(
        0,
        timeAcc.current + Date.now() - startTime.current,
      );
    }, 10);

    return () => {
      clearInterval(interval);
    };
  }, [paused, startTime, timeMillis]);

  return { timeMillis, reset };
};
