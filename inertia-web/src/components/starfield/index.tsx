import { Starfield as CanvasStarfield } from './canvas-starfield';
import { Starfield as ThreeJsStarfield } from './threejs-starfield';

export const Starfield = ({
  numStars,
  speed,
  useHardwareAcceleration = true,
}: {
  numStars: number;
  speed: number;
  useHardwareAcceleration?: boolean;
}) => {
  if (useHardwareAcceleration) {
    return <ThreeJsStarfield numStars={numStars} speed={speed} />;
  }
  return <CanvasStarfield numStars={numStars} speed={speed} />;
};
