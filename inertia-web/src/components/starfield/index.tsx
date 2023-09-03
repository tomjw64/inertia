import { useRef, useEffect } from 'preact/hooks';
import style from './style.module.scss';
import debounce from 'lodash/debounce';

// Heavily inspired by: https://codepen.io/ksenia-k/pen/gOPboQg

const Z_LIMIT = 1000;
const FULL_BRIGHTNESS_RADIUS = 4;

type Star = {
  x: number;
  y: number;
  z: number;
};

const genStar = (): Star => {
  return {
    x: Math.random(),
    y: Math.random(),
    z: Math.random() * Z_LIMIT,
  };
};

const moveStar = (star: Star, speed: number) => {
  star.z = (star.z + speed) % Z_LIMIT;
};

const asLastPosition = (star: Star, speed: number): Star => {
  return { ...star, z: Math.max(0, star.z - speed) };
};

const getStarArcInfo = (
  canvas: HTMLCanvasElement,
  star: Star
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
  const opacity = 1 * (radius / FULL_BRIGHTNESS_RADIUS);

  return {
    x,
    y,
    radius,
    opacity,
  };
};

const showStar = (
  canvas: HTMLCanvasElement,
  context: CanvasRenderingContext2D,
  star: Star,
  speed: number
) => {
  const { x, y, radius, opacity } = getStarArcInfo(canvas, star);
  const {
    x: lastX,
    y: lastY,
    radius: lastRadius,
  } = getStarArcInfo(canvas, asLastPosition(star, speed));
  const deltaY = y - lastY;
  const deltaX = x - lastX;
  const angle = Math.atan2(deltaY, deltaX);
  const arcStart = angle + Math.PI / 2;
  const arcEnd = angle - Math.PI / 2;

  const color = `rgba(255, 255, 255, ${opacity})`;

  context.beginPath();
  // Looks cool, but kills performance
  // context.shadowColor = color;
  // context.shadowBlur = speed;
  context.fillStyle = color;
  context.arc(lastX, lastY, lastRadius, arcStart, arcEnd);
  context.arc(x, y, radius, arcEnd, arcStart);
  context.closePath();
  context.fill();
};

type StarfieldProps = {
  numStars: number;
  speed: number;
};

export const Starfield = ({ numStars, speed }: StarfieldProps) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const stars = useRef<Star[]>([]);
  const animationFrame = useRef<number>(0);
  const _lastAnimationFrameTimestamp = useRef<DOMHighResTimeStamp | null>(null);

  if (stars.current.length > numStars) {
    stars.current.length = numStars;
  } else if (stars.current.length < numStars) {
    for (let i = stars.current.length; i < numStars; i++) {
      stars.current.push(genStar());
    }
  }

  useEffect(() => {
    const animate = (_timestamp: DOMHighResTimeStamp) => {
      const canvas = canvasRef.current;
      if (canvas == null) {
        return;
      }

      const context = canvas.getContext('2d');
      if (context == null) {
        return;
      }

      context.fillStyle = '#373b55';
      context.fillRect(0, 0, canvas.width, canvas.height);
      for (const star of stars.current) {
        showStar(canvas, context, star, speed);
        moveStar(star, speed);
      }
      animationFrame.current = window.requestAnimationFrame(animate);
    };
    animate(0);
    return () => {
      if (animationFrame.current != 0) {
        window.cancelAnimationFrame(animationFrame.current);
      }
    };
  }, [speed]);

  useEffect(() => {
    const setup = () => {
      const canvas = canvasRef.current;
      if (canvas == null) {
        return;
      }

      const context = canvas.getContext('2d');
      if (context == null) {
        return;
      }

      canvas.height = window.innerHeight;
      canvas.width = window.innerWidth;
    };

    setup();

    const debouncedSetup = debounce(setup, 200);
    window.addEventListener('resize', debouncedSetup);
    return () => {
      window.removeEventListener('resize', debouncedSetup);
    };
  }, []);

  return <canvas className={style.background} ref={canvasRef} />;
};
