import { ComponentChildren } from 'preact';
import style from './style.module.scss';

type CommonProps = {
  wrap?: boolean;
  expand?: boolean;
  children?: ComponentChildren;
};
type AlignmentProps =
  | {
      justify?: string;
      column?: undefined;
      align?: undefined;
    }
  | {
      justify?: string;
      column: false;
      align?: undefined;
    }
  | {
      justify?: undefined;
      column: true;
      align?: string;
    };
type Props = CommonProps & AlignmentProps;

export const FlexCenter = ({
  children,
  column,
  wrap,
  justify,
  expand,
  align,
}: Props) => {
  const classes = [style.wrapper];
  if (column) {
    classes.push(style.column);
  }
  if (expand) {
    classes.push(style.expand);
  }
  classes.push(wrap ? style.wrap : style.nowrap);
  return (
    <div
      className={classes.join(' ')}
      style={{ justifyContent: justify, alignItems: align }}
    >
      {children}
    </div>
  );
};
