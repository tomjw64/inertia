import { ComponentChildren } from 'preact';
import style from './style.module.scss';

export const Foreground = ({ children }: { children?: ComponentChildren }) => {
  return <div className={style.foreground}>{children}</div>;
};
