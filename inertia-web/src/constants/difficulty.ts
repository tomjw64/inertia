import { Difficulty } from 'inertia-core';
import { Nullable } from '../utils/types';

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
  Easy: 0,
  Medium: 0,
  Hard: 0,
  Hardest: 0,
};
