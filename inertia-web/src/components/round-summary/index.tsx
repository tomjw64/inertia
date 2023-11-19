import {
  ActorSquares,
  PlayerId,
  RoundSummary as RoundSummaryState,
} from 'inertia-core';
import { Scoreboard } from '../scoreboard';
import { Board } from '../board';
import { emptyBoard } from '../../utils/board';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { ThemedButton } from '../themed-form';
import { PanelTitle } from '../panel-title';
import { Divider } from '../divider';

export const RoundSummary = ({
  state,
  userPlayerId,
  actorSquares,
  onStartRound,
}: {
  state: RoundSummaryState;
  userPlayerId: PlayerId;
  actorSquares: ActorSquares;
  onStartRound: () => void;
}) => {
  const roundPanelTitle =
    state.meta.round_number === 0
      ? 'Lobby'
      : `End of Round ${state.meta.round_number}`;
  const roundStartButtonText =
    state.meta.round_number === 0 ? 'Start Game' : ' Start Next Round';

  return (
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
        actorSquares={actorSquares}
      />
    </FlexCenter>
  );
};
