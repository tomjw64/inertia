import {
  ActorSquares,
  encode_position,
  encode_solution,
  get_group_min_moves_board,
  get_min_assists_board,
  get_min_crawls_board,
  get_min_moves_board,
  Position,
  solve,
  WalledBoard,
} from 'inertia-core';
import { isEqual } from 'lodash';
import { useEffect, useMemo, useState } from 'preact/hooks';
import { Divider } from '../../components/divider';
import { ErrorPage } from '../../components/error-page';
import { FlexCenter } from '../../components/flex-center';
import { PanelTitle } from '../../components/panel-title';
import { AppControls } from '../../components/room-controls';
import {
  SimpleBoard,
  SquareMouseEvent,
  SquareRegionType,
} from '../../components/simple-board';
import { Starfield } from '../../components/starfield';
import { ThemedButton, ThemedFormLine } from '../../components/themed-form';
import { ThemedPanel } from '../../components/themed-panel';
import { NamedPosition } from '../../types';
import { emptyBoard } from '../../utils/board';
import {
  BoardSelection,
  isActorSelection,
  useClickAwayDeselect,
} from '../../utils/selection';
import { StateSetter } from '../../utils/types';
import {
  clearUrlParams,
  useUrlSyncedPositionsState,
} from '../../utils/url-params';

type WallCoords = {
  type: keyof WalledBoard;
  row: number;
  column: number;
};

const getWallForRegion = (
  row: number,
  column: number,
  region: SquareRegionType,
): WallCoords | undefined => {
  if (region === SquareRegionType.LEFT && column !== 0) {
    return {
      type: 'vertical',
      row,
      column: column - 1,
    };
  }
  if (region === SquareRegionType.RIGHT && column !== 15) {
    return {
      type: 'vertical',
      row,
      column,
    };
  }
  if (region === SquareRegionType.TOP && row !== 0) {
    return {
      type: 'horizontal',
      row: row - 1,
      column,
    };
  }
  if (region === SquareRegionType.BOTTOM && row !== 15) {
    return {
      type: 'horizontal',
      row,
      column,
    };
  }
  return undefined;
};

const toggleWall = (walls: WalledBoard, coords: WallCoords) => {
  const { type, column, row } = coords;
  const indexes: [number, number] =
    type === 'horizontal' ? [column, row] : [row, column];
  walls[type][indexes[0]]![indexes[1]] = !walls[type][indexes[0]]![indexes[1]];
};

export const BoardEditor = () => {
  const [positions, setPositions] = useUrlSyncedPositionsState();
  const position = positions[0]?.position;
  if (!position) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>Could not parse position from url.</ErrorPage>
      </>
    );
  }
  return <ValidBoardEditor position={position} setPositions={setPositions} />;
};

