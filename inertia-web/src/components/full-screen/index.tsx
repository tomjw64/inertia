import { ComponentChildren } from 'preact';
import style from './style.module.scss';

export const FullScreen = ({ children }: { children: ComponentChildren }) => {
  return <div className={style.fullScreen}>{children}</div>;
};
