import style from './style.module.scss';
import { useState } from 'preact/hooks';
import { Starfield } from '../../components/starfield';
import { StarfieldControls } from '../../components/starfield-controls';

export const StarfieldSandbox = () => {
  const [numStars, setNumStars] = useState<number>(1000);
  const [starSpeed, setStarSpeed] = useState<number>(4);

  return (
    <>
      <Starfield numStars={numStars} speed={starSpeed} />
      <div class={style.room}>
        <StarfieldControls
          numStars={numStars}
          setNumStars={setNumStars}
          starSpeed={starSpeed}
          setStarSpeed={setStarSpeed}
        />
      </div>
    </>
  );
};
