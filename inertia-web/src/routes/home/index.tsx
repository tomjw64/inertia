import debounce from 'lodash/debounce';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import style from './style.module.scss';
import { Starfield } from '../../components/starfield';
import { getPlayerName, savePlayerName } from '../../utils/storage';
import { generatePlayerName } from '../../utils/player-gen';
import { Divider } from '../../components/divider';
import { ThemedPanel } from '../../components/themed-panel';
import { FlexCenter } from '../../components/flex-center';
import {
  ThemedButton,
  ThemedFormLine,
  ThemedInput,
  ThemedSelect,
} from '../../components/themed-form';
import { Difficulty } from 'inertia-core';

const DIFFICULTY_TO_VALUE = {
  [Difficulty.Easiest]: 0,
  [Difficulty.Easy]: 1,
  [Difficulty.Medium]: 2,
  [Difficulty.Hard]: 3,
  [Difficulty.Hardest]: 4,
};

const debouncedSavePlayerName = debounce(savePlayerName, 200);

export const Home = () => {
  const homeRef = useRef<HTMLDivElement | null>(null);
  const titleRef = useRef<HTMLDivElement | null>(null);

  const [isStartOptionsExpanded, setIsStartOptionsExpanded] = useState(false);
  const startOptionsIcon = isStartOptionsExpanded
    ? '/contract-arrow.svg'
    : '/expand-arrow.svg';
  const [minDifficulty, setMinDifficulty] = useState(Difficulty.Easiest);
  const [maxDifficulty, setMaxDifficulty] = useState(Difficulty.Hard);

  const initialSavedPlayerName = useMemo(() => getPlayerName(), []);

  const [nameInput, setNameInput] = useState(initialSavedPlayerName);

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
        <FlexCenter expand>
          <ThemedPanel>
            <FlexCenter column>
              <div className={style.subtitle}>Inertia</div>
              <Divider />
              <ThemedFormLine>
                <ThemedButton
                  onClick={() => {
                    window.location.href = `/room/${Math.floor(
                      Math.random() * 999_999
                    )}?minDifficulty=${minDifficulty}&maxDifficulty=${maxDifficulty}`;
                  }}
                >
                  Start Game
                </ThemedButton>
                <ThemedButton
                  onClick={() => {
                    setIsStartOptionsExpanded(!isStartOptionsExpanded);
                  }}
                >
                  <img src={startOptionsIcon} />
                </ThemedButton>
              </ThemedFormLine>
              <div
                className={style.difficultySelectionContainer}
                data-expanded={isStartOptionsExpanded}
              >
                <FlexCenter column>
                  <div className={style.difficultySelection}>
                    <FlexCenter expand justify="space-between">
                      <span className={style.difficultySelectionLabel}>
                        Min difficulty:
                      </span>
                      <ThemedSelect
                        value={minDifficulty}
                        onChange={(e) => {
                          const selection = e.currentTarget.value as Difficulty;
                          const other = maxDifficulty;

                          setMinDifficulty(selection);
                          if (
                            DIFFICULTY_TO_VALUE[selection] >
                            DIFFICULTY_TO_VALUE[other]
                          ) {
                            setMaxDifficulty(selection);
                          }
                        }}
                      >
                        {Object.keys(DIFFICULTY_TO_VALUE).map((difficulty) => (
                          <option>{difficulty}</option>
                        ))}
                      </ThemedSelect>
                    </FlexCenter>
                  </div>
                  <div className={style.difficultySelection}>
                    <FlexCenter expand justify="space-between">
                      <span className={style.difficultySelectionLabel}>
                        Max difficulty:
                      </span>
                      <ThemedSelect
                        value={maxDifficulty}
                        onChange={(e) => {
                          const selection = e.currentTarget.value as Difficulty;
                          const other = minDifficulty;

                          setMaxDifficulty(selection);
                          if (
                            DIFFICULTY_TO_VALUE[selection] <
                            DIFFICULTY_TO_VALUE[other]
                          ) {
                            setMinDifficulty(selection);
                          }
                        }}
                      >
                        {Object.keys(DIFFICULTY_TO_VALUE).map((difficulty) => (
                          <option>{difficulty}</option>
                        ))}
                      </ThemedSelect>
                    </FlexCenter>
                  </div>
                </FlexCenter>
              </div>
              <Divider text={'or'} />
              <ThemedFormLine>
                <ThemedButton
                  disabled={!joinGameInput}
                  onClick={() => {
                    window.location.href = `/room/${joinGameInput}`;
                  }}
                >
                  Join Game
                </ThemedButton>
                <ThemedInput
                  size="short"
                  numeric
                  value={joinGameInput}
                  onInput={(e) => setJoinGameInput(e.currentTarget.value)}
                  placeholder="Room #"
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
      </div>
    </>
  );
};
