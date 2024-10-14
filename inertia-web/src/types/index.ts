import { Position, SolutionStep } from 'inertia-core';

export type NamedPosition = {
  name: string;
  position: Position;
};

export type NamedSolution = {
  name: string;
  solution: SolutionStep[];
};
