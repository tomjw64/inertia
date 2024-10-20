import { ComponentChildren, JSX } from 'preact';
import style from './style.module.scss';

export const ThemedFormLine = ({
  children,
}: {
  children?: ComponentChildren;
}) => {
  return <div className={style.formLine}>{children}</div>;
};

export const ThemedLinkButton = ({
  children,
  href,
  onClick,
  disabled = false,
}: {
  children?: ComponentChildren;
  disabled?: boolean;
  onClick?: JSX.MouseEventHandler<HTMLAnchorElement>;
  href?: string;
}) => {
  if (disabled) {
    return <ThemedButton disabled>{children}</ThemedButton>;
  }
  return (
    <a className={style.button} onClick={onClick} href={href}>
      {children}
    </a>
  );
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
  autofocus = false,
  placeholder,
}: {
  value?:
    | string
    | number
    | string[]
    | JSX.SignalLike<string | number | string[] | undefined>;
  onInput?: JSX.DOMAttributes<HTMLInputElement>['onInput'];
  numeric?: boolean;
  autofocus?: boolean;
  size?: 'short' | 'medium';
  placeholder?: string;
}) => {
  const classNames = [style.input, style[size]];
  const numericProps = numeric
    ? {
        type: 'number',
        pattern: '[0-9]*',
        inputmode: 'numeric',
        onKeyPress: (event: KeyboardEvent) => {
          if (!/[0-9]/.test(event.key)) {
            event.preventDefault();
          }
        },
      }
    : {};
  return (
    <input
      autoFocus={autofocus}
      autofocus={autofocus}
      className={classNames.join(' ')}
      value={value}
      onInput={onInput}
      placeholder={placeholder}
      {...numericProps}
    />
  );
};

export type SelectOption<T> = {
  text: string;
  value: T;
};
export const ThemedSelect = <
  T extends JSX.HTMLAttributes<HTMLSelectElement>['value'],
>({
  options,
  value,
  onChange,
}: {
  options: SelectOption<T>[];
  value: T;
  onChange: (selected: T) => void;
}) => {
  return (
    <select
      className={style.select}
      value={value}
      onChange={(e) => onChange(e.currentTarget.value as T)}
    >
      {options.map(({ text, value }) => (
        <option value={value}>{text}</option>
      ))}
    </select>
  );
};
