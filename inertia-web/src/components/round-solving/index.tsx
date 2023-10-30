import { PlayerId, RoundSolving as RoundSolvingState } from 'inertia-core';
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

export const RoundSolving = ({
  state,
  userPlayerId,
  countdownTimeLeft,
  onYieldSolve,
}: {
  state: RoundSolvingState;
  userPlayerId: PlayerId;
  countdownTimeLeft: number;
  onYieldSolve: () => void;
}) => {
  const solver = state.meta.player_info[state.solver];

  const isUserSolver = solver.player_id === userPlayerId;

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
              <RenderWhen when={isUserSolver}>
                <FlexCenter column>
                  <Divider />
                  <ThemedButton
                    onClick={() => {
                      onYieldSolve();
                    }}
                  >
                    Give Up
                  </ThemedButton>
                </FlexCenter>
              </RenderWhen>
            </ThemedPanel>
          </FlexCenter>
          <Board
            walledBoard={state.board.walled_board}
            goal={state.board.goal}
            actorSquares={state.board.actor_squares}
          />
        </FlexCenter>
      </Foreground>
    </div>
  );
};
