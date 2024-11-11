import {
  decode_position,
  decode_solution,
  encode_position,
  encode_solution,
} from 'inertia-core';
import { debounce } from 'lodash';
import { useEffect, useMemo, useState } from 'preact/hooks';
import { NamedSolution } from '../types';

export const clearUrlParams = (params: string[]) => {
  for (const param of params) {
    setUrlParam(param, ['']);
  }
};

export const setUrlParam = (param: string, values: string[]) => {
  const currentState = window.history.state;
  const currentSearchParams = new URLSearchParams(window.location.search);
  currentSearchParams.delete(param);
  for (const value of values) {
    if (!value) {
      continue;
    }
    currentSearchParams.append(param, value);
  }
  const newUrl = [window.location.pathname, currentSearchParams.toString()]
    .filter(Boolean)
    .join('?');
  window.history.replaceState(currentState, '', newUrl);
};

export const debouncedSetUrlParam = debounce(setUrlParam, 200);

export const useUrlSyncedPositionsState = () => {
  const initialUrlPosition = useInitialUrlPositions();
  const [positions, setPositions] = useState(initialUrlPosition);
  useEffect(() => {
    debouncedSetUrlParam(
      'position',
      positions.map(
        (position) =>
          `${position.name ? position.name + ':' : ''}${encode_position(position.position)}`,
      ),
    );
  }, [positions]);
  return [positions, setPositions] as [typeof positions, typeof setPositions];
};

export const useUrlSyncedSolutionsState = () => {
  const initialUrlSolutions = useInitialUrlSolutions();
  const [solutions, setSolutions] =
    useState<NamedSolution[]>(initialUrlSolutions);
  useEffect(() => {
    debouncedSetUrlParam(
      'solution',
      solutions.map(
        (solution) =>
          `${solution.name ? solution.name + ':' : ''}:${encode_solution(solution.solution)}`,
      ),
    );
  }, [solutions]);
  return [solutions, setSolutions] as [typeof solutions, typeof setSolutions];
};

export const useInitialUrlPositions = () => {
  return useMemo(
    () =>
      new URLSearchParams(window.location.search)
        .getAll('position')
        .flatMap((param) => {
          const nameAndPosition = param.split(':');
          if (nameAndPosition.length <= 0) {
            return [];
          }
          if (nameAndPosition.length > 2) {
            return [];
          }
          const hasName = nameAndPosition.length == 2;
          const name = hasName ? nameAndPosition[0]! : '';
          const positionBytes = hasName
            ? nameAndPosition[1]!
            : nameAndPosition[0]!;
          const position = decode_position(positionBytes);
          return position ? [{ name, position }] : [];
        }),
    [],
  );
};

export const useInitialUrlSolutions = () => {
  return useMemo(
    () =>
      new URLSearchParams(window.location.search)
        .getAll('solution')
        .flatMap((param) => {
          const nameAndSolution = param.split(':');
          if (nameAndSolution.length <= 0) {
            return [];
          }
          if (nameAndSolution.length > 2) {
            return [];
          }
          const hasName = nameAndSolution.length == 2;
          const name = hasName ? nameAndSolution[0]! : '';
          const solutionBytes = hasName
            ? nameAndSolution[1]!
            : nameAndSolution[0]!;
          const solution = decode_solution(solutionBytes);
          return solution ? [{ name, solution }] : [];
        }),
    [],
  );
};
