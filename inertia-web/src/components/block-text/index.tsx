import { ComponentChildren } from 'preact';
import style from './style.module.scss';
import classNames from 'classnames';

export const BlockText = ({
  muted = false,
  children,
}: {
  muted?: boolean;
  children: ComponentChildren;
}) => (
  <p className={classNames(style.blockText, { [style.muted]: muted })}>
    {children}
  </p>
);
