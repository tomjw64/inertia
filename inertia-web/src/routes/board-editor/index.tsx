import { ActorSquares, Position, WalledBoard } from 'inertia-core';
import { decode_position, encode_position } from 'inertia-wasm';
import debounce from 'lodash/debounce';
import { useEffect, useMemo, useState } from 'preact/hooks';
import { Divider } from '../../components/divider';
import {
  BoardSelection,
  SimpleBoard,
  SquareMouseEvent,
  SquareRegionType,
  isActorSelection,
} from '../../components/simple-board';
import { ErrorPage } from '../../components/error-page';
import { FlexCenter } from '../../components/flex-center';
import { PanelTitle } from '../../components/panel-title';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ThemedButton } from '../../components/themed-form';
import { ThemedPanel } from '../../components/themed-panel';
import { defaultPositionBytes, emptyBoard } from '../../utils/board';

const debouncedSetUrlParams = debounce((params: URLSearchParams) => {
  const currentState = window.history.state;
  const currentUrl = window.location.href;
  const newUrl = currentUrl.split('?')[0] + '?' + params.toString();
  window.history.replaceState(currentState, '', newUrl);
}, 200);

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
    walls.vertical[row][column - 1] = !walls.vertical[row][column - 1];
  } else if (region === SquareRegionType.RIGHT && column !== 15) {
    walls.vertical[row][column] = !walls.vertical[row][column];
  } else if (region === SquareRegionType.TOP && row !== 0) {
    walls.horizontal[column][row - 1] = !walls.horizontal[column][row - 1];
  } else if (region === SquareRegionType.BOTTOM && row !== 15) {
    walls.horizontal[column][row] = !walls.horizontal[column][row];
  }
};

export const BoardEditor = () => {
  const originalOrDefaultPosition = useMemo(() => {
    const originalPositionBytes = new URLSearchParams(
      window.location.search,
    ).get('position');
    return decode_position(originalPositionBytes ?? defaultPositionBytes);
  }, []);

  const [selection, setSelection] = useState<BoardSelection>(
    BoardSelection.NONE,
  );
  const [position, setPosition] = useState<Position>(originalOrDefaultPosition);
  const [mouseOverIndicatorWall, setMouseOverIndicatorWall] =
    useState<WalledBoard>(emptyBoard());

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

  useEffect(() => {
    const urlParams = new URLSearchParams();
    if (position) {
      urlParams.append('position', encode_position(position));
    }
    debouncedSetUrlParams(urlParams);
  }, [position]);

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

  const handleClickRegion = (event: SquareMouseEvent) => {
    const { squareIndex, region, row, column } = event;

    if (
      selection === BoardSelection.NONE &&
      region !== SquareRegionType.CENTER
    ) {
      const currentWalls = structuredClone(position.walled_board);
      toggleWall(currentWalls, row, column, region);
      setPosition({
        ...position,
        walled_board: currentWalls,
      });
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
      setPosition({
        ...position,
        actor_squares: newActorSquares,
      });
      setSelection(BoardSelection.NONE);
      return;
    }

    if (selection === BoardSelection.GOAL) {
      setPosition({
        ...position,
        goal: squareIndex,
      });
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
          </FlexCenter>
        </ThemedPanel>
        <SimpleBoard
          walledBoard={position.walled_board}
          goal={position.goal}
          actorSquares={position.actor_squares}
          selection={selection}
          indicatorWalls={mouseOverIndicatorWall}
          onClickRegion={handleClickRegion}
          onMouseEnterRegion={handleMouseEnterRegion}
          onMouseLeaveBoard={handleMouseLeaveBoard}
        />
      </FlexCenter>
    </>
  );
};
