import { animate } from 'motion';
import { Nullable } from '../utils/nullable';

export const shake = (element: Nullable<Element>) => {
  if (element == null) {
    return;
  }
  const animationOffset = [0, -1, 2, -4, 4, -4, 4, -4, 2, -1, 0];
  const animationOffsetAsTranslate = animationOffset.map(
    (offset) => `translateX(${offset}px)`
  );
  animate(element, { transform: animationOffsetAsTranslate }, { duration: 1 });
};
