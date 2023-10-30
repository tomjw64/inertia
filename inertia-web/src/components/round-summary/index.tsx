import { PlayerId, RoundSummary as RoundSummaryState } from 'inertia-core';
import { Scoreboard } from '../scoreboard';
import { Starfield } from '../starfield';
import { Board } from '../board';
import { emptyBoard } from '../../utils/board';
import { Foreground } from '../foreground';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { ThemedButton } from '../themed-form';
import { PanelTitle } from '../panel-title';
import { Divider } from '../divider';

export const RoundSummary = ({
  state,
  userPlayerId,
  onStartRound,
}: {
  state: RoundSummaryState;
  userPlayerId: PlayerId;
  onStartRound: () => void;
}) => {
  const roundPanelTitle =
    state.meta.round_number === 0
      ? 'Lobby'
      : `End of Round ${state.meta.round_number}`;
  const roundStartButtonText =
    state.meta.round_number === 0 ? 'Start Game' : ' Start Next Round';

  return (
    <div>
      <Starfield numStars={500} speed={0.5} />
      <Foreground>
        <FlexCenter wrap>
          <FlexCenter wrap>
            <Scoreboard
              players={state.meta.player_info}
              userPlayerId={userPlayerId}
            />
            <ThemedPanel>
              <FlexCenter column>
                <PanelTitle>{roundPanelTitle}</PanelTitle>
                <Divider />
                <ThemedButton onClick={onStartRound}>
                  {roundStartButtonText}
                </ThemedButton>
              </FlexCenter>
            </ThemedPanel>
          </FlexCenter>
          <Board
            walledBoard={state.last_round_board?.walled_board || emptyBoard()}
            goal={state.last_round_board?.goal ?? 255}
            actorSquares={state.last_round_board?.actor_squares ?? [0, 1, 2, 3]}
          />
        </FlexCenter>
      </Foreground>
    </div>
  );
};
