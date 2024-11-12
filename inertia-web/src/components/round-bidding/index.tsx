import {
  RoundStart as RoundStartState,
  RoundBidding as RoundBiddingState,
  PlayerId,
  Position,
} from 'inertia-core';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { ThemedButton, ThemedFormLine, ThemedInput } from '../themed-form';
import { useCallback, useEffect, useState } from 'preact/hooks';
import { Divider } from '../divider';
import { PanelTitle } from '../panel-title';
import { Bids } from '../bids';
import { isMobile } from '../../utils/is-mobile';
import { RenderWhen } from '../utils/RenderWhen';
import { BlockText } from '../block-text';
import { PlayableBoard } from '../playable-board';
import { useCountdown } from '../../utils/hooks/use-countdown';
import { Timer } from '../timer';

export const RoundBidding = ({
  state,
  userPlayerId,
  countdownTimeLeft,
  position,
  onBid,
  onReadyBid,
  onUnreadyBid,
}: {
  state: RoundStartState | RoundBiddingState;
  position: Position;
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

  const {
    reset: resetFirstBidCountdown,
    timeLeftMillis: firstBidTimeLeftMillis,
  } = useCountdown({
    timeMillis: firstBidSubmitted ? 0 : countdownTimeLeft,
    paused: firstBidSubmitted,
  });

  const {
    reset: resetSecondBidCountdown,
    timeLeftMillis: secondBidTimeLeftMillis,
  } = useCountdown({
    timeMillis: firstBidSubmitted ? 0 : 60000,
    paused: !firstBidSubmitted,
  });

  useEffect(() => {
    if (!firstBidSubmitted) {
      resetFirstBidCountdown(countdownTimeLeft);
    } else {
      resetSecondBidCountdown(countdownTimeLeft);
    }
  }, [
    countdownTimeLeft,
    firstBidSubmitted,
    resetFirstBidCountdown,
    resetSecondBidCountdown,
  ]);

  const playerBid = playerBids?.bids?.[userPlayerId] ?? { type: 'None' };
  const bidType = playerBid.type ?? 'None';
  const currentBidValue =
    'content' in playerBid ? playerBid.content.value : null;
  const isBidReady = bidType === 'ProspectiveReady' || bidType === 'NoneReady';
  const isBidReadyable =
    bidType === 'Prospective' ||
    bidType === 'ProspectiveReady' ||
    bidType === 'None' ||
    bidType === 'NoneReady';
  const canChangeReadyStatus = firstBidSubmitted && isBidReadyable;

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
    [handleSubmitBid],
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
            <FlexCenter wrap>
              <FlexCenter column>
                <BlockText>First bid</BlockText>
                <Timer
                  time={firstBidTimeLeftMillis}
                  paused={firstBidSubmitted}
                />
              </FlexCenter>
              <FlexCenter column>
                <BlockText>All bids</BlockText>
                <Timer
                  time={secondBidTimeLeftMillis}
                  paused={!firstBidSubmitted}
                />
              </FlexCenter>
            </FlexCenter>
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
      <PlayableBoard position={position} />
    </FlexCenter>
  );
};
