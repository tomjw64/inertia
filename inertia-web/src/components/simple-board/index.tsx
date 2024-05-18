import classNames from 'classnames';
import { ActorSquares, ExpandedBitBoard, WalledBoard } from 'inertia-core';
import { animate } from 'motion';
import { useLayoutEffect, useRef } from 'preact/hooks';
import { getActorColor, getActorIndex } from '../../utils/actor-colors';
import style from './style.module.scss';

export const ACTOR_FLIP_ANIMATE_DURATION = 0.2;
export const MOVE_INDICATOR_ANIMATE_DURATION = 0.2;

const ACTOR_FLIP_ATTR = 'data-animate-actor-flip-key';
const INDICATOR_ATTR = 'data-animate-indicator';

export enum SquareRegionType {
  TOP = 'top',
  BOTTOM = 'bottom',
  LEFT = 'left',
  RIGHT = 'right',
  CENTER = 'center',
}

export type SquareMouseEvent = {
  row: number;
  column: number;
  squareIndex: number;
  region: SquareRegionType;
};

export enum BoardSelection {
  GOAL = -2,
  NONE = -1,
  RED = getActorIndex('red'),
  BLUE = getActorIndex('blue'),
  GREEN = getActorIndex('green'),
  YELLOW = getActorIndex('yellow'),
}

export const isActorSelection = (
  selection: BoardSelection,
): selection is 0 | 1 | 2 | 3 => {
  return selection >= 0;
};

type BoardCommonProps = {
  walledBoard: WalledBoard;
  goal: number;
  actorSquares: ActorSquares;
  selection: BoardSelection;
  indicatorSquares?: ExpandedBitBoard;
  emphasizedIndicatorSquares?: ExpandedBitBoard;
  indicatorWalls?: WalledBoard;
  onClickRegion?: (event: SquareMouseEvent) => void;
  onMouseEnterRegion?: (event: SquareMouseEvent) => void;
  onMouseLeaveBoard?: () => void;
};

type BoardProps = BoardCommonProps;

type BoardRowProps = BoardCommonProps & {
  row: number;
};

type BoardSquareProps = BoardRowProps & {
  column: number;
  squareIndex: number;
};

type BorderSlotProps = Pick<
  BoardSquareProps,
  'row' | 'column' | 'walledBoard' | 'indicatorWalls'
>;

type GoalSlotProps = Pick<
  BoardSquareProps,
  'squareIndex' | 'goal' | 'selection'
>;

type ActorSlotProps = Pick<
  BoardSquareProps,
  'squareIndex' | 'selection' | 'actorSquares'
>;

type IndicatorSlotProps = Pick<
  BoardSquareProps,
  | 'squareIndex'
  | 'selection'
  | 'indicatorSquares'
  | 'emphasizedIndicatorSquares'
>;

type SquareRegionProps = Pick<
  BoardSquareProps,
  | 'row'
  | 'column'
  | 'squareIndex'
  | 'onClickRegion'
  | 'onMouseEnterRegion'
  | 'actorSquares'
> & { type: SquareRegionType };

// TODO: Get rid of so many queries inside event handlers. Hydrate only the
// first time if needed.

