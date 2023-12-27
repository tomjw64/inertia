import {
  ActorSquares,
  PlayerId,
  RoundSolving as RoundSolvingState,
  SolutionStep,
} from 'inertia-core';
import { Countdown } from '../countdown';
import { Board } from '../board';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';
import { RenderWhen } from '../utils/RenderWhen';
import { ThemedButton } from '../themed-form';
import { shake } from '../../animations/shake';
import { useRef } from 'preact/hooks';
import { BlockText } from '../block-text';

export const RoundSolving = ({
  state,
  userPlayerId,
  countdownTimeLeft,
  actorSquares,
  onYieldSolve,
  onMoveActor,
}: {
  state: RoundSolvingState;
  userPlayerId: PlayerId;
  countdownTimeLeft: number;
  actorSquares: ActorSquares;
  onYieldSolve: () => void;
  onMoveActor: (step: SolutionStep) => void;
}) => {
  const solver = state.meta.player_info[state.solver];

  const isUserSolver = solver.player_id === userPlayerId;

  const movesMade = state.solution.length;
  const bidMoves = state.player_bids.bids[solver.player_id].content!.value;
  const isOutOfMoves = movesMade >= bidMoves;

  const giveUpButton = useRef<HTMLDivElement | null>(null);
  const emphasizeOutOfMoves = () => shake(giveUpButton.current);

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
            <Countdown timeLeft={countdownTimeLeft} />
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
      <Board
        walledBoard={state.board.walled_board}
        goal={state.board.goal}
        actorSquares={actorSquares}
        interactive={isUserSolver}
        onMoveActor={isOutOfMoves ? emphasizeOutOfMoves : onMoveActor}
      />
    </FlexCenter>
  );
};
