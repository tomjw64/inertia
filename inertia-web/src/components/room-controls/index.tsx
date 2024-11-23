import { useRef } from 'preact/hooks';
import style from './style.module.scss';
import { usePopup } from '../../utils/popup';
import { Popup } from '../popup';

export const AppControls = () => {
  const sharePopupElement = useRef<HTMLDivElement>(null);
  const popup = usePopup();

  return (
    <>
      <div className={style.controls}>
        <div className={style.controlGroup}>
          <a href="/">
            <img className={style.control} src="/home.svg" />
          </a>
          <a
            onClick={() => {
              navigator.clipboard
                .writeText(window.location.href)
                .then(() => popup(sharePopupElement.current))
                .catch((e) => {
                  throw new Error('Failed to write to clipboard: ', e);
                });
            }}
          >
            <img className={style.control} src="/link.svg" />
            <Popup ref={sharePopupElement}>
              Shareable link copied to clipboard!
            </Popup>
          </a>
        </div>
        {/* <div className={style.controlGroup}>
          <a>
            <img className={style.control} src="/settings.svg" />
          </a>
        </div> */}
      </div>
    </>
  );
};
