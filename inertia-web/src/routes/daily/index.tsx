import { useEffect, useState } from 'preact/hooks';
import { FlexCenter } from '../../components/flex-center';
import { PlayableBoard } from '../../components/playable-board';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ThemedPanel } from '../../components/themed-panel';
import { defaultPosition } from '../../utils/board';
import { getBackendUrl } from '../../utils/backend';
import { decode_position, Solution, SolutionStep } from 'inertia-core';
import { RenderWhen } from '../../components/utils/RenderWhen';
import { ErrorPage } from '../../components/error-page';
import { PanelTitle } from '../../components/panel-title';

export const Daily = () => {
  const [fetchError, setFetchError] = useState(false);
  const [date, setDate] = useState('');
  const [position, setPosition] = useState(defaultPosition());
  const [solution, setSolution] = useState<Solution>([]);

  useEffect(() => {
    const getDailyPosition = async () => {
      const response = await fetch(getBackendUrl('daily'));
      const data = await response.json();
      const decodedResponsePosition = decode_position(data.position);
      if (!decodedResponsePosition) {
        setFetchError(true);
        return;
      }
      setDate(data.date);
      setPosition(decodedResponsePosition);
    };
    getDailyPosition().catch((err) => {
      console.error(err);
      setFetchError(true);
    });
  }, []);

  const onMoveActor = (step: SolutionStep) => {
    setSolution([...solution, step]);
  };

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      <RenderWhen when={fetchError}>
        <ErrorPage>Server error. Try again later.</ErrorPage>;
      </RenderWhen>
      <RenderWhen when={!fetchError}>
        <FlexCenter wrap>
          <ThemedPanel>
            <FlexCenter column>
              <PanelTitle>Daily Puzzle: {date} </PanelTitle>
            </FlexCenter>
          </ThemedPanel>
          <PlayableBoard
            position={position}
            interactive
            onMoveActor={onMoveActor}
          />
        </FlexCenter>
      </RenderWhen>
    </>
  );
};
