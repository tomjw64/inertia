import style from './style.module.scss';
import { useState } from 'preact/hooks';
import { Starfield as GpuStarfield } from '../../components/gpu-starfield';
import { Starfield as CanvasStarfield } from '../../components/starfield';
import { StarfieldControls } from '../../components/starfield-controls';

export const StarfieldSandbox = () => {
  const [numStars, setNumStars] = useState<number>(1000);
  const [starSpeed, setStarSpeed] = useState<number>(4);
  const [useGpu, setUseGpu] = useState<boolean>(true);

  return (
    <>
      {useGpu ? (
        <GpuStarfield numStars={numStars} speed={starSpeed} />
      ) : (
        <CanvasStarfield numStars={numStars} speed={starSpeed} />
      )}
      <div class={style.room}>
        <StarfieldControls
          numStars={numStars}
          setNumStars={setNumStars}
          starSpeed={starSpeed}
          setStarSpeed={setStarSpeed}
          useGpu={useGpu}
          setUseGpu={setUseGpu}
        />
      </div>
    </>
  );
};
