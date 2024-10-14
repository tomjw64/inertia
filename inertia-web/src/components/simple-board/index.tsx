import classNames from 'classnames';
import {
  ExpandedBitBoard,
  MetaBoardWrapper,
  Position,
  WalledBoard,
} from 'inertia-core';
import { animate } from 'motion';
import { useLayoutEffect, useRef } from 'preact/hooks';
import { getActorColor } from '../../utils/actor-colors';
import style from './style.module.scss';
import { FlexCenter } from '../flex-center';
import { BoardSelection, isActorSelection } from '../../utils/selection';

export const ACTOR_FLIP_ANIMATE_DURATION = 0.2;
export const MOVE_INDICATOR_ANIMATE_DURATION = 0.2;

const BOARD_FLIP_ATTR = 'data-flip-board';
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

type BoardCommonProps = {
  position: Position;
  selection: BoardSelection;
  interactive?: boolean;
  indicatorSquares?: ExpandedBitBoard;
  emphasizedIndicatorSquares?: ExpandedBitBoard;
  indicatorWalls?: WalledBoard;
  onClickRegion?: (event: SquareMouseEvent) => void;
  onMouseEnterRegion?: (event: SquareMouseEvent) => void;
  onMouseLeaveBoard?: () => void;
  metaBoard?: MetaBoardWrapper;
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
  'row' | 'column' | 'position' | 'indicatorWalls'
>;

type GoalSlotProps = Pick<
  BoardSquareProps,
  'squareIndex' | 'position' | 'selection'
>;

type ActorSlotProps = Pick<
  BoardSquareProps,
  'squareIndex' | 'selection' | 'position'
>;

type IndicatorSlotProps = Pick<
  BoardSquareProps,
  | 'squareIndex'
  | 'selection'
  | 'indicatorSquares'
  | 'emphasizedIndicatorSquares'
>;

type MetaDebugSlotProps = Pick<BoardSquareProps, 'squareIndex' | 'metaBoard'>;

type SquareRegionProps = Pick<
  BoardSquareProps,
  | 'row'
  | 'column'
  | 'squareIndex'
  | 'position'
  | 'interactive'
  | 'indicatorSquares'
  | 'onClickRegion'
  | 'onMouseEnterRegion'
> & { type: SquareRegionType };

// TODO: Get rid of so many queries inside event handlers. Hydrate only the
// first time if needed.

export const SimpleBoard = ({
  position,
  selection,
  interactive,
  indicatorSquares,
  emphasizedIndicatorSquares,
  indicatorWalls,
  onClickRegion,
  onMouseEnterRegion,
  onMouseLeaveBoard,
  metaBoard,
}: BoardProps) => {
  const boardElement = useRef<HTMLDivElement>(null);

  const boardFlipRect = useRef<DOMRect | null>(null);
  const actorFlipRects = useRef(new Map());

  const resetFlipRects = () => {
    const board = document.querySelector(`[${BOARD_FLIP_ATTR}]`);
    if (board) {
      boardFlipRect.current = board.getBoundingClientRect();
    }

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
    let boardDeltaX = 0;
    let boardDeltaY = 0;
    const originalBoardRect = boardFlipRect.current;
    const board = document.querySelector(`[${BOARD_FLIP_ATTR}]`);
    if (originalBoardRect && board) {
      const currentBoardRect = board.getBoundingClientRect();
      boardDeltaX = originalBoardRect.x - currentBoardRect.x;
      boardDeltaY = originalBoardRect.y - currentBoardRect.y;
      boardFlipRect.current = currentBoardRect;
    }

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
          const deltaX = firstRect.x - lastRect.x - boardDeltaX;
          const deltaY = firstRect.y - lastRect.y - boardDeltaY;

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
  }, [
    position.actor_squares,
    indicatorSquares,
    emphasizedIndicatorSquares,
    selection,
  ]);

  return (
    <div
      {...{ [BOARD_FLIP_ATTR]: true }}
      className={style.board}
      ref={boardElement}
      tabIndex={0}
      onMouseLeave={onMouseLeaveBoard}
    >
      {[...Array(16).keys()].map((row) => (
        <BoardRow
          {...{
            row,
            position,
            selection,
            interactive,
            indicatorSquares,
            emphasizedIndicatorSquares,
            indicatorWalls,
            onClickRegion,
            onMouseEnterRegion,
            metaBoard,
          }}
        />
      ))}
    </div>
  );
};

