import style from './style.module.scss';
import { useState } from 'preact/hooks';
import { Starfield } from '../../components/starfield';
import { StarfieldControls } from '../../components/starfield-controls';

export const StarfieldSandbox = () => {
  const [numStars, setNumStars] = useState<number>(1000);
  const [starSpeed, setStarSpeed] = useState<number>(4);
  const [useGpu, setUseGpu] = useState<boolean>(true);

  return (
    <>
      <Starfield
        numStars={numStars}
        speed={starSpeed}
        useHardwareAcceleration={useGpu}
      />
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