export const SimpleBoard = ({
  walledBoard,
  goal,
  actorSquares,
  selection,
  indicatorSquares,
  emphasizedIndicatorSquares,
  indicatorWalls,
  onClickRegion,
  onMouseEnterRegion,
  onMouseLeaveBoard,
}: BoardProps) => {
  const actorFlipRects = useRef(new Map());

  const resetFlipRects = () => {
    document
      .querySelectorAll(`[${ACTOR_FLIP_ATTR}]`)
      .forEach((flippedActor) => {
        const flipKey = flippedActor.getAttribute(ACTOR_FLIP_ATTR);
        actorFlipRects.current.set(
          flipKey,
          flippedActor.getBoundingClientRect(),
        );
      });
  };
  resetFlipRects();

  const initMoveIndicatorAnimations = () => {
    return Array.from(document.querySelectorAll(`[${INDICATOR_ATTR}]`)).map(
      (moveIndicator) => {
        const animation = animate(
          moveIndicator,
          { opacity: [0, 1], scale: [0.1, 1] },
          {
            duration: MOVE_INDICATOR_ANIMATE_DURATION,
            easing: 'ease-out',
          },
        );
        animation.pause();
        return animation;
      },
    );
  };

  useLayoutEffect(() => {
    const indicatorAnimations = initMoveIndicatorAnimations();
    const actorFlipAnimations = Promise.allSettled(
      Array.from(document.querySelectorAll(`[${ACTOR_FLIP_ATTR}]`)).map(
        (flippedActor) => {
          const flipKey = flippedActor.getAttribute(ACTOR_FLIP_ATTR);
          const firstRect = actorFlipRects.current.get(flipKey);
          if (!firstRect) {
            return Promise.resolve();
          }
          const lastRect = flippedActor.getBoundingClientRect();
          const deltaX = firstRect.x - lastRect.x;
          const deltaY = firstRect.y - lastRect.y;

          actorFlipRects.current.set(
            flipKey,
            flippedActor.getBoundingClientRect(),
          );

          if (Math.abs(deltaX) < 1 && Math.abs(deltaY) < 1) {
            return Promise.resolve();
          }
          return animate(
            flippedActor,
            {
              transform: [
                `translate(${deltaX}px, ${deltaY}px)`,
                'translate(0px, 0px)',
              ],
            },
            { duration: ACTOR_FLIP_ANIMATE_DURATION, easing: 'ease-in-out' },
          ).finished;
        },
      ),
    );
    actorFlipAnimations.then(() => {
      indicatorAnimations.forEach((animation) => animation.play());
    });
  }, [actorSquares, indicatorSquares, emphasizedIndicatorSquares, selection]);

  return (
    <div className={style.board} tabIndex={0} onMouseLeave={onMouseLeaveBoard}>
      {[...Array(16).keys()].map((row) => (
        <BoardRow
          {...{
            row,
            walledBoard,
            goal,
            actorSquares,
            selection,
            indicatorSquares,
            emphasizedIndicatorSquares,
            indicatorWalls,
            onClickRegion,
            onMouseEnterRegion,
          }}
        />
      ))}
    </div>
  );
};

const BoardRow = ({
  row,
  walledBoard,
  goal,
  actorSquares,
  selection,
  indicatorSquares,
  emphasizedIndicatorSquares,
  indicatorWalls,
  onClickRegion,
  onMouseEnterRegion,
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
            selection,
            indicatorSquares,
            emphasizedIndicatorSquares,
            indicatorWalls,
            onClickRegion,
            onMouseEnterRegion,
            squareIndex: row * 16 + column,
          }}
        />
      ))}
    </>
  );
};

const BoardSquare = ({
  row,
  column,
  squareIndex,
  walledBoard,
  goal,
  actorSquares,
  selection,
  indicatorSquares,
  emphasizedIndicatorSquares,
  indicatorWalls,
  onClickRegion,
  onMouseEnterRegion,
}: BoardSquareProps) => {
  return (
    <div className={style.square}>
      <IndicatorSlot
        {...{
          squareIndex,
          selection,
          indicatorSquares,
          emphasizedIndicatorSquares,
        }}
      />
      <GoalSlot {...{ squareIndex, goal, selection }} />
      <ActorSlot {...{ squareIndex, actorSquares, selection }} />
      <BorderSlot {...{ row, column, walledBoard, indicatorWalls }} />
      {Object.values(SquareRegionType).map((type) => {
        return (
          <SquareRegion
            {...{
              type,
              row,
              column,
              squareIndex,
              actorSquares,
              onClickRegion,
              onMouseEnterRegion,
            }}
          />
        );
      })}
    </div>
  );
};

