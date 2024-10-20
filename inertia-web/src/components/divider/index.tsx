import classNames from 'classnames';
import style from './style.module.scss';

export const Divider = ({
  text,
  narrow,
}: {
  text?: string;
  narrow?: boolean;
}) => {
  return (
    <div
      className={classNames(style.divider, {
        [style.withText]: !!text,
        [style.narrow]: narrow,
      })}
    >
      {text}
    </div>
  );
};
