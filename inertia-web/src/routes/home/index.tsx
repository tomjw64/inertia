import debounce from 'lodash/debounce';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import style from './style.module.scss';
import { Starfield } from '../../components/starfield';
import { getPlayerName, savePlayerName } from '../../utils/storage';
import { generatePlayerName } from '../../utils/player-gen';
import { Divider } from '../../components/divider';
import { ThemedPanel } from '../../components/themed-panel';
import { Foreground } from '../../components/foreground';
import { FlexCenter } from '../../components/flex-center';
import {
  ThemedButton,
  ThemedFormLine,
  ThemedInput,
} from '../../components/themed-form';

export const Home = () => {
  const homeRef = useRef<HTMLDivElement | null>(null);
  const titleRef = useRef<HTMLDivElement | null>(null);

  const initialSavedPlayerName = useMemo(() => getPlayerName(), []);

  const [nameInput, setNameInput] = useState(initialSavedPlayerName);

  const debouncedSavePlayerName = debounce(savePlayerName, 200);

  const [joinGameInput, setJoinGameInput] = useState('');

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

    const debouncedResetAnimationVars = debounce(setAnimationVars, 200);
    window.addEventListener('resize', debouncedResetAnimationVars);
    const justInCaseInterval = setInterval(debouncedResetAnimationVars, 1000);
    return () => {
      clearInterval(justInCaseInterval);
      window.removeEventListener('resize', debouncedResetAnimationVars);
    };
  }, []);

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <div className={style.home} ref={homeRef}>
        <div className={[style.title, style.titleX].join(' ')} ref={titleRef}>
          <div className={style.titleY}>INERTIA</div>
        </div>
        <Foreground>
          <FlexCenter expand>
            <ThemedPanel>
              <FlexCenter column>
                <div className={style.subtitle}>Inertia</div>
                <Divider />
                <ThemedButton
                  onClick={() => {
                    window.location.href = `/room/${Math.floor(
                      Math.random() * 999_999
                    )}`;
                  }}
                >
                  Start Game
                </ThemedButton>
                <Divider text={'or'} />
                <ThemedFormLine>
                  <ThemedButton
                    onClick={() => {
                      window.location.href = `/room/${joinGameInput}`;
                    }}
                  >
                    Join Game
                  </ThemedButton>
                  <ThemedInput
                    size="short"
                    value={joinGameInput}
                    onInput={(e) => setJoinGameInput(e.currentTarget.value)}
                  />
                </ThemedFormLine>
                <Divider />
                <FlexCenter>
                  <div className={style.nameHeader}>Name:</div>
                  <ThemedFormLine>
                    <ThemedInput
                      value={nameInput}
                      onInput={(e) => {
                        setNameInput(e.currentTarget.value);
                        debouncedSavePlayerName(e.currentTarget.value);
                      }}
                    />
                    <ThemedButton
                      onClick={() => {
                        const generatedName = generatePlayerName();
                        setNameInput(generatedName);
                        debouncedSavePlayerName(generatedName);
                      }}
                    >
                      <img src="/refresh.svg" />
                    </ThemedButton>
                  </ThemedFormLine>
                </FlexCenter>
              </FlexCenter>
            </ThemedPanel>
          </FlexCenter>
        </Foreground>
      </div>
    </>
  );
};
