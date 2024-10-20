import { useCallback, useState } from 'preact/hooks';

export const useRandom = ({ start, end }: { start: number; end: number }) => {
  const getRandom = useCallback(
    () => start + Math.floor(Math.random() * (end - start)),
    [end, start],
  );
  const [random, setRandom] = useState(getRandom());
  const reset = useCallback(() => {
    setRandom(getRandom());
  }, [getRandom]);
  return { random, reset };
};
