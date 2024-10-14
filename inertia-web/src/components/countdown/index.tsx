import { useComputed } from '@preact/signals';
import style from './style.module.scss';
import { useCountdown } from '../../utils/hooks/use-countdown';
import { FlexCenter } from '../flex-center';

const pad = (num: number, length: number) =>
  num.toString().padStart(length, '0');

export const Countdown = ({
  timeLeft,
  paused,
}: {
  timeLeft: number;
  paused?: boolean;
}) => {
  const timeLeftMillis = useCountdown(timeLeft, paused);
  const timeLeftFormatted = useComputed(() => {
    const millisTotal = timeLeftMillis.value;
    const centisTotal = Math.floor(millisTotal / 10);
    const secondsTotal = Math.floor(millisTotal / 1000);
    const minutesTotal = Math.floor(secondsTotal / 60);

    const minutes = pad(minutesTotal % 60, 1);
    const seconds = pad(secondsTotal % 60, 2);
    const centis = pad(centisTotal % 100, 2);
    return `${minutes}:${seconds}.${centis}`;
  });

  const classNames = [style.content];
  if (paused) {
    classNames.push(style.paused);
  }

  return (
    <div className={classNames.join(' ')}>
      <FlexCenter>
        <span>{timeLeftFormatted}</span>
      </FlexCenter>
    </div>
  );
};
