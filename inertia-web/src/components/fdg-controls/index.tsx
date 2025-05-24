import { JSX } from 'preact';
import { StateSetter } from '../../utils/types';
import { FlexCenter } from '../flex-center';
import style from './style.module.scss';

type FdgControlsProps = {
  t: number;
  setT: StateSetter<number>;
};

export const FdgControls = ({ t, setT }: FdgControlsProps) => {
  const handleTChange = (e: JSX.TargetedEvent<HTMLInputElement>) => {
    setT(Number(e.currentTarget.value));
  };

  const max = 200;

  return (
    <div className={style.controls}>
      <FlexCenter>
        <label>T</label>
        <input
          type="range"
          min={0}
          max={max}
          value={t}
          step={1}
          onChange={handleTChange}
        />
        <div>
          [{t}/{max}]
        </div>
      </FlexCenter>
    </div>
  );
};
