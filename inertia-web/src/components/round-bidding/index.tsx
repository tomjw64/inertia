import {
  RoundStart as RoundStartState,
  RoundBidding as RoundBiddingState,
  PlayerId,
  ActorSquares,
} from 'inertia-core';
import { Countdown } from '../countdown';
import { Board } from '../board';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { ThemedButton, ThemedFormLine, ThemedInput } from '../themed-form';
import { useCallback, useEffect, useState } from 'preact/hooks';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';
import { isMobile } from '../../utils/is-mobile';
import { RenderWhen } from '../utils/RenderWhen';
import { BlockText } from '../spaced-text';

export const RoundBidding = ({
  state,
  userPlayerId,
  countdownTimeLeft,
  actorSquares,
  onBid,
  onReadyBid,
  onUnreadyBid,
}: {
  state: RoundStartState | RoundBiddingState;
  actorSquares: ActorSquares;
  userPlayerId: PlayerId;
  countdownTimeLeft: number;
  onBid: (value: number) => void;
  onReadyBid: () => void;
  onUnreadyBid: () => void;
}) => {
  const [pendingBid, setPendingBid] = useState('');

  const [showInvalidBid, setShowInvalidBid] = useState(false);

  const playerBids = 'player_bids' in state ? state.player_bids : undefined;
  const firstBidSubmitted = !!playerBids;
  const bidType = playerBids?.bids?.[userPlayerId]?.type ?? 'None';
  const currentBidValue = playerBids?.bids?.[userPlayerId]?.content?.value;
  const isBidReady = bidType === 'ProspectiveReady' || bidType === 'NoneReady';
  const isStatusReadyable =
    bidType === 'Prospective' ||
    bidType === 'ProspectiveReady' ||
    bidType === 'None' ||
    bidType === 'NoneReady';
  const canChangeReadyStatus = firstBidSubmitted && isStatusReadyable;

  const handleSubmitBid = useCallback(() => {
    const bidValue = parseInt(pendingBid);
    if (isNaN(bidValue)) {
      return;
    }
    if (currentBidValue != null && currentBidValue < bidValue) {
      setShowInvalidBid(true);
      return;
    }
    setShowInvalidBid(false);
    onBid(bidValue);
    setPendingBid('');
  }, [currentBidValue, onBid, pendingBid]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Enter') {
        handleSubmitBid();
      }
    },
    [handleSubmitBid]
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <FlexCenter wrap>
      <FlexCenter wrap>
        <Bids
          players={state.meta.player_info}
          playerBids={playerBids}
          userPlayerId={userPlayerId}
        />
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
            <RenderWhen when={showInvalidBid}>
              <BlockText>You can't raise your bid!</BlockText>
            </RenderWhen>
            <ThemedFormLine>
              <ThemedButton
                disabled={!canChangeReadyStatus}
                onClick={() => {
                  if (isBidReady) {
                    onUnreadyBid();
                  } else {
                    onReadyBid();
                  }
                }}
              >
                {isBidReady ? 'Unready' : 'Ready'}
              </ThemedButton>
              <ThemedInput
                autofocus={!isMobile()}
                size="short"
                value={pendingBid}
                numeric
                onInput={(e) => {
                  setPendingBid(e.currentTarget.value);
                }}
              />
              <ThemedButton disabled={isBidReady} onClick={handleSubmitBid}>
                Bid
              </ThemedButton>
            </ThemedFormLine>
          </FlexCenter>
        </ThemedPanel>
      </FlexCenter>
      <Board
        walledBoard={state.board.walled_board}
        goal={state.board.goal}
        actorSquares={actorSquares}
      />
    </FlexCenter>
  );
};
