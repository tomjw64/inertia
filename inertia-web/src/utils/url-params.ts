import {
  decode_position,
  decode_solution,
  encode_position,
  encode_solution,
} from 'inertia-core';
import { debounce } from 'lodash';
import { useEffect, useMemo, useState } from 'preact/hooks';
import { NamedPosition, NamedSolution } from '../types';

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
  const initialUrlPositions = useInitialUrlPositions();
  const [positions, setPositions] = useState<NamedPosition[]>(
    initialUrlPositions.length ? initialUrlPositions : [],
  );
  useEffect(() => {
    debouncedSetUrlParam(
      'position',
      positions.map(
        (position) =>
          `${encode_position(position.position)}${position.name ? ':' + position.name : ''}`,
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
          `${encode_solution(solution.solution)}${solution.name ? ':' + solution.name : ''}`,
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
          const [positionBytes, name] = nameAndPosition;
          if (!positionBytes) {
            return [];
          }
          const position = decode_position(positionBytes);
          if (!position) {
            return [];
          }
          return [{ name, position }];
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
          const [solutionBytes, name] = nameAndSolution;
          if (!solutionBytes) {
            return [];
          }
          const solution = decode_solution(solutionBytes);
          if (!solution) {
            return [];
          }
          return [{ name, solution }];
        }),
    [],
  );
};