const SquareRegion = ({
  row,
  column,
  squareIndex,
  actorSquares,
  type,
  onClickRegion,
  onMouseEnterRegion,
}: SquareRegionProps) => {
  const isActorPresent = actorSquares.includes(squareIndex);
  const eventPayload = { row, column, squareIndex, region: type };
  const eventProps = {
    onClick:
      onClickRegion &&
      ((originalEvent: MouseEvent) => {
        originalEvent.stopPropagation(); // For easier deselection
        onClickRegion(eventPayload);
      }),
    onMouseEnter:
      onMouseEnterRegion && (() => onMouseEnterRegion(eventPayload)),
  };
  return (
    <div
      className={classNames(style.slot, style.region, style[`region-${type}`], {
        [style['actor-adjusted']]: isActorPresent,
      })}
      {...eventProps}
    />
  );
};

const BorderSlot = ({
  row,
  column,
  walledBoard,
  indicatorWalls,
}: BorderSlotProps) => {
  const { horizontal, vertical } = walledBoard;
  const { horizontal: indicatorHorizontal, vertical: indicatorVertical } =
    indicatorWalls ?? {};
  const wallClasses = {
    [style['wall-bottom']]: horizontal[column][row],
    [style['wall-top']]: horizontal[column][row - 1],
    [style['wall-right']]: vertical[row][column],
    [style['wall-left']]: vertical[row][column - 1],
    [style['wall-bottom-indicator']]: indicatorHorizontal?.[column]?.[row],
    [style['wall-top-indicator']]: indicatorHorizontal?.[column]?.[row - 1],
    [style['wall-right-indicator']]: indicatorVertical?.[row]?.[column],
    [style['wall-left-indicator']]: indicatorVertical?.[row]?.[column - 1],
  };

  return <div className={classNames(style.slot, style.border, wallClasses)} />;
};

const IndicatorSlot = ({
  squareIndex,
  selection,
  indicatorSquares,
  emphasizedIndicatorSquares,
}: IndicatorSlotProps) => {
  if (!isActorSelection(selection)) {
    return <></>;
  }

  const actorColor = getActorColor(selection);
  const isIndicatorHere = indicatorSquares?.[squareIndex];
  const isEmphasized = emphasizedIndicatorSquares?.[squareIndex];

  if (!isIndicatorHere && !isEmphasized) {
    return <></>;
  }

  return (
    <div
      {...{ [INDICATOR_ATTR]: true }}
      className={classNames(style.slot, {
        [style[`indicator-${actorColor}`]]: isIndicatorHere,
        [style[`indicator-${actorColor}-emphasized`]]: isEmphasized,
      })}
    />
  );
};

const GoalSlot = ({ squareIndex, goal, selection }: GoalSlotProps) => {
  const isGoalHere = goal === squareIndex;
  if (!isGoalHere) {
    return <></>;
  }

  const isGoalSelected = selection === BoardSelection.GOAL;

  return (
    <div
      className={classNames(style.slot, style.goal, {
        [style.selected]: isGoalSelected,
      })}
    />
  );
};

const ActorSlot = ({
  squareIndex,
  actorSquares,
  selection,
}: ActorSlotProps) => {
  const actorIndex = actorSquares.indexOf(squareIndex);
  const isActorHere = actorIndex !== -1;
  if (!isActorHere) {
    return <></>;
  }

  const actorColor = getActorColor(actorIndex);
  const isSelected = selection === actorIndex;

  return (
    <img
      {...{ [ACTOR_FLIP_ATTR]: actorColor }}
      src="/actor.svg"
      className={classNames(
        style.slot,
        style.actor,
        style[`actor-${actorColor}`],
        {
          [style.selected]: isSelected,
        },
      )}
      tabIndex={0}
      onFocus={() => {
        /* TODO */
      }}
    />
  );
};
