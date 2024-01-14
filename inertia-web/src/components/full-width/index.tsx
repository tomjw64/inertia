import { ComponentChildren } from 'preact';
import style from './style.module.scss';

export const FullWidth = ({ children }: { children?: ComponentChildren }) => {
  return <div className={style.fullWidth}>{children}</div>;
};
