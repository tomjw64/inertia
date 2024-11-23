import {
  encode_position,
  encode_solution,
  get_difficulty,
  PlayerId,
  Position,
  RoundSummary as RoundSummaryState,
} from 'inertia-core';
import { BlockText } from '../block-text';
import { Divider } from '../divider';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { PlayableBoard } from '../playable-board';
import { Scoreboard } from '../scoreboard';
import { ThemedButton } from '../themed-form';
import { ThemedPanel } from '../themed-panel';
import { RenderWhen } from '../utils/RenderWhen';

export const RoundSummary = ({
  state,
  userPlayerId,
  position,
  onStartRound,
}: {
  state: RoundSummaryState;
  userPlayerId: PlayerId;
  position: Position;
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
      ? state.meta.player_info[state.last_solver]?.player_name ?? 'unknown'
      : 'unknown';
  const lastRoundOptimalSolutionMoves =
    state.last_round_optimal_solution?.length ?? -1;
  const lastRoundDifficulty = state.last_round_optimal_solution
    ? get_difficulty(state.last_round_optimal_solution)
    : 'unknown';

  const openBoardExplorer = () => {
    const boardExplorerParams = new URLSearchParams();
    if (state.last_round_board) {
      boardExplorerParams.append(
        'position',
        encode_position(state.last_round_board),
      );
    }
    if (state.last_round_optimal_solution) {
      boardExplorerParams.append(
        'solution',
        `${encode_solution(state.last_round_optimal_solution)}:Optimal solution:`,
      );
    }
    if (state.last_round_solution) {
      boardExplorerParams.append(
        'solution',
        `${encode_solution(
          state.last_round_solution,
        )}:${lastRoundSolverName}'s solution`,
      );
    }
    const boardExplorerUrl = `/explore?${boardExplorerParams.toString()}`;
    window.open(boardExplorerUrl, '_blank');
  };

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
            <RenderWhen when={!isGameStart}>
              <Divider />
              <RenderWhen when={wasLastRoundSolved}>
                <BlockText>
                  {`${lastRoundSolverName} found a solution with ${lastRoundSolutionMoves} ${
                    lastRoundSolutionMoves === 1 ? ' move' : ' moves'
                  }!`}
                </BlockText>
              </RenderWhen>
              <RenderWhen when={!wasLastRoundSolved}>
                <BlockText>Nobody found a solution last round.</BlockText>
              </RenderWhen>
              <BlockText>
                {`Optimal solution: ${lastRoundOptimalSolutionMoves} ${
                  lastRoundOptimalSolutionMoves === 1 ? ' move' : ' moves'
                }`}
              </BlockText>
              <BlockText>{`Difficulty: ${lastRoundDifficulty}`}</BlockText>
            </RenderWhen>
            <Divider />
            <FlexCenter>
              <RenderWhen when={!isGameStart}>
                <ThemedButton onClick={openBoardExplorer}>
                  View in Board Explorer
                </ThemedButton>
              </RenderWhen>
              <ThemedButton onClick={onStartRound}>
                {roundStartButtonText}
              </ThemedButton>
            </FlexCenter>
          </FlexCenter>
        </ThemedPanel>
      </FlexCenter>
      <PlayableBoard position={position} />
    </FlexCenter>
  );
};
