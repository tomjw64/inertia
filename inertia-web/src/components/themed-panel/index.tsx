import style from './style.module.scss';
import { FlexCenter } from '../flex-center';
import { ComponentChildren } from 'preact';

export const ThemedPanel = ({ children }: { children?: ComponentChildren }) => {
  return (
    <div className={style.wrapper}>
      <FlexCenter>
        <div className={style.content}>{children}</div>
      </FlexCenter>
    </div>
  );
};
