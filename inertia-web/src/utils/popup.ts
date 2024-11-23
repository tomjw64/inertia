import { animate, AnimationControls } from 'motion';
import { sleep } from './sleep';
import { useMemo } from 'preact/hooks';
import { Nullable } from './types';

const createPopup = () => {
  let currentAnimationId = 0;
  let animation: AnimationControls | undefined;
  return async (element: Nullable<HTMLElement>) => {
    if (!element) {
      return;
    }
    const animationId = ++currentAnimationId;
    const isCanceled = () => {
      return currentAnimationId !== animationId;
    };
    if (animation) {
      animation.cancel();
    }
    element.style.display = 'block';
    animation = animate(
      element,
      { opacity: [0, 1], scale: [0.1, 1] },
      {
        duration: 0.2,
        easing: 'ease-in-out',
      },
    );
    await animation.finished;
    await sleep(2000);
    if (!isCanceled()) {
      animation = animate(
        element,
        { opacity: [1, 0], scale: [1, 0.1] },
        {
          duration: 0.2,
          easing: 'ease-in-out',
        },
      );
    }
    await animation.finished;
    if (!isCanceled()) {
      element.style.display = 'none';
    }
  };
};

export const usePopup = () => {
  return useMemo(() => createPopup(), []);
};
