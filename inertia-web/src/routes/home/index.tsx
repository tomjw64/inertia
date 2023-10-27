import debounce from 'lodash/debounce';
import { useEffect, useRef, useState } from 'preact/hooks';
import style from './style.module.scss';
import { Starfield } from '../../components/starfield';
import { getOrCreatePlayerName } from '../../utils/storage';
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

  const [nameInput, setNameInput] = useState(getOrCreatePlayerName());

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
        <Foreground>
          <FlexCenter expand>
            <ThemedPanel>
              <FlexCenter column>
                <div className={style.subtitle}>Inertia</div>
                <Divider />
                <ThemedButton>Start Game</ThemedButton>
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
                <ThemedFormLine>
                  <ThemedButton>Set Name</ThemedButton>
                  <ThemedInput
                    value={nameInput}
                    onInput={(e) => setNameInput(e.currentTarget.value)}
                  />
                  <ThemedButton
                    onClick={() => {
                      setNameInput(generatePlayerName());
                    }}
                  >
                    <img src="/refresh.svg" />
                  </ThemedButton>
                </ThemedFormLine>
              </FlexCenter>
            </ThemedPanel>
          </FlexCenter>
        </Foreground>
      </div>
    </>
  );
};
