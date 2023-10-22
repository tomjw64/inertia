import { useComputed } from '@preact/signals';
import { useCountdown } from '../../utils/use-countdown';
import style from './style.module.scss';

export const CountdownPanel = ({
  initialCountdownTimeLeft,
}: {
  initialCountdownTimeLeft: number;
}) => {
  const timeLeftMillis = useCountdown(initialCountdownTimeLeft, 10);
  const timeLeftFormatted = useComputed(() =>
    (timeLeftMillis.value / 1000).toFixed(2)
  );

  if (timeLeftMillis == null) {
    return <></>;
  }

  return (
    <div className={style.countdownWrapper}>
      <div className={style.countdownContent}>{timeLeftFormatted}</div>
    </div>
  );
};
