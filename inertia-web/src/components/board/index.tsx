import {
  StateUpdater,
  useEffect,
  useMemo,
  useState,
  useLayoutEffect,
  useRef,
} from 'preact/hooks';
import {} from 'preact/hooks';
import debounce from 'lodash/debounce';
import { animate } from 'motion';
import style from './style.module.scss';
import {
  ActorSquares,
  Direction,
  ExpandedBitBoard,
  Square,
  WalledBoard,
} from 'inertia-core';
import {
  get_movement_for_actor,
  get_movement_ray_for_actor,
} from 'inertia-wasm';

const actorColors = ['red', 'blue', 'green', 'yellow'];
const getActorColor = (actorIndex: number) => {
  return actorColors[actorIndex];
};

const keySelectionMap = {
  r: 0,
  '1': 0,
  b: 1,
  '2': 1,
  g: 2,
  '3': 2,
  y: 3,
  '4': 3,
};

type MoveActorFunction = (actorIndex: number, squareIndex: number) => void;

type BoardProps = {
  walledBoard: WalledBoard;
  actorSquares: ActorSquares;
  moveActor: MoveActorFunction;
};

export const Board = ({ walledBoard, actorSquares, moveActor }: BoardProps) => {
  const [selectedActor, setSelectedActor] = useState(-1);

  const actorFlipRects = useRef(new Map()).current;
  const actorFlipAttr = 'data-animate-actor-flip-key';
  const actorFlipAnimateDuration = 0.2;

  const moveIndicatorAttr = 'data-animate-move-indicator';
  const moveIndicatorAnimateDelay = 0.2;
  const moveIndicatorAnimateDuration = 0.2;

  useEffect(() => {
    const resetActorFlipRects = () => {
      document
        .querySelectorAll(`[${actorFlipAttr}]`)
        .forEach((flippedActor) => {
          const flipKey = flippedActor.getAttribute(actorFlipAttr);
          actorFlipRects.set(flipKey, flippedActor.getBoundingClientRect());
        });
    };
    resetActorFlipRects();

    const debouncedResetActorFlipRects = debounce(resetActorFlipRects, 200);
    window.addEventListener('resize', debouncedResetActorFlipRects);
    window.addEventListener('scroll', debouncedResetActorFlipRects);
    return () => {
      window.removeEventListener('resize', debouncedResetActorFlipRects);
      window.removeEventListener('scroll', debouncedResetActorFlipRects);
    };
  }, []);

  useLayoutEffect(() => {
    document
      .querySelectorAll(`[${moveIndicatorAttr}]`)
      .forEach((moveIndicator) => {
        animate(
          moveIndicator,
          { opacity: [0, 1], scale: [0.1, 1] },
          {
            delay: 0,
            duration: moveIndicatorAnimateDuration,
            easing: 'ease-out',
          }
        );
      });
  }, [selectedActor]);

  useLayoutEffect(() => {
    document
      .querySelectorAll(`[${moveIndicatorAttr}]`)
      .forEach((moveIndicator) => {
        animate(
          moveIndicator,
          { opacity: [0, 1], scale: [0.1, 1] },
          {
            delay: moveIndicatorAnimateDelay,
            duration: moveIndicatorAnimateDuration,
            easing: 'ease-out',
          }
        );
      });
  }, [actorSquares]);

  useLayoutEffect(() => {
    document.querySelectorAll(`[${actorFlipAttr}]`).forEach((flippedActor) => {
      const flipKey = flippedActor.getAttribute(actorFlipAttr);
      const firstRect = actorFlipRects.get(flipKey);
      if (!firstRect) {
        return;
      }
      const lastRect = flippedActor.getBoundingClientRect();
      const deltaX = firstRect.x - lastRect.x;
      const deltaY = firstRect.y - lastRect.y;

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
        { duration: actorFlipAnimateDuration, easing: 'ease-in-out' }
      );
    });
  });

  const handleKeyPress = (e: KeyboardEvent) => {
    const selection = keySelectionMap[e.key];

    if (selection == null) {
      return;
    }

    setSelectedActor(selection);
  };

  useEffect(() => {
    window.addEventListener('keydown', handleKeyPress);
    return () => {
      window.removeEventListener('keydown', handleKeyPress);
    };
  }, [handleKeyPress]);

  const movementRaySquares = useMemo(() => {
    return Object.values(Direction)
      .map((direction) => {
        return {
          [direction]: get_movement_ray_for_actor(
            {
              walled_board: walledBoard,
              actor_squares: actorSquares,
            },
            selectedActor,
            direction
          ),
        } as Record<Direction, ExpandedBitBoard>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [walledBoard, actorSquares, selectedActor]);

  const movementSquares = useMemo(() => {
    return Object.values(Direction)
      .map((direction) => {
        return {
          [direction]: get_movement_for_actor(
            {
              walled_board: walledBoard,
              actor_squares: actorSquares,
            },
            selectedActor,
            direction
          ),
        } as Record<Direction, Square>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [walledBoard, actorSquares, selectedActor]);

  return (
    <div className={style.board}>
      {[...Array(16).keys()].map((row) => (
        <BoardRow
          {...{
            row,
            walledBoard,
            actorSquares,
            selectedActor,
            setSelectedActor,
            movementRaySquares,
            movementSquares,
            moveActor,
          }}
        />
      ))}
    </div>
  );
};

type BoardRowProps = {
  row: number;
  walledBoard: WalledBoard;
  actorSquares: ActorSquares;
  selectedActor: number;
  setSelectedActor: StateUpdater<number>;
  movementRaySquares: Record<Direction, ExpandedBitBoard>;
  movementSquares: Record<Direction, Square>;
  moveActor: MoveActorFunction;
};

const BoardRow = ({
  row,
  walledBoard,
  actorSquares,
  selectedActor,
  setSelectedActor,
  movementRaySquares,
  movementSquares,
  moveActor,
}: BoardRowProps) => {
  return (
    <>
      {[...Array(16).keys()].map((column) => (
        <BoardSquare
          {...{
            row,
            column,
            walledBoard,
            actorSquares,
            selectedActor,
            setSelectedActor,
            movementRaySquares,
            movementSquares,
            moveActor,
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
  actorSquares: ActorSquares;
  selectedActor: number;
  setSelectedActor: StateUpdater<number>;
  movementRaySquares: Record<Direction, ExpandedBitBoard>;
  movementSquares: Record<Direction, Square>;
  moveActor: MoveActorFunction;
};

const BoardSquare = ({
  row,
  column,
  walledBoard,
  actorSquares,
  selectedActor,
  setSelectedActor,
  movementRaySquares,
  movementSquares,
  moveActor,
}: BoardSquareProps) => {
  const { horizontal, vertical, goal } = walledBoard;
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
      setSelectedActor(actorIndex);
    };

    const actor = (
      <img
        src="/actor.svg"
        className={actorFeatures.join(' ')}
        data-animate-actor-flip-key={getActorColor(actorIndex)}
        onClick={onSelect}
      />
    );

    return <div className={features.join(' ')}>{actor}</div>;
  }

  if (isGoalHere) {
    return <div className={features.join(' ')} />;
  }

  for (const direction of Object.values(Direction)) {
    if (movementSquares[direction] === squareIndex) {
      const moveIndicator = (
        <div
          className={style[`move-${getActorColor(selectedActor)}`]}
          data-animate-move-indicator
        />
      );
      features.push(style['move-target']);
      return (
        <div
          className={features.join(' ')}
          onClick={() => moveActor(selectedActor, movementSquares[direction])}
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
        />
      );
      features.push(style['move-target']);
      return (
        <div
          className={features.join(' ')}
          onClick={() => moveActor(selectedActor, movementSquares[direction])}
        >
          {moveRayIndicator}
        </div>
      );
    }
  }

  return <div className={features.join(' ')} />;
};
