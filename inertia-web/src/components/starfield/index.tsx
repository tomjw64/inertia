import { useRef, useEffect } from 'preact/hooks';
import style from './style.module.scss';
import { debounce } from 'lodash';
import { RenderWhen } from '../utils/RenderWhen';
import { useLazyRef } from '../../utils/hooks/use-lazy-ref';

const getCanvasWidth = () =>
  Math.floor(window.innerWidth * window.devicePixelRatio);

const getCanvasHeight = () =>
  Math.floor(window.innerHeight * window.devicePixelRatio);

export const Starfield = ({
  numStars,
  speed,
}: {
  numStars: number;
  speed: number;
}) => {
  return (
    <div className={style.background}>
      <RenderWhen when={numStars > 0}>
        <NonEmptyStarfield numStars={numStars} speed={speed} />
      </RenderWhen>
    </div>
  );
};

export const NonEmptyStarfield = ({
  numStars,
  speed,
}: {
  numStars: number;
  speed: number;
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const workerRef = useLazyRef(
    () => new Worker(new URL('./starfield.worker.ts', import.meta.url)),
  );

  useEffect(() => {
    const canvas = canvasRef.current!.transferControlToOffscreen();
    const worker = workerRef.current;
    worker.postMessage(
      {
        canvas,
        canvasWidth: getCanvasWidth(),
        canvasHeight: getCanvasHeight(),
      },
      [canvas],
    );

    return () => {
      worker.terminate();
    };
  }, [workerRef]);

  useEffect(() => {
    const handleResize = debounce(() => {
      workerRef.current.postMessage({
        canvasWidth: getCanvasWidth(),
        canvasHeight: getCanvasHeight(),
      });
    }, 200);
    window.addEventListener('resize', handleResize);
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [workerRef]);

  useEffect(() => {
    const worker = workerRef.current;
    worker.postMessage({ numStars, speed });
  }, [numStars, speed, workerRef]);

  return <canvas className={style.canvas} ref={canvasRef} />;
};
