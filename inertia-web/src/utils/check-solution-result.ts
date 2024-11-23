import { CheckSolutionResult } from 'inertia-core';

export const CHECK_SOLUTION_RESULTS: Record<
  CheckSolutionResult,
  CheckSolutionResult
> = {
  NotASolution: 'NotASolution',
  InferiorSolution: 'InferiorSolution',
  ComparableSolution: 'ComparableSolution',
  SuperiorSolution: 'SuperiorSolution',
};
