import { decode_position } from 'inertia-wasm';
import { Board } from '../../components/board';
import { FlexCenter } from '../../components/flex-center';
import { RoomControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ErrorPage } from '../../components/error-page';

// defaultBoard = AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_

export const BoardExplorer = () => {
  const urlParams = new URLSearchParams(window.location.search);

  const positionBase64 = urlParams.get('position');
  if (!positionBase64) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>No board to show.</ErrorPage>
      </>
    );
  }

  const position = decode_position(positionBase64);
  if (!position) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>Could not parse position from url.</ErrorPage>
      </>
    );
  }

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <RoomControls />
      <FlexCenter wrap>
        <Board
          interactive
          walledBoard={position.walled_board}
          goal={position.goal}
          actorSquares={position.actor_squares}
        />
      </FlexCenter>
    </>
  );
};
