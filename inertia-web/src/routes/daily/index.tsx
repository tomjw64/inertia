import { FlexCenter } from '../../components/flex-center';
import { PlayableBoard } from '../../components/playable-board';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ThemedPanel } from '../../components/themed-panel';

export const Daily = () => {
  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      <FlexCenter wrap>
        <ThemedPanel></ThemedPanel>
        <PlayableBoard
          position={position}
          interactive
          onMoveActor={onMoveActor}
        />
      </FlexCenter>
    </>
  );
};
