import { useRef, useEffect } from 'preact/hooks';
import style from './style.module.scss';
import { debounce, range, groupBy } from 'lodash';

// Heavily inspired by: https://codepen.io/ksenia-k/pen/gOPboQg

type Star = {
  x: number;
  y: number;
  z: number;
};

const getColorString = (r: number, g: number, b: number) =>
  `#${((r << 16) | (g << 8) | b).toString(16)}`;

const getColorComponent = (fgColor: number, bgColor: number, alpha: number) =>
  Math.floor(fgColor * alpha + bgColor * (1 - alpha));

const getStarOpacityColor = (alpha: number) => {
  return getColorString(
    getColorComponent(STAR_R, BG_R, alpha),
    getColorComponent(STAR_G, BG_G, alpha),
    getColorComponent(STAR_B, BG_B, alpha),
  );
};

const Z_LIMIT = 1000;
const FULL_BRIGHTNESS_RADIUS = 4;

const BG_R = 0x37;
const BG_G = 0x3b;
const BG_B = 0x55;

const STAR_R = 0xff;
const STAR_G = 0xff;
const STAR_B = 0xff;

const OPACITY_LEVELS = 10;
const STAR_COLOR_BY_OPACITY = range(1, OPACITY_LEVELS + 1)
  .map((val) => val / OPACITY_LEVELS)
  .map(getStarOpacityColor);

const getContext = (canvas: HTMLCanvasElement) => canvas.getContext('2d')!;

const resizeCanvas = (canvas: HTMLCanvasElement) => {
  const pixelRatio = window.devicePixelRatio;
  canvas.height = Math.floor(window.innerHeight * pixelRatio);
  canvas.width = Math.floor(window.innerWidth * pixelRatio);
};

const clearCanvas = (canvas: HTMLCanvasElement) => {
  getContext(canvas).clearRect(0, 0, canvas.width, canvas.height);
};

const genStar = (): Star => {
  return {
    x: Math.random(),
    y: Math.random(),
    z: Math.random() * Z_LIMIT,
  };
};

const resetStar = (star: Star) => {
  star.z = 0;
  star.x = Math.random();
  star.y = Math.random();
};

const moveStar = (star: Star, speed: number) => {
  star.z = star.z + speed;
  if (star.z > Z_LIMIT) {
    resetStar(star);
  }
};

const getStarAsNStepsAgo = (star: Star, speed: number, n: number): Star => {
  return { x: star.x, y: star.y, z: Math.max(0, star.z - speed * n) };
};

const getStarInfo = (
  canvas: HTMLCanvasElement,
  star: Star,
): {
  x: number;
  y: number;
  radius: number;
  opacity: number;
} => {
  const magnification = Z_LIMIT / (Z_LIMIT - star.z);

  const centerX = canvas.width / 2;
  const centerY = canvas.height / 2;
  const x = (star.x * canvas.width - centerX) * magnification + centerX;
  const y = (star.y * canvas.height - centerY) * magnification + centerY;

  const radius = magnification;
  const opacity = Math.min(1, (radius - 1) / (FULL_BRIGHTNESS_RADIUS - 1));

  return {
    x,
    y,
    radius,
    opacity,
  };
};

const getStarInfoNStepsAgo = (
  canvas: HTMLCanvasElement,
  star: Star,
  speed: number,
  n: number,
) => {
  const {
    x: lastX,
    y: lastY,
    radius: lastRadius,
  } = getStarInfo(canvas, getStarAsNStepsAgo(star, speed, n));
  return {
    lastX,
    lastY,
    lastRadius,
  };
};

const starToPathAndOpacity = (
  canvas: HTMLCanvasElement,
  star: Star,
  speed: number,
  blurNSteps: number,
): [Path2D, number] | null => {
  const { x, y, radius, opacity } = getStarInfo(canvas, star);
  const { lastX, lastY, lastRadius } = getStarInfoNStepsAgo(
    canvas,
    star,
    speed,
    blurNSteps,
  );

  const deltaY = y - lastY;
  const deltaX = x - lastX;
  const angle = Math.atan2(deltaY, deltaX);
  const arcStart = angle + Math.PI / 2;
  const arcEnd = angle - Math.PI / 2;

  if (lastX < 0 || lastY < 0 || lastX > canvas.width || lastY > canvas.height) {
    resetStar(star);
    return null;
  }

  const path = new Path2D();
  path.arc(lastX, lastY, lastRadius, arcStart, arcEnd);
  path.arc(x, y, radius, arcEnd, arcStart);
  path.closePath();
  return [path, opacity];
};

type StarfieldProps = {
  numStars: number;
  speed: number;
};

export const Starfield = ({ numStars, speed }: StarfieldProps) => {
  const canvas = useRef<HTMLCanvasElement>(null);
  const stars = useRef<Star[]>([]);
  const animationFrame = useRef<number>(0);

  if (stars.current.length > numStars) {
    stars.current.length = numStars;
  } else if (stars.current.length < numStars) {
    for (let i = stars.current.length; i < numStars; i++) {
      stars.current.push(genStar());
    }
  }

  useEffect(() => {
    const setup = () => {
      resizeCanvas(canvas.current!);
    };

    setup();

    const debouncedSetup = debounce(setup, 200);
    window.addEventListener('resize', debouncedSetup);
    return () => {
      window.removeEventListener('resize', debouncedSetup);
    };
  }, []);

  useEffect(() => {
    const animateMotion = (_timestamp: number) => {
      clearCanvas(canvas.current!);

      if (numStars == 0) {
        return;
      }

      const starPaths: [Path2D, number][] = stars.current
        .map((star) => starToPathAndOpacity(canvas.current!, star, speed, 1))
        .filter((path) => path != null);

      const pathsByOpacityColor = groupBy(starPaths, ([_path, opacity]) => {
        const opacityIndex = Math.min(
          Math.round(opacity * STAR_COLOR_BY_OPACITY.length),
          STAR_COLOR_BY_OPACITY.length - 1,
        );
        return STAR_COLOR_BY_OPACITY[opacityIndex];
      });

      const context = getContext(canvas.current!);
      Object.entries(pathsByOpacityColor).forEach(([opacityColor, paths]) => {
        context.fillStyle = opacityColor;
        for (const [path, _] of paths) {
          context.fill(path);
        }
      });

      for (const star of stars.current) {
        moveStar(star, speed);
      }

      animationFrame.current = window.requestAnimationFrame(animateMotion);
    };

    animateMotion(0);

    return () => {
      if (animationFrame.current != 0) {
        window.cancelAnimationFrame(animationFrame.current);
      }
    };
  }, [numStars, speed]);
  return (
    <div className={style.background}>
      <canvas className={style.canvas} ref={canvas} />;
    </div>
  );
};
