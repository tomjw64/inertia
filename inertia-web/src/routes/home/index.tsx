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
import { Difficulty } from 'inertia-core';
import { Tray } from '../../components/tray';
import { FullWidth } from '../../components/full-width';
import { JSX } from 'preact/jsx-runtime';
import { DIFFICULTIES, DIFFICULTY_TO_VALUE } from '../../constants/difficulty';
import { FullScreen } from '../../components/full-screen';

const DifficultyOptions = () => {
  return (
    <>
      {Object.keys(DIFFICULTY_TO_VALUE).map((difficulty) => (
        <option>{difficulty}</option>
      ))}
    </>
  );
};

const MultiplayerSection = () => {
  const [isStartOptionsExpanded, setIsStartOptionsExpanded] = useState(false);
  const startOptionsIcon = isStartOptionsExpanded
    ? '/contract-arrow.svg'
    : '/expand-arrow.svg';
  const [minDifficulty, setMinDifficulty] = useState(DIFFICULTIES.Easiest);
  const [maxDifficulty, setMaxDifficulty] = useState(DIFFICULTIES.Hard);

  const [joinGameInput, setJoinGameInput] = useState('');

  const onChangeMinDifficulty: JSX.DOMAttributes<HTMLSelectElement>['onChange'] =
    (e) => {
      const selection = e.currentTarget.value as Difficulty;
      const other = maxDifficulty;

      setMinDifficulty(selection);
      if (DIFFICULTY_TO_VALUE[selection] > DIFFICULTY_TO_VALUE[other]) {
        setMaxDifficulty(selection);
      }
    };

  const onChangeMaxDifficulty: JSX.DOMAttributes<HTMLSelectElement>['onChange'] =
    (e) => {
      const selection = e.currentTarget.value as Difficulty;
      const other = minDifficulty;

      setMaxDifficulty(selection);
      if (DIFFICULTY_TO_VALUE[selection] < DIFFICULTY_TO_VALUE[other]) {
        setMinDifficulty(selection);
      }
    };

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
                value={minDifficulty}
                onChange={onChangeMinDifficulty}
              >
                <DifficultyOptions />
              </ThemedSelect>
            </FlexCenter>
          </div>
          <div className={style.difficultySelection}>
            <FlexCenter expand justify="space-between">
              <span>Max difficulty:</span>
              <ThemedSelect
                value={maxDifficulty}
                onChange={onChangeMaxDifficulty}
              >
                <DifficultyOptions />
              </ThemedSelect>
            </FlexCenter>
          </div>
        </Tray>
      </FullWidth>
      <Divider text={'or'} narrow></Divider>
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
              <MultiplayerSection />
              <SettingsSection />
            </FlexCenter>
          </ThemedPanel>
        </FlexCenter>
      </FullScreen>
    </>
  );
};
