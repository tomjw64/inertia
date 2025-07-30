import { ActorSquares, MetaBoardWrapper, WalledBoard } from 'inertia-core';
import {
  encode_position,
  encode_solution,
  get_min_assists_board,
  get_group_min_moves_board,
  get_min_moves_board,
  solve,
  get_min_crawls_board,
} from 'inertia-core';
import { useEffect, useState } from 'preact/hooks';
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
import { emptyBoard } from '../../utils/board';
import {
  clearUrlParams,
  useUrlSyncedPositionsState,
} from '../../utils/url-params';
import {
  BoardSelection,
  isActorSelection,
  useClickAwayDeselect,
} from '../../utils/selection';

// TODO: use wasm and reuse logic from walled_board.rs (requires internet)?
const toggleWall = (
  walls: WalledBoard,
  row: number,
  column: number,
  region: SquareRegionType,
) => {
  if (region === SquareRegionType.CENTER) {
    return;
  }
  if (region === SquareRegionType.LEFT && column !== 0) {
    walls.vertical[row]![column - 1] = !walls.vertical[row]![column - 1];
  } else if (region === SquareRegionType.RIGHT && column !== 15) {
    walls.vertical[row]![column] = !walls.vertical[row]![column];
  } else if (region === SquareRegionType.TOP && row !== 0) {
    walls.horizontal[column]![row - 1] = !walls.horizontal[column]![row - 1];
  } else if (region === SquareRegionType.BOTTOM && row !== 15) {
    walls.horizontal[column]![row] = !walls.horizontal[column]![row];
  }
};

export const BoardEditor = () => {
  const [metaBoardType, setMetaBoardType] = useState('');

  const [selection, setSelection] = useState(BoardSelection.NONE);
  useClickAwayDeselect(setSelection);

  const [positions, setPositions] = useUrlSyncedPositionsState();

  const [mouseOverIndicatorWall, setMouseOverIndicatorWall] =
    useState(emptyBoard());

  useEffect(() => {
    clearUrlParams(['solution']);
  }, []);

  const position = positions[0]?.position;
  if (!position) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>Could not parse position from url.</ErrorPage>
      </>
    );
  }

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
      const currentWalls = structuredClone(position.walled_board);
      toggleWall(currentWalls, row, column, region);
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
    const indicatorWall = emptyBoard();
    toggleWall(indicatorWall, row, column, region);
    setMouseOverIndicatorWall(indicatorWall);
  };

  const handleMouseLeaveBoard = () => {
    setMouseOverIndicatorWall(emptyBoard());
  };

  let metaBoard: MetaBoardWrapper | undefined;
  if (metaBoardType === 'min_moves') {
    metaBoard = get_min_moves_board(position);
  } else if (metaBoardType === 'group_min_moves') {
    metaBoard = get_group_min_moves_board(position);
  } else if (metaBoardType === 'min_assists') {
    metaBoard = get_min_assists_board(position);
  } else if (metaBoardType === 'min_crawls') {
    metaBoard = get_min_crawls_board(position);
  }

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
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
          indicatorWalls={mouseOverIndicatorWall}
          onClickRegion={handleClickRegion}
          onMouseEnterRegion={handleMouseEnterRegion}
          onMouseLeaveBoard={handleMouseLeaveBoard}
          interactive
        />
      </FlexCenter>
    </>
  );
};
