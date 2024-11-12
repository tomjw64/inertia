import { ComponentChildren } from 'preact';
import style from './style.module.scss';
import classNames from 'classnames';

export const BlockText = ({
  muted = false,
  bold = false,
  children,
}: {
  muted?: boolean;
  bold?: boolean;
  children: ComponentChildren;
}) => (
  <p
    className={classNames(style.blockText, {
      [style.muted]: muted,
      [style.bold]: bold,
    })}
  >
    {children}
  </p>
);
