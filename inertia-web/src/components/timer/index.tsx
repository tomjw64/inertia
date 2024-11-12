import { Signal, useComputed } from '@preact/signals';
import style from './style.module.scss';
import { FlexCenter } from '../flex-center';
import { mmssccFormat } from '../../utils/time';
import classNames from 'classnames';

export const Timer = ({ time, paused }: { time: Signal; paused?: boolean }) => {
  const timeFormatted = useComputed(() => mmssccFormat(time.value));

  return (
    <div className={classNames(style.content, { [style.paused]: paused })}>
      <FlexCenter>
        <span>{timeFormatted}</span>
      </FlexCenter>
    </div>
  );
};
