import {
  StateUpdater,
  useEffect,
  useMemo,
  useState,
  useLayoutEffect,
  useRef,
} from 'preact/hooks';
import { animate } from 'motion';
import style from './style.module.scss';
import {
  ActorSquares,
  Direction,
  ExpandedBitBoard,
  SolutionStep,
  Square,
  WalledBoard,
} from 'inertia-core';
import {
  get_movement_for_actor,
  get_movement_ray_for_actor,
} from 'inertia-wasm';
import { getActorColor } from '../../utils/actor-colors';

export const ACTOR_FLIP_ANIMATE_DURATION = 0.2;
export const MOVE_INDICATOR_ANIMATE_DELAY = ACTOR_FLIP_ANIMATE_DURATION;
export const MOVE_INDICATOR_ANIMATE_DURATION = 0.2;

const BOARD_FLIP_ATTR = 'data-flip-board';
const ACTOR_FLIP_ATTR = 'data-animate-actor-flip-key';
const MOVE_INDICATOR_ATTR = 'data-animate-move-indicator';

const KEY_SELECTION_MAP = {
  r: 0,
  b: 1,
  g: 2,
  y: 3,
};

type MoveActorFunction = (solutionStep: SolutionStep) => void;

type BoardProps = {
  walledBoard: WalledBoard;
  goal: number;
  actorSquares: ActorSquares;
  interactive?: boolean;
  onMoveActor?: MoveActorFunction;
};

