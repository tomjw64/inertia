import { ComponentChildren, Ref } from 'preact';
import style from './style.module.scss';
import { forwardRef } from 'preact/compat';

export const Popup = forwardRef(
  ({ children }: { children: ComponentChildren }, ref: Ref<HTMLDivElement>) => {
    return (
      <div ref={ref} className={style.popup}>
        {children}
      </div>
    );
  },
);
