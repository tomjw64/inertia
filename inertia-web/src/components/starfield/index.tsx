import { useRef, useEffect } from 'preact/hooks';
import style from './style.module.scss';
import { debounce } from 'lodash';

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
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const workerRef = useRef(
    new Worker(new URL('./starfield.worker.ts', import.meta.url)),
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
  }, []);

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
  }, []);

  useEffect(() => {
    const worker = workerRef.current;
    worker.postMessage({ numStars, speed });
  }, [numStars, speed]);

  return (
    <div className={style.background}>
      <canvas className={style.canvas} ref={canvasRef} />
    </div>
  );
};
