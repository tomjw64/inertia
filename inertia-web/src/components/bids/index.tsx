import { PlayerBid, PlayerBids, PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { get_next_solver } from 'inertia-core';
import { RenderWhen } from '../utils/RenderWhen';
import { PlayerListItem } from '../player-status';
import { useState } from 'preact/hooks';
import { BlockText } from '../block-text';
import { FullWidth } from '../full-width';

export const Bids = ({
  players,
  playerBids,
  userPlayerId,
  solving = false,
}: {
  players: Record<PlayerId, PlayerInfo>;
  playerBids?: PlayerBids;
  userPlayerId: PlayerId;
  solving?: boolean;
}) => {
  const [isCompact, setIsCompact] = useState(Object.keys(players).length > 4);
  const leader = playerBids == null ? undefined : get_next_solver(playerBids);

  const allPlayers = Object.entries(players);
  const compactPlayers = allPlayers.filter(([playerId, _]) => {
    const isLeader = playerId === leader?.toString();
    const isUserPlayer = playerId === userPlayerId.toString();
    return !isCompact || isLeader || isUserPlayer;
  });

  const numOmittedPlayers = Object.keys(players).length - compactPlayers.length;
  const useCompactPlayers = numOmittedPlayers > 1;

  const displayedPlayers = useCompactPlayers ? compactPlayers : allPlayers;

  return (
    <div onClick={() => setIsCompact(!isCompact)}>
      <ThemedPanel>
        <FlexCenter column>
          <PanelTitle>Bids</PanelTitle>
          <Divider />
          <FullWidth>
            {displayedPlayers.map(([playerId, playerInfo]) => {
              return (
                <PlayerItem
                  key={playerId}
                  userPlayerId={userPlayerId}
                  playerInfo={playerInfo}
                  playerBid={
                    playerBids?.bids?.[parseInt(playerId)] ?? { type: 'None' }
                  }
                  isLeader={leader?.toString() === playerId}
                  solving={solving}
                />
              );
            })}
            <RenderWhen when={useCompactPlayers}>
              <BlockText>{numOmittedPlayers} more players</BlockText>
            </RenderWhen>
          </FullWidth>
        </FlexCenter>
      </ThemedPanel>
    </div>
  );
};

const PlayerItem = ({
  userPlayerId,
  playerInfo,
  playerBid,
  isLeader,
  solving,
}: {
  userPlayerId: PlayerId;
  playerInfo: PlayerInfo;
  playerBid: PlayerBid;
  isLeader: boolean;
  solving: boolean;
}) => {
  const bidType = playerBid.type;
  const bidText =
    bidType === 'None' || bidType === 'NoneReady'
      ? '-'
      : playerBid.content.value;
  const isBidReady = bidType === 'ProspectiveReady' || bidType === 'NoneReady';
  const isFailed = bidType === 'Failed';
  const readyBoxImgSrc = isBidReady
    ? '/check-box-checked.svg'
    : '/check-box-empty.svg';

  const playerBidClassNames = [style.playerBid];
  if (isFailed) {
    playerBidClassNames.push(style.failed);
  }

  return (
    <PlayerListItem
      userPlayerId={userPlayerId}
      playerInfo={playerInfo}
      modifier={
        <>
          <RenderWhen when={!solving}>
            <img src={readyBoxImgSrc} />
          </RenderWhen>
          <RenderWhen when={isFailed}>
            <img src="/fail.svg" />
          </RenderWhen>
          <RenderWhen when={isLeader}>
            <img src="/star.svg" />
          </RenderWhen>
        </>
      }
    >
      <span className={playerBidClassNames.join(' ')}>{bidText}</span>
    </PlayerListItem>
  );
};
