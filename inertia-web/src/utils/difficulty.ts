import { Difficulty } from 'inertia-core';
import { Nullable } from './types';
import { useRange } from './hooks/use-range';
import { useCallback } from 'preact/hooks';

export const DIFFICULTIES: Record<Difficulty, Difficulty> = {
  Easiest: 'Easiest',
  Easy: 'Easy',
  Medium: 'Medium',
  Hard: 'Hard',
  Hardest: 'Hardest',
};

export const parseDifficulty = (difficulty: Nullable<string>) => {
  if (!difficulty) {
    return undefined;
  }
  return (DIFFICULTIES as Record<string, Difficulty>)[difficulty];
};

export const DIFFICULTY_TO_VALUE: Record<Difficulty, number> = {
  Easiest: 0,
  Easy: 1,
  Medium: 2,
  Hard: 3,
  Hardest: 4,
};

export const VALUE_TO_DIFFICULTY: Record<number, Difficulty> = {
  0: DIFFICULTIES.Easiest,
  1: DIFFICULTIES.Easy,
  2: DIFFICULTIES.Medium,
  3: DIFFICULTIES.Hard,
  4: DIFFICULTIES.Hardest,
};

export const DIFFICULTY_OPTIONS = Object.values(VALUE_TO_DIFFICULTY).map(
  (difficulty) => ({ text: difficulty, value: difficulty }),
);

export const useDifficultyRange = (
  initialMin: Difficulty,
  initialMax: Difficulty,
) => {
  const {
    min: minValue,
    setMin: setMinValue,
    max: maxValue,
    setMax: setMaxValue,
  } = useRange(
    DIFFICULTY_TO_VALUE[initialMin],
    DIFFICULTY_TO_VALUE[initialMax],
  );

  const setMinDifficulty = useCallback(
    (value: Difficulty) => setMinValue(DIFFICULTY_TO_VALUE[value]),
    [setMinValue],
  );

  const setMaxDifficulty = useCallback(
    (value: Difficulty) => setMaxValue(DIFFICULTY_TO_VALUE[value]),
    [setMaxValue],
  );

  return {
    minDifficulty: VALUE_TO_DIFFICULTY[minValue]!,
    setMinDifficulty,
    maxDifficulty: VALUE_TO_DIFFICULTY[maxValue]!,
    setMaxDifficulty,
  };
};
