import { isEqual } from 'lodash';
import { useRef } from 'preact/hooks';

export const useStaticRefWhenEqual = <T>(value: T) => {
  const ref = useRef<T>();
  if (!isEqual(value, ref.current)) {
    ref.current = value;
  }
  return ref.current;
};
