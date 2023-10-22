import style from './style.module.scss';

export const Divider = ({ text }: { text?: string }) => {
  if (text) {
    return <div className={style.dividerText}>{text}</div>;
  }
  return <div className={style.divider} />;
};
