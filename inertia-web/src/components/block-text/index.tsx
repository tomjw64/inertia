import { ComponentChildren } from 'preact';
import style from './style.module.scss';

export const BlockText = ({ children }: { children: ComponentChildren }) => (
  <p className={style.blockText}>{children}</p>
);