export const Board = ({
  walledBoard,
  goal,
  actorSquares,
  interactive = false,
  onMoveActor = () => {},
}: BoardProps) => {
  const [selectedActor, setSelectedActor] = useState(-1);

  const actorSquaresForHookDependency = JSON.stringify(actorSquares);

  const boardElement = useRef<HTMLDivElement>(null);

  const boardFlipRect = useRef<DOMRect | null>(null);
  const actorFlipRects = useRef(new Map()).current;

  const animateMoveIndicators = (delay: number = 0) => {
    document
      .querySelectorAll(`[${MOVE_INDICATOR_ATTR}]`)
      .forEach((moveIndicator) => {
        const animation = animate(
          moveIndicator,
          { opacity: [0, 1], scale: [0.1, 1] },
          {
            // delay,
            duration: MOVE_INDICATOR_ANIMATE_DURATION,
            easing: 'ease-out',
          }
        );
        animation.pause();
        setTimeout(animation.play, delay * 1000);
      });
  };

  const resetFlipRects = () => {
    const board = document.querySelector(`[${BOARD_FLIP_ATTR}]`);
    if (board) {
      boardFlipRect.current = board.getBoundingClientRect();
    }

    document
      .querySelectorAll(`[${ACTOR_FLIP_ATTR}]`)
      .forEach((flippedActor) => {
        const flipKey = flippedActor.getAttribute(ACTOR_FLIP_ATTR);
        actorFlipRects.set(flipKey, flippedActor.getBoundingClientRect());
      });
  };

  resetFlipRects();

  useEffect(() => {
    const deselectActor = (e: MouseEvent) => {
      const target = e.target;
      if (
        target instanceof HTMLElement &&
        target.closest('[data-no-deselect]')
      ) {
        return;
      }
      setSelectedActor(-1);
    };
    window.addEventListener('click', deselectActor);
    return () => {
      window.removeEventListener('click', deselectActor);
    };
  });

  useLayoutEffect(() => {
    animateMoveIndicators();
  }, [selectedActor]);

  useLayoutEffect(() => {
    animateMoveIndicators(MOVE_INDICATOR_ANIMATE_DELAY);
  }, [actorSquares]);

  useLayoutEffect(() => {
    let baordDeltaX = 0;
    let baordDeltaY = 0;
    const originalBoardRect = boardFlipRect.current;
    const board = document.querySelector(`[${BOARD_FLIP_ATTR}]`);
    if (originalBoardRect && board) {
      const currentBoardRect = board.getBoundingClientRect();
      baordDeltaX = originalBoardRect.x - currentBoardRect.x;
      baordDeltaY = originalBoardRect.y - currentBoardRect.y;
      boardFlipRect.current = currentBoardRect;
    }

    document
      .querySelectorAll(`[${ACTOR_FLIP_ATTR}]`)
      .forEach((flippedActor) => {
        const flipKey = flippedActor.getAttribute(ACTOR_FLIP_ATTR);
        const firstRect = actorFlipRects.get(flipKey);
        if (!firstRect) {
          return;
        }
        const lastRect = flippedActor.getBoundingClientRect();
        const deltaX = firstRect.x - lastRect.x - baordDeltaX;
        const deltaY = firstRect.y - lastRect.y - baordDeltaY;

        actorFlipRects.set(flipKey, flippedActor.getBoundingClientRect());

        if (Math.abs(deltaX) < 1 && Math.abs(deltaY) < 1) {
          return;
        }
        animate(
          flippedActor,
          {
            transform: [
              `translate(${deltaX}px, ${deltaY}px)`,
              'translate(0px, 0px)',
            ],
          },
          { duration: ACTOR_FLIP_ANIMATE_DURATION, easing: 'ease-in-out' }
        );
      });
  }, [actorFlipRects, actorSquaresForHookDependency]);

  useLayoutEffect(() => {
    const boardElementCurrent = boardElement.current;
    if (boardElementCurrent == null) {
      return;
    }

    const handleKeyDown = (e: KeyboardEvent) => {
      if (!interactive) {
        return;
      }

      const selection = KEY_SELECTION_MAP[e.key];

      if (selection == null) {
        return;
      }

      setSelectedActor(selection);
    };

    boardElementCurrent.addEventListener('keydown', handleKeyDown);
    return () => {
      boardElementCurrent.removeEventListener('keydown', handleKeyDown);
    };
  }, [interactive]);

  const movementRaySquares = useMemo(() => {
    return Object.values(Direction)
      .map((direction) => {
        return {
          [direction]: get_movement_ray_for_actor(
            {
              walled_board: walledBoard,
              actor_squares: actorSquares,
              goal,
            },
            selectedActor,
            direction
          ),
        } as Record<Direction, ExpandedBitBoard>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [walledBoard, actorSquares, goal, selectedActor]);

  const movementSquares = useMemo(() => {
    return Object.values(Direction)
      .map((direction) => {
        return {
          [direction]: get_movement_for_actor(
            {
              walled_board: walledBoard,
              actor_squares: actorSquares,
              goal,
            },
            selectedActor,
            direction
          ),
        } as Record<Direction, Square>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [walledBoard, actorSquares, goal, selectedActor]);

  return (
    <div
      className={style.board}
      ref={boardElement}
      tabIndex={0}
      data-flip-board
    >
      {[...Array(16).keys()].map((row) => (
        <BoardRow
          {...{
            row,
            walledBoard,
            goal,
            actorSquares,
            selectedActor,
            setSelectedActor,
            movementRaySquares,
            movementSquares,
            interactive,
            onMoveActor,
          }}
        />
      ))}
    </div>
  );
};

type BoardRowProps = {
  row: number;
  walledBoard: WalledBoard;
  goal: number;
  actorSquares: ActorSquares;
  selectedActor: number;
  setSelectedActor: StateUpdater<number>;
  movementRaySquares: Record<Direction, ExpandedBitBoard>;
  movementSquares: Record<Direction, Square>;
  interactive: boolean;
  onMoveActor: MoveActorFunction;
};

const BoardRow = ({
  row,
  walledBoard,
  goal,
  actorSquares,
  selectedActor,
  setSelectedActor,
  movementRaySquares,
  movementSquares,
  interactive,
  onMoveActor,
}: BoardRowProps) => {
  return (
    <>
      {[...Array(16).keys()].map((column) => (
        <BoardSquare
          {...{
            row,
            column,
            walledBoard,
            goal,
            actorSquares,
            selectedActor,
            setSelectedActor,
            movementRaySquares,
            movementSquares,
            interactive,
            onMoveActor,
          }}
        />
      ))}
    </>
  );
};

type BoardSquareProps = {
  row: number;
  column: number;
  walledBoard: WalledBoard;
  goal: number;
  actorSquares: ActorSquares;
  selectedActor: number;
  setSelectedActor: StateUpdater<number>;
  movementRaySquares: Record<Direction, ExpandedBitBoard>;
  movementSquares: Record<Direction, Square>;
  interactive?: boolean;
  onMoveActor: MoveActorFunction;
};

const BoardSquare = ({
  row,
  column,
  walledBoard,
  goal,
  actorSquares,
  selectedActor,
  setSelectedActor,
  movementRaySquares,
  movementSquares,
  interactive,
  onMoveActor,
}: BoardSquareProps) => {
  const { horizontal, vertical } = walledBoard;
  const features = [style.square];

  if (horizontal[column][row]) {
    features.push(style['block-down']);
  }
  if (horizontal[column][row - 1]) {
    features.push(style['block-up']);
  }

  if (vertical[row][column]) {
    features.push(style['block-right']);
  }
  if (vertical[row][column - 1]) {
    features.push(style['block-left']);
  }

  const squareIndex = row * 16 + column;

  const isGoalHere = goal === squareIndex;

  if (isGoalHere) {
    features.push(style['goal']);
  }

  const actorIndex = actorSquares.indexOf(squareIndex);
  const isActorHere = actorIndex !== -1;

  if (isActorHere) {
    const actorFeatures = [
      style.actor,
      style[`actor-${getActorColor(actorIndex)}`],
    ];

    if (selectedActor === actorIndex) {
      actorFeatures.push(style.selected);
    }

    const onSelect = () => {
      if (!interactive) {
        return;
      }
      setSelectedActor(actorIndex);
    };

    const actor = (
      <img
        src="/actor.svg"
        className={actorFeatures.join(' ')}
        data-animate-actor-flip-key={getActorColor(actorIndex)}
        tabIndex={0}
        onClick={onSelect}
        onFocus={onSelect}
      />
    );

    return (
      <div className={features.join(' ')} data-no-deselect>
        {actor}
      </div>
    );
  }

  for (const direction of Object.values(Direction)) {
    if (movementSquares[direction] === squareIndex) {
      const moveIndicator = (
        <div
          className={style[`move-${getActorColor(selectedActor)}`]}
          data-animate-move-indicator
          data-no-deselect
        />
      );
      features.push(style['move-target']);
      return (
        <div
          className={features.join(' ')}
          data-no-deselect
          onClick={() => onMoveActor({ actor: selectedActor, direction })}
        >
          {moveIndicator}
        </div>
      );
    }
  }

  for (const direction of Object.values(Direction)) {
    if (movementRaySquares[direction][squareIndex]) {
      const moveRayIndicator = (
        <div
          className={style[`move-ray-${getActorColor(selectedActor)}`]}
          data-animate-move-indicator
          data-no-deselect
        />
      );
      features.push(style['move-target']);
      return (
        <div
          className={features.join(' ')}
          data-no-deselect
          onClick={() => onMoveActor({ actor: selectedActor, direction })}
        >
          {moveRayIndicator}
        </div>
      );
    }
  }

  return <div className={features.join(' ')} />;
};
