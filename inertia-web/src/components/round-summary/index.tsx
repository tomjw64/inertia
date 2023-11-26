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
import { RenderWhen } from '../utils/RenderWhen';
import { BlockText } from '../spaced-text';

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
  const isGameStart = state.meta.round_number === 0;
  const roundStartButtonText = isGameStart ? 'Start Game' : ' Start Next Round';
  const wasLastRoundSolved = state.last_round_solution != null;
  const lastRoundSolutionMoves = state.last_round_solution?.length ?? -1;
  const lastRoundSolverName =
    state.last_solver != null
      ? state.meta.player_info[state.last_solver].player_name ?? 'unknown'
      : 'unknown';

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
            <RenderWhen when={wasLastRoundSolved}>
              <BlockText>{`${lastRoundSolverName} found a solution with ${lastRoundSolutionMoves} moves!`}</BlockText>
            </RenderWhen>
            <RenderWhen when={!isGameStart && !wasLastRoundSolved}>
              <BlockText>Nobody found a solution last round.</BlockText>
            </RenderWhen>
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
