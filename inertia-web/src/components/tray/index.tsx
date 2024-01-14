import { ComponentChildren } from 'preact';
import style from './style.module.scss';
import classnames from 'classnames';

export const Tray = ({
  expanded,
  children,
  transformOrigin,
}: {
  expanded: boolean;
  transformOrigin?: string;
  children?: ComponentChildren;
}) => {
  return (
    <div className={classnames(style.wrapper, { [style.expanded]: expanded })}>
      <div className={style.inner} style={{ transformOrigin }}>
        {children}
      </div>
    </div>
  );
};
