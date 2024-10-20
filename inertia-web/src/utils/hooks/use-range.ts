import { useCallback, useState } from 'preact/hooks';

export const useRange = (initialMin: number, initialMax: number) => {
  const [min, setInnerMin] = useState(initialMin);
  const [max, setInnerMax] = useState(initialMax);

  const setMin = useCallback((value: number) => {
    setInnerMin(value);
    setInnerMax((lastMax) => Math.max(value, lastMax));
  }, []);

  const setMax = useCallback((value: number) => {
    setInnerMin((lastMin) => Math.min(value, lastMin));
    setInnerMax(value);
  }, []);

  return { min, setMin, max, setMax };
};
