import { useState } from 'preact/hooks';
import style from './style.module.scss';
import { Starfield } from '../../components/starfield';
import { savePlayerName, getPlayerName } from '../../utils/storage';
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
import { Tray } from '../../components/tray';
import { FullWidth } from '../../components/full-width';
import {
  DIFFICULTIES,
  DIFFICULTY_OPTIONS,
  useDifficultyRange,
} from '../../utils/difficulty';
import { FullScreen } from '../../components/full-screen';

const SingleplayerSection = () => {
  const [isStartOptionsExpanded, setIsStartOptionsExpanded] = useState(false);
  const { minDifficulty, setMinDifficulty, maxDifficulty, setMaxDifficulty } =
    useDifficultyRange(DIFFICULTIES.Easiest, DIFFICULTIES.Hard);

  const startOptionsIcon = isStartOptionsExpanded
    ? '/contract-arrow.svg'
    : '/expand-arrow.svg';

  const startGame = () => {};
  return (
    <>
      <Divider text={'Singleplayer'} />
      <ThemedFormLine>
        <ThemedButton onClick={startGame}>Start Game</ThemedButton>
        <ThemedButton
          onClick={() => {
            setIsStartOptionsExpanded(!isStartOptionsExpanded);
          }}
        >
          <img src={startOptionsIcon} />
        </ThemedButton>
      </ThemedFormLine>
      <FullWidth>
        <Tray inset expanded={isStartOptionsExpanded}>
          <div className={style.difficultySelection}>
            <FlexCenter expand justify="space-between">
              <span>Min difficulty:</span>
              <ThemedSelect
                options={DIFFICULTY_OPTIONS}
                value={minDifficulty}
                onChange={setMinDifficulty}
              />
            </FlexCenter>
          </div>
          <div className={style.difficultySelection}>
            <FlexCenter expand justify="space-between">
              <span>Max difficulty:</span>
              <ThemedSelect
                options={DIFFICULTY_OPTIONS}
                value={maxDifficulty}
                onChange={setMaxDifficulty}
              />
            </FlexCenter>
          </div>
        </Tray>
      </FullWidth>
      <Divider text={'or'} narrow />
      <ThemedButton onClick={() => {}}>Daily Puzzle</ThemedButton>
    </>
  );
};

const MultiplayerSection = () => {
  const [isStartOptionsExpanded, setIsStartOptionsExpanded] = useState(false);
  const { minDifficulty, setMinDifficulty, maxDifficulty, setMaxDifficulty } =
    useDifficultyRange(DIFFICULTIES.Easiest, DIFFICULTIES.Hard);
  const [joinGameInput, setJoinGameInput] = useState('');

  const startOptionsIcon = isStartOptionsExpanded
    ? '/contract-arrow.svg'
    : '/expand-arrow.svg';

  const startGame = () => {
    window.location.href = `/room/${Math.floor(
      Math.random() * 999_999,
    )}?minDifficulty=${minDifficulty}&maxDifficulty=${maxDifficulty}`;
  };

  return (
    <>
      <Divider text={'Multiplayer'} />
      <ThemedFormLine>
        <ThemedButton onClick={startGame}>Start Game</ThemedButton>
        <ThemedButton
          onClick={() => {
            setIsStartOptionsExpanded(!isStartOptionsExpanded);
          }}
        >
          <img src={startOptionsIcon} />
        </ThemedButton>
      </ThemedFormLine>
      <FullWidth>
        <Tray inset expanded={isStartOptionsExpanded}>
          <div className={style.difficultySelection}>
            <FlexCenter expand justify="space-between">
              <span>Min difficulty:</span>
              <ThemedSelect
                options={DIFFICULTY_OPTIONS}
                value={minDifficulty}
                onChange={setMinDifficulty}
              />
            </FlexCenter>
          </div>
          <div className={style.difficultySelection}>
            <FlexCenter expand justify="space-between">
              <span>Max difficulty:</span>
              <ThemedSelect
                options={DIFFICULTY_OPTIONS}
                value={maxDifficulty}
                onChange={setMaxDifficulty}
              />
            </FlexCenter>
          </div>
        </Tray>
      </FullWidth>
      <Divider text={'or'} narrow />
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
    </>
  );
};

const SettingsSection = () => {
  const [nameInput, setNameInput] = useState(() => getPlayerName());

  const generateNewName = () => {
    const generatedName = generatePlayerName();
    setNameInput(generatedName);
    savePlayerName(generatedName);
  };

  return (
    <>
      <Divider text={'Settings'} />
      <FlexCenter>
        <div className={style.nameHeader}>Name:</div>
        <ThemedFormLine>
          <ThemedInput
            value={nameInput}
            onInput={(e) => {
              setNameInput(e.currentTarget.value);
              savePlayerName(e.currentTarget.value);
            }}
          />
          <ThemedButton onClick={generateNewName}>
            <img src="/refresh.svg" />
          </ThemedButton>
        </ThemedFormLine>
      </FlexCenter>
    </>
  );
};

export const Home = () => {
  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <FullScreen>
        <FlexCenter expand>
          <ThemedPanel>
            <FlexCenter column>
              <div className={style.title}>Inertia</div>
              <SingleplayerSection />
              <MultiplayerSection />
              <SettingsSection />
            </FlexCenter>
          </ThemedPanel>
        </FlexCenter>
      </FullScreen>
    </>
  );
};
