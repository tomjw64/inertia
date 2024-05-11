import { Position } from 'inertia-core';
import { decode_position, encode_position } from 'inertia-wasm';
import debounce from 'lodash/debounce';
import { useEffect, useMemo, useState } from 'preact/hooks';
import { Board } from '../../components/board';
import { Divider } from '../../components/divider';
import { ErrorPage } from '../../components/error-page';
import { FlexCenter } from '../../components/flex-center';
import { PanelTitle } from '../../components/panel-title';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ThemedPanel } from '../../components/themed-panel';
import { defaultPositionBytes } from '../../utils/board';

const debouncedSetUrlParams = debounce((params: URLSearchParams) => {
  const currentState = window.history.state;
  const currentUrl = window.location.href;
  const newUrl = currentUrl.split('?')[0] + '?' + params.toString();
  window.history.replaceState(currentState, '', newUrl);
}, 200);

export const BoardEditor = () => {
  const originalOrDefaultPosition = useMemo(() => {
    const originalPositionBytes = new URLSearchParams(
      window.location.search,
    ).get('position');
    return decode_position(originalPositionBytes ?? defaultPositionBytes);
  }, []);

  const [position, setPosition] = useState<Position>(originalOrDefaultPosition);
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

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      <FlexCenter wrap>
        <ThemedPanel>
          <FlexCenter column>
            <PanelTitle>Board Editor</PanelTitle>
            <Divider />
          </FlexCenter>
        </ThemedPanel>
        <Board
          walledBoard={position.walled_board}
          goal={position.goal}
          actorSquares={position.actor_squares}
        />
      </FlexCenter>
    </>
  );
};