const BoardRow = ({
  row,
  position,
  selection,
  interactive,
  indicatorSquares,
  emphasizedIndicatorSquares,
  indicatorWalls,
  onClickRegion,
  onMouseEnterRegion,
  metaBoard,
}: BoardRowProps) => {
  return (
    <>
      {[...Array(16).keys()].map((column) => (
        <BoardSquare
          {...{
            row,
            column,
            squareIndex: row * 16 + column,
            position,
            selection,
            interactive,
            indicatorSquares,
            emphasizedIndicatorSquares,
            indicatorWalls,
            onClickRegion,
            onMouseEnterRegion,
            metaBoard,
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
  position,
  selection,
  interactive,
  indicatorSquares,
  emphasizedIndicatorSquares,
  indicatorWalls,
  onClickRegion,
  onMouseEnterRegion,
  metaBoard,
}: BoardSquareProps) => {
  return (
    <div className={style.square}>
      <IndicatorSlot
        {...{
          squareIndex,
          selection,
          interactive,
          indicatorSquares,
          emphasizedIndicatorSquares,
        }}
      />
      <GoalSlot {...{ squareIndex, position, selection, interactive }} />
      <BorderSlot {...{ row, column, position, indicatorWalls }} />
      <ActorSlot {...{ squareIndex, position, selection, interactive }} />
      <MetaDebugSlot {...{ squareIndex, metaBoard }} />
      {Object.values(SquareRegionType).map((type) => {
        return (
          <SquareRegion
            {...{
              type,
              row,
              column,
              squareIndex,
              position,
              interactive,
              indicatorSquares,
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
  type,
  row,
  column,
  squareIndex,
  position,
  interactive,
  indicatorSquares,
  onClickRegion,
  onMouseEnterRegion,
}: SquareRegionProps) => {
  const isActorHere = position.actor_squares.includes(squareIndex);
  const isIndicatorHere = indicatorSquares?.[squareIndex];
  const selectable =
    isIndicatorHere || (isActorHere && type === SquareRegionType.CENTER);

  const eventPayload = { row, column, squareIndex, region: type };
  const eventProps = {
    onClick:
      interactive && onClickRegion
        ? (originalEvent: MouseEvent) => {
            originalEvent.stopPropagation(); // For easier deselection
            onClickRegion(eventPayload);
          }
        : undefined,
    onMouseEnter:
      interactive && onMouseEnterRegion
        ? () => onMouseEnterRegion(eventPayload)
        : undefined,
  };
  return (
    <div
      className={classNames(style.slot, style.region, style[`region-${type}`], {
        [style['actor-adjusted']]: isActorHere,
        [style.selectable]: interactive && selectable,
      })}
      {...eventProps}
    />
  );
};

const BorderSlot = ({
  row,
  column,
  position,
  indicatorWalls,
}: BorderSlotProps) => {
  const { horizontal, vertical } = position.walled_board;
  const { horizontal: indicatorHorizontal, vertical: indicatorVertical } =
    indicatorWalls ?? {};
  const wallClasses = {
    [style['wall-bottom']]: horizontal[column]![row],
    [style['wall-top']]: horizontal[column]![row - 1],
    [style['wall-right']]: vertical[row]![column],
    [style['wall-left']]: vertical[row]![column - 1],
    [style['wall-bottom-indicator']]: indicatorHorizontal?.[column]?.[row],
    [style['wall-top-indicator']]: indicatorHorizontal?.[column]?.[row - 1],
    [style['wall-right-indicator']]: indicatorVertical?.[row]?.[column],
    [style['wall-left-indicator']]: indicatorVertical?.[row]?.[column - 1],
  };

  return <div className={classNames(style.slot, style.border, wallClasses)} />;
};

const MetaDebugSlot = ({ squareIndex, metaBoard }: MetaDebugSlotProps) => {
  const meta = metaBoard?.squares?.[squareIndex];
  if (meta == null) {
    return <></>;
  }
  return (
    <div className={classNames(style.slot)}>
      <FlexCenter expand>
        <span>{meta}</span>
      </FlexCenter>
    </div>
  );
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

const GoalSlot = ({ squareIndex, position, selection }: GoalSlotProps) => {
  const isGoalHere = position.goal === squareIndex;
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

const ActorSlot = ({ squareIndex, position, selection }: ActorSlotProps) => {
  const actorIndex = position.actor_squares.indexOf(squareIndex);
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
