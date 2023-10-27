import {
  RoundStart as RoundStartState,
  RoundBidding as RoundBiddingState,
} from 'inertia-core';
import { Countdown } from '../countdown';
import { Starfield } from '../starfield';
import { Board } from '../board';
import { Foreground } from '../foreground';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { ThemedButton, ThemedFormLine, ThemedInput } from '../themed-form';
import { useState } from 'preact/hooks';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';

export const RoundBidding = ({
  state,
  countdownTimeLeft,
  onBid,
}: {
  state: RoundStartState | RoundBiddingState;
  countdownTimeLeft: number;
  onBid: (value: number) => void;
}) => {
  const [bid, setBid] = useState('');

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
                <div>First bid</div>
                <Countdown
                  timeLeft={countdownTimeLeft}
                  paused={firstBidSubmitted}
                />
                <div>All bids</div>
                <Countdown
                  timeLeft={firstBidSubmitted ? countdownTimeLeft : 60000}
                  paused={!firstBidSubmitted}
                />
                <Divider />
                <ThemedFormLine>
                  <ThemedButton
                    onClick={() => {
                      const bidValue = parseInt(bid);
                      onBid(bidValue);
                      setBid('');
                    }}
                  >
                    Bid
                  </ThemedButton>
                  <ThemedInput
                    size="short"
                    value={bid}
                    numeric
                    onInput={(e) => setBid(e.currentTarget.value)}
                  />
                  <ThemedButton
                    onClick={() => {
                      // onReadyBid();
                    }}
                  >
                    Ready
                  </ThemedButton>
                </ThemedFormLine>
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
