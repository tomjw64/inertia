import {
  Direction,
  ExpandedBitBoard,
  get_movement_for_actor,
  get_movement_ray_for_actor,
  MetaBoardWrapper,
  Position,
  SolutionStep,
  Square,
} from 'inertia-core';
import { useMemo, useState } from 'preact/hooks';
import { fromSquares, removeSquares, union } from '../../utils/bitboard';
import { DIRECTIONS } from '../../utils/direction';
import { BoardSelection, useClickAwayDeselect } from '../../utils/selection';
import {
  SimpleBoard,
  SquareMouseEvent,
  SquareRegionType,
} from '../simple-board';

export const PlayableBoard = ({
  position,
  interactive,
  onMoveActor,
  metaBoard,
}: {
  position: Position;
  interactive?: boolean;
  onMoveActor?: (solutionStep: SolutionStep) => void;
  metaBoard?: MetaBoardWrapper;
}) => {
  const [selection, setSelection] = useState(BoardSelection.NONE);
  useClickAwayDeselect(setSelection);

  const movementRaySquares = useMemo(() => {
    return Object.values(DIRECTIONS)
      .map((direction) => {
        return {
          [direction]: removeSquares(
            get_movement_ray_for_actor(position, selection, direction),
            position.actor_squares,
          ),
        } as Record<Direction, ExpandedBitBoard>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [position, selection]);
  const indicatorSquares = useMemo(
    () => union(Object.values(movementRaySquares)),
    [movementRaySquares],
  );

  const movementSquares = useMemo(() => {
    return Object.values(DIRECTIONS)
      .map((direction) => {
        return {
          [direction]: get_movement_for_actor(position, selection, direction),
        } as Record<Direction, Square>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [position, selection]);
  const emphasizedIndicatorSquares = useMemo(
    () =>
      removeSquares(
        fromSquares(Object.values(movementSquares)),
        position.actor_squares,
      ),
    [movementSquares, position],
  );

  const onClickRegion = (event: SquareMouseEvent) => {
    const { squareIndex, region } = event;

    const selectingActorIndex = position.actor_squares.indexOf(squareIndex);
    if (region === SquareRegionType.CENTER && selectingActorIndex !== -1) {
      setSelection(
        selection === selectingActorIndex
          ? BoardSelection.NONE
          : selectingActorIndex,
      );
      return;
    }

    for (const direction of Object.values(DIRECTIONS)) {
      if (movementRaySquares[direction][squareIndex]) {
        onMoveActor?.({
          actor: selection,
          direction,
        });
        return;
      }
    }

    setSelection(BoardSelection.NONE);
  };

  return (
    <SimpleBoard
      metaBoard={metaBoard}
      position={position}
      selection={interactive ? selection : BoardSelection.NONE}
      interactive={interactive}
      onClickRegion={onClickRegion}
      indicatorSquares={interactive ? indicatorSquares : undefined}
      emphasizedIndicatorSquares={
        interactive ? emphasizedIndicatorSquares : undefined
      }
    />
  );
};
