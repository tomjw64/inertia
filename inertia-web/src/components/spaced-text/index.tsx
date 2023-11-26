import { ComponentChild } from 'preact';
import style from './style.module.scss';

export const BlockText = ({ children }: { children: ComponentChild }) => (
  <p className={style.blockText}>{children}</p>
);
