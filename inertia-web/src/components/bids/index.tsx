import { PlayerBid, PlayerBids, PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { get_next_solver } from 'inertia-wasm';
import { RenderWhen } from '../utils/RenderWhen';
import { PlayerListItem } from '../player-status';

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
  const leader = playerBids == null ? undefined : get_next_solver(playerBids);

  return (
    <ThemedPanel>
      <FlexCenter column>
        <PanelTitle>Bids</PanelTitle>
        <Divider />
        <div className={style.playerList}>
          {Object.entries(players).map(([playerId, playerInfo]) => {
            return (
              <PlayerItem
                userPlayerId={userPlayerId}
                playerInfo={playerInfo}
                playerBid={playerBids?.bids?.[playerId] ?? { type: 'None' }}
                isLeader={leader?.toString() === playerId}
                solving={solving}
              />
            );
          })}
        </div>
      </FlexCenter>
    </ThemedPanel>
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
  const isBidReady = bidType === 'ProspectiveReady';
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
