import { useCallback, useRef, useState } from 'preact/hooks';

// TODO: Is it better to write a (tested?) throttled queue utility and just have
// useThrottledQueue delegate to that?
export const useThrottledQueue = <T extends NonNullable<unknown>>({
  throttleMs,
  onData,
}: {
  throttleMs: number;
  onData: (data: T) => void;
}) => {
  const nextEventTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const queue = useRef<T[]>([]);

  const [isProcessing, setIsProcessing] = useState<boolean>(false);

  const throttleMsRef = useRef<number>(throttleMs);
  // TODO: Consumer cannot be changed :(
  const onDataRef = useRef<(data: T) => void>(onData);

  const processQueue = useCallback(() => {
    if (nextEventTimer.current) {
      return;
    }
    if (queue.current.length === 0) {
      return;
    }
    const sendDataAndWait = () => {
      const data = queue.current.shift();
      if (data == null) {
        nextEventTimer.current = null;
        setIsProcessing(false);
        return;
      }
      setIsProcessing(true);
      onDataRef.current(data);
      nextEventTimer.current = setTimeout(() => {
        sendDataAndWait();
      }, throttleMsRef.current);
    };
    sendDataAndWait();
  }, []);

  const setQueue = useCallback((update: T[]) => {
    queue.current = update;
  }, []);

  const clearQueue = useCallback(() => {
    if (nextEventTimer.current) {
      clearTimeout(nextEventTimer.current);
    }
    nextEventTimer.current = null;
    queue.current = [];
    setIsProcessing(false);
  }, []);

  return { clearQueue, processQueue, setQueue, isProcessing };
};
