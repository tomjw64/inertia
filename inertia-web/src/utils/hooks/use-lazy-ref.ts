import { MutableRef, useEffect, useRef } from 'preact/hooks';

export const useLazyRef = <T>(supplier: () => T) => {
  const ref = useRef<T>(null);
  if (ref.current == null) {
    ref.current = supplier();
  }
  return ref as MutableRef<T>;
};

export const useLazyEffectRef = <T>(supplier: () => T) => {
  const ref = useRef<T | null>(null);
  useEffect(() => {
    if (ref.current == null) {
      ref.current = supplier();
    }
    // Supplier should only ever be called on the first render
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return ref;
};
