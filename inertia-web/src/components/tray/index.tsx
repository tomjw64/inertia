import { ComponentChildren } from 'preact';
import style from './style.module.scss';
import classnames from 'classnames';

export const Tray = ({
  expanded,
  children,
  transformOrigin,
  inset = false,
}: {
  expanded: boolean;
  transformOrigin?: string;
  children?: ComponentChildren;
  inset?: boolean;
}) => {
  return (
    <div className={classnames(style.wrapper, { [style.expanded]: expanded })}>
      <div
        className={classnames(style.inner, { [style.inset]: inset })}
        style={{ transformOrigin }}
      >
        {children}
      </div>
    </div>
  );
};
