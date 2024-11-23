import {
  PlayerId,
  Position,
  RoundSolving as RoundSolvingState,
  SolutionStep,
} from 'inertia-core';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';
import { RenderWhen } from '../utils/RenderWhen';
import { ThemedButton } from '../themed-form';
import { shake } from '../../animations/shake';
import { useEffect, useRef } from 'preact/hooks';
import { BlockText } from '../block-text';
import { PlayableBoard } from '../playable-board';
import { Timer } from '../timer';
import { useCountdown } from '../../utils/hooks/use-countdown';

export const RoundSolving = ({
  state,
  userPlayerId,
  countdownTimeLeft,
  position,
  onYieldSolve,
  onMoveActor,
}: {
  state: RoundSolvingState;
  userPlayerId: PlayerId;
  countdownTimeLeft: number;
  position: Position;
  onYieldSolve: () => void;
  onMoveActor: (step: SolutionStep) => void;
}) => {
  const solver = state.meta.player_info[state.solver]!;

  const isUserSolver = solver.player_id === userPlayerId;

  const movesMade = state.solution.length;
  const bidMoves = (
    state.player_bids.bids[solver.player_id] as { content: { value: number } }
  ).content.value;
  const isOutOfMoves = movesMade >= bidMoves;

  const giveUpButton = useRef<HTMLDivElement | null>(null);
  const emphasizeOutOfMoves = () => shake(giveUpButton.current);

  const { reset: resetCountdown, timeLeftMillis } = useCountdown({
    timeMillis: countdownTimeLeft,
  });

  useEffect(() => {
    resetCountdown(countdownTimeLeft);
  }, [countdownTimeLeft, resetCountdown]);

  return (
    <FlexCenter wrap>
      <FlexCenter wrap>
        <Bids
          players={state.meta.player_info}
          playerBids={state.player_bids}
          userPlayerId={userPlayerId}
          solving
        />
        <ThemedPanel>
          <FlexCenter column>
            <PanelTitle>Round {state.meta.round_number}</PanelTitle>
            <Divider />
            <div>{solver.player_name} solving...</div>
            <Timer time={timeLeftMillis} />
          </FlexCenter>

          <FlexCenter column>
            <Divider />
            <FlexCenter column>
              <BlockText>{`Moves used: ${movesMade}/${bidMoves}`}</BlockText>
              <RenderWhen when={isOutOfMoves}>
                <BlockText>Out of moves!</BlockText>
              </RenderWhen>

              <RenderWhen when={isUserSolver}>
                <div ref={giveUpButton}>
                  <ThemedButton
                    onClick={() => {
                      onYieldSolve();
                    }}
                  >
                    {isOutOfMoves ? 'Accept Failure' : 'Give Up'}
                  </ThemedButton>
                </div>
              </RenderWhen>
            </FlexCenter>
          </FlexCenter>
        </ThemedPanel>
      </FlexCenter>
      <PlayableBoard
        position={position}
        interactive={isUserSolver}
        onMoveActor={isOutOfMoves ? emphasizeOutOfMoves : onMoveActor}
      />
    </FlexCenter>
  );
};
