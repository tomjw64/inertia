import { RoundSolving as RoundSolvingState } from 'inertia-core';
import { Countdown } from '../countdown';
import { Starfield } from '../starfield';
import { Board } from '../board';
import { Foreground } from '../foreground';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { useState } from 'preact/hooks';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';

export const RoundSolving = ({
  state,
  countdownTimeLeft,
}: {
  state: RoundSolvingState;
  countdownTimeLeft: number;
}) => {
  const playerBids = 'player_bids' in state ? state.player_bids : undefined;
  const firstBidSubmitted = !!playerBids;

  return (
    <div>
      <Starfield numStars={500} speed={0.5} />
      <Foreground>
        <FlexCenter wrap>
          <FlexCenter wrap>
            <Bids players={state.meta.player_info} playerBids={playerBids} />
            <ThemedPanel>
              <FlexCenter column>
                <PanelTitle>Round {state.meta.round_number}</PanelTitle>
                <Divider />
                <div>{state.meta.player_info[state.solver]} solving...</div>
                <Countdown
                  timeLeft={firstBidSubmitted ? countdownTimeLeft : 60000}
                  paused={!firstBidSubmitted}
                />
              </FlexCenter>
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
