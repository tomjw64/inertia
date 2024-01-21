import { useRef } from 'preact/hooks';
import style from './style.module.scss';
import { animate } from 'motion';

export const AppControls = () => {
  const linkPopupElement = useRef<HTMLDivElement>(null);
  const linkPreventClose = useRef<boolean>(false);
  const linkPopupTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);

  const doPopup = () => {
    const linkPopup = linkPopupElement.current;
    if (linkPopup == null) {
      return;
    }
    const timeout = linkPopupTimeout.current;
    if (timeout != null) {
      clearTimeout(timeout);
    }
    linkPopup.classList.add(style.visible);
    linkPreventClose.current = true;
    animate(
      linkPopup,
      { opacity: [0, 1], scale: [0.1, 1] },
      {
        duration: 0.2,
        easing: 'ease-in-out',
      }
    ).finished.then(() => {
      linkPopupTimeout.current = setTimeout(() => {
        linkPreventClose.current = false;
        animate(
          linkPopup,
          { opacity: [1, 0], scale: [1, 0.1] },
          {
            duration: 0.2,
            easing: 'ease-in-out',
          }
        ).finished.then(() => {
          if (!linkPreventClose.current) {
            linkPopup.classList.remove(style.visible);
          }
        });
      }, 2000);
    });
  };

  return (
    <>
      <div className={style.controls}>
        <div className={style.controlGroup}>
          <a
            onClick={() => {
              window.location.href = '/';
            }}
          >
            <img className={style.control} src="/home.svg" />
          </a>
          <a
            onClick={() => {
              navigator.clipboard
                .writeText(window.location.href)
                .then(doPopup)
                .catch((e) => {
                  throw new Error('Failed to write to clipboard: ', e);
                });
            }}
          >
            <img className={style.control} src="/link.svg" />
            <div
              ref={linkPopupElement}
              className={style.controlPopup}
              data-animate-link-popup
            >
              Shareable link copied to clipboard!
            </div>
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
