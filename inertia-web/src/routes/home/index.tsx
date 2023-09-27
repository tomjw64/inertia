import { useEffect, useRef, useState } from 'preact/hooks';
import { Starfield } from '../../components/starfield';
import style from './style.module.scss';
import debounce from 'lodash/debounce';
import { getOrCreatePlayerName } from '../../utils/storage';
import { generatePlayerName } from '../../utils/player-name';

export const Home = () => {
  const homeRef = useRef<HTMLDivElement | null>(null);
  const titleRef = useRef<HTMLDivElement | null>(null);

  const [nameInput, setNameInput] = useState(getOrCreatePlayerName());

  useEffect(() => {
    const setAnimationVars = () => {
      const homeElement = homeRef.current;
      const titleElement = titleRef.current;

      if (!homeElement || !titleElement) {
        return;
      }

      homeElement.style.setProperty(
        '--bounce-width',
        titleElement.clientWidth.toString() + 'px'
      );
      homeElement.style.setProperty(
        '--bounce-height',
        titleElement.clientHeight.toString() + 'px'
      );
    };
    setAnimationVars();

    const debouncedResetActorFlipRects = debounce(setAnimationVars, 200);
    window.addEventListener('resize', debouncedResetActorFlipRects);
    const justInCaseInterval = setInterval(debouncedResetActorFlipRects, 1000);
    return () => {
      clearInterval(justInCaseInterval);
      window.removeEventListener('resize', debouncedResetActorFlipRects);
    };
  }, []);

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <div className={style.home} ref={homeRef}>
        <div className={[style.title, style.titleX].join(' ')} ref={titleRef}>
          <div className={style.titleY}>INERTIA</div>
        </div>
        <div className={style.foreground}>
          <div className={style.foregroundContentOuter}>
            <div className={style.foregroundContent}>
              <div className={style.subtitle}>Inertia</div>
              <div className={style.divider}></div>
              <div>
                <div className={style.inputButtonForm}>
                  <button className={style.button}>Set Name</button>
                  <input
                    className={[style.input, style.medium].join(' ')}
                    value={nameInput}
                    onChange={(e) => {
                      setNameInput(e.currentTarget.value);
                    }}
                  />
                  <button
                    className={style.button}
                    onClick={() => {
                      setNameInput(generatePlayerName());
                    }}
                  >
                    <img src="/refresh.svg" />
                  </button>
                </div>
              </div>
              <div className={style.divider}></div>
              <div>
                <button className={style.button}>Start Game</button>
              </div>
              <div className={style.dividerText}>or</div>
              <div className={style.joinGameSection}>
                <div className={style.inputButtonForm}>
                  <button className={style.button}>Join Game</button>
                  <input
                    className={[style.input, style.short].join(' ')}
                  ></input>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};
