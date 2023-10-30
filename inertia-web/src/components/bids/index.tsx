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
                leader={leader?.toString() === playerId}
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
  leader,
  solving,
}: {
  playerName: string;
  playerBid: PlayerBid;
  isPlayer: boolean;
  leader: boolean;
  solving: boolean;
}) => {
  const playerItemClassNames = [style.playerItem];
  if (isPlayer) {
    playerItemClassNames.push(style.isPlayer);
  }

  const bidText = playerBid.type === 'None' ? '-' : playerBid.content.value;
  const isBidReady = playerBid.type === 'ProspectiveReady';
  const readyBoxImgSrc = isBidReady
    ? '/check-box-checked.svg'
    : '/check-box-empty.svg';

  return (
    <div className={playerItemClassNames.join(' ')}>
      <div className={style.playerNameAndStatus}>
        <span className={style.playerName}>{playerName}</span>
        <div className={style.playerStatus}>
          <RenderWhen when={!solving}>
            <img src={readyBoxImgSrc} />
          </RenderWhen>
          <RenderWhen when={leader}>
            <img src="/star.svg" />
          </RenderWhen>
        </div>
      </div>
      <span className={style.playerBid}>{bidText}</span>
    </div>
  );
};
