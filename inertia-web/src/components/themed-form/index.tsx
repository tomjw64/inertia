import { ComponentChildren, JSX } from 'preact';
import style from './style.module.scss';

export const ThemedFormLine = ({
  children,
}: {
  children?: ComponentChildren;
}) => {
  return <div className={style.formLine}>{children}</div>;
};

export const ThemedButton = ({
  children,
  onClick,
  disabled = false,
}: {
  children?: ComponentChildren;
  onClick?: JSX.MouseEventHandler<HTMLButtonElement>;
  disabled?: boolean;
}) => {
  return (
    <button className={style.button} onClick={onClick} disabled={disabled}>
      {children}
    </button>
  );
};

export const ThemedInput = ({
  value,
  onInput,
  numeric,
  size = 'medium',
}: {
  value?:
    | string
    | number
    | string[]
    | JSX.SignalLike<string | number | string[] | undefined>;
  onInput?: JSX.DOMAttributes<HTMLInputElement>['onInput'];
  numeric?: boolean;
  size?: 'short' | 'medium';
}) => {
  const classNames = [style.input, style[size]];
  const numericProps = numeric
    ? {
        type: 'number',
        pattern: '[0-9]*',
        inputmode: 'numeric',
        onKeyPress: (event) => {
          if (!/[0-9]/.test(event.key)) {
            event.preventDefault();
          }
        },
      }
    : {};
  return (
    <input
      className={classNames.join(' ')}
      value={value}
      onInput={onInput}
      {...numericProps}
    />
  );
};
