import { Dispatch, StateUpdater, useEffect } from 'preact/hooks';
import { getActorIndex } from './actor-colors';
import { StateSetter } from './types';

export enum BoardSelection {
  GOAL = -2,
  NONE = -1,
  RED = getActorIndex('red'),
  BLUE = getActorIndex('blue'),
  GREEN = getActorIndex('green'),
  YELLOW = getActorIndex('yellow'),
}

export const useClickAwayDeselect = (
  setSelection: StateSetter<BoardSelection>,
) => {
  useEffect(() => {
    const listener = () => {
      // Click events don't propogate from the board
      setSelection(BoardSelection.NONE);
    };
    document.addEventListener('click', listener);
    return () => {
      document.removeEventListener('click', listener);
    };
  });
};

export const isActorSelection = (
  selection: BoardSelection,
): selection is 0 | 1 | 2 | 3 => {
  return selection >= 0;
};
