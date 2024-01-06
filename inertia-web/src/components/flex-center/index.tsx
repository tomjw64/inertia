import { ComponentChildren } from 'preact';
import style from './style.module.scss';

export const FlexCenter = ({
  children,
  column,
  wrap,
  justify,
  expand,
}: {
  wrap?: boolean;
  column?: boolean;
  expand?: boolean;
  justify?: string;
  children?: ComponentChildren;
}) => {
  const classes = [style.wrapper];
  if (column) {
    classes.push(style.column);
  }
  if (expand) {
    classes.push(style.expand);
  }
  classes.push(wrap ? style.wrap : style.nowrap);
  return (
    <div className={classes.join(' ')} style={{ justifyContent: justify }}>
      {children}
    </div>
  );
};
