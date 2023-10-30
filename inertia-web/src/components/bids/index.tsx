import { PlayerBid, PlayerBids, PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { get_next_solver } from 'inertia-wasm';
import { RenderWhen } from '../utils/RenderWhen';

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
                playerName={playerInfo.player_name}
                playerBid={playerBids?.bids?.[playerId] ?? { type: 'None' }}
                isLeader={leader?.toString() === playerId}
                solving={solving}
                isPlayer={playerInfo.player_id === userPlayerId}
              />
            );
          })}
        </div>
      </FlexCenter>
    </ThemedPanel>
  );
};

const PlayerItem = ({
  playerName,
  playerBid,
  isPlayer,
  isLeader,
  solving,
}: {
  playerName: string;
  playerBid: PlayerBid;
  isPlayer: boolean;
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

  const playerItemClassNames = [style.playerItem];
  if (isPlayer) {
    playerItemClassNames.push(style.isPlayer);
  }

  const playerBidClassNames = [style.playerBid];
  if (isFailed) {
    playerBidClassNames.push(style.failed);
  }

  return (
    <div className={playerItemClassNames.join(' ')}>
      <div className={style.playerNameAndStatus}>
        <span className={style.playerName}>{playerName}</span>
        <div className={style.playerStatus}>
          <RenderWhen when={!solving}>
            <img src={readyBoxImgSrc} />
          </RenderWhen>
          <RenderWhen when={isFailed}>
            <img src="/fail.svg" />
          </RenderWhen>
          <RenderWhen when={isLeader}>
            <img src="/star.svg" />
          </RenderWhen>
        </div>
      </div>
      <span className={playerBidClassNames.join(' ')}>{bidText}</span>
    </div>
  );
};
