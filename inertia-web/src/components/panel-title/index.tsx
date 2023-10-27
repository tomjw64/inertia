import { ComponentChildren } from 'preact';
import style from './style.module.scss';

export const PanelTitle = ({ children }: { children?: ComponentChildren }) => {
  return <div className={style.title}>{children}</div>;
};