export const ValidBoardEditor = ({
  position,
  setPositions,
}: {
  position: Position;
  setPositions: StateSetter<NamedPosition[]>;
}) => {
  const [metaBoardType, setMetaBoardType] = useState('');

  const [selection, setSelection] = useState(BoardSelection.NONE);
  useClickAwayDeselect(setSelection);

  const [indicatorWallCoords, setIndicatorWallCoords] = useState<
    WallCoords | undefined
  >(undefined);

  const mouseOverIndicatorWalls = useMemo(() => {
    const indicatorWalls = emptyBoard();
    if (!indicatorWallCoords) {
      return indicatorWalls;
    }
    toggleWall(indicatorWalls, indicatorWallCoords);
    return indicatorWalls;
  }, [indicatorWallCoords]);

  useEffect(() => {
    clearUrlParams(['solution']);
  }, []);

  const metaBoard = useMemo(() => {
    if (metaBoardType === 'min_moves') {
      return get_min_moves_board(position);
    } else if (metaBoardType === 'group_min_moves') {
      return get_group_min_moves_board(position);
    } else if (metaBoardType === 'min_assists') {
      return get_min_assists_board(position);
    } else if (metaBoardType === 'min_crawls') {
      return get_min_crawls_board(position);
    }
  }, [metaBoardType, position]);

  const openBoardExplorer = () => {
    const boardExplorerParams = new URLSearchParams();
    boardExplorerParams.append('position', encode_position(position));
    const boardExplorerUrl = `/explore?${boardExplorerParams.toString()}`;
    window.open(boardExplorerUrl, '_blank');
  };

  const openSolvedInBoardExplorer = () => {
    const boardExplorerParams = new URLSearchParams();
    boardExplorerParams.append('position', encode_position(position));
    const start = Date.now();
    const solution = solve(position);
    const end = Date.now();
    if (solution) {
      console.log(
        `Solution of length ${solution?.length} solved in ${(end - start) / 1000} seconds.`,
      );
      boardExplorerParams.append(
        'solution',
        `${encode_solution(solution)}:Optimal`,
      );
    }
    const boardExplorerUrl = `/explore?${boardExplorerParams.toString()}`;
    window.open(boardExplorerUrl, '_blank');
  };

  const handleClickRegion = (event: SquareMouseEvent) => {
    const { squareIndex, region, row, column } = event;

    if (
      selection === BoardSelection.NONE &&
      region !== SquareRegionType.CENTER
    ) {
      const toggledWall = getWallForRegion(row, column, region);
      if (!toggledWall) {
        return;
      }
      const currentWalls = structuredClone(position.walled_board);
      toggleWall(currentWalls, toggledWall);
      setPositions([
        {
          name: '',
          position: {
            ...position,
            walled_board: currentWalls,
          },
        },
      ]);
      return;
    }

    const selectingActorIndex = position.actor_squares.indexOf(squareIndex);
    if (selectingActorIndex !== -1) {
      setSelection(
        selection === selectingActorIndex
          ? BoardSelection.NONE
          : selectingActorIndex,
      );
      return;
    }

    const isSelectingGoal = position.goal === squareIndex;
    if (isSelectingGoal) {
      setSelection(
        selection === BoardSelection.GOAL
          ? BoardSelection.NONE
          : BoardSelection.GOAL,
      );
      return;
    }

    if (isActorSelection(selection)) {
      const newActorSquares = [...position.actor_squares] as ActorSquares;
      newActorSquares[selection] = squareIndex;
      setPositions([
        {
          name: '',
          position: {
            ...position,
            actor_squares: newActorSquares,
          },
        },
      ]);
      setSelection(BoardSelection.NONE);
      return;
    }

    if (selection === BoardSelection.GOAL) {
      setPositions([
        {
          name: '',
          position: {
            ...position,
            goal: squareIndex,
          },
        },
      ]);
      setSelection(BoardSelection.NONE);
      return;
    }
  };

  const handleMouseEnterRegion = (event: SquareMouseEvent) => {
    if (selection !== BoardSelection.NONE) {
      return;
    }
    const { row, column, region } = event;
    if (!isEqual(indicatorWallCoords, getWallForRegion(row, column, region))) {
      setIndicatorWallCoords(getWallForRegion(row, column, region));
    }
  };

  const handleMouseLeaveBoard = () => {
    if (indicatorWallCoords) {
      setIndicatorWallCoords(undefined);
    }
  };

  return (
    <>
      <Starfield numStars={0} speed={0.5} />
      <AppControls />
      <FlexCenter wrap>
        <ThemedPanel>
          <FlexCenter column>
            <PanelTitle>Board Editor</PanelTitle>
            <Divider />
            <ThemedButton onClick={openBoardExplorer}>
              View in Board Explorer
            </ThemedButton>
            <ThemedButton onClick={openSolvedInBoardExplorer}>
              Solve in Board Explorer
            </ThemedButton>
            <Divider />
            MetaBoard Options
            <ThemedFormLine>
              <ThemedButton onClick={() => setMetaBoardType('')}>
                None
              </ThemedButton>
              <ThemedButton onClick={() => setMetaBoardType('min_assists')}>
                Min Assists
              </ThemedButton>
              <ThemedButton onClick={() => setMetaBoardType('min_moves')}>
                Min Moves
              </ThemedButton>
              <ThemedButton onClick={() => setMetaBoardType('group_min_moves')}>
                Group Min Moves
              </ThemedButton>
              <ThemedButton onClick={() => setMetaBoardType('min_crawls')}>
                Min Crawls
              </ThemedButton>
            </ThemedFormLine>
          </FlexCenter>
        </ThemedPanel>
        <SimpleBoard
          metaBoard={metaBoard}
          position={position}
          selection={selection}
          indicatorWalls={mouseOverIndicatorWalls}
          onClickRegion={handleClickRegion}
          onMouseEnterRegion={handleMouseEnterRegion}
          onMouseLeaveBoard={handleMouseLeaveBoard}
          interactive
        />
      </FlexCenter>
    </>
  );
};
