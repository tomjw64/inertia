import {
  ActorSquares,
  PlayerId,
  RoundSolving as RoundSolvingState,
  SolutionStep,
} from 'inertia-core';
import { Countdown } from '../countdown';
import { Starfield } from '../starfield';
import { Board } from '../board';
import { Foreground } from '../foreground';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';
import { RenderWhen } from '../utils/RenderWhen';
import { ThemedButton } from '../themed-form';
import { useLayoutEffect } from 'preact/hooks';
import { animate } from 'motion';

const emphasizeOutOfMoves = () => {
  const outOfMovesElement = document.querySelector(
    `[data-animate-out-of-moves]`
  );
  if (outOfMovesElement == null) {
    return;
  }
  const animationOffset = [0, -1, 2, -4, 4, -4, 4, -4, 2, -1, 0];
  const animationOffsetAsTranslate = animationOffset.map(
    (offset) => `translateX(${offset}px)`
  );
  animate(
    outOfMovesElement,
    { transform: animationOffsetAsTranslate },
    { duration: 1 }
  );
};

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

  return (
    <div>
      <Starfield numStars={500} speed={0.5} />
      <Foreground>
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
                  <span>{`Moves used: ${movesMade}/${bidMoves}`}</span>
                  <RenderWhen when={isOutOfMoves}>
                    <span>{`Out of moves`}</span>
                  </RenderWhen>

                  <RenderWhen when={isUserSolver}>
                    <div data-animate-out-of-moves>
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
      </Foreground>
    </div>
  );
};
