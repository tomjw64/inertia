import { PlayerBid, PlayerBids, PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { get_next_solver } from 'inertia-wasm';

export const Bids = ({
  players,
  playerBids,
}: {
  players: Record<PlayerId, PlayerInfo>;
  playerBids?: PlayerBids;
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
  leader,
}: {
  playerName: string;
  playerBid: PlayerBid;
  leader: boolean;
}) => {
  const bidText = playerBid.type === 'None' ? '-' : playerBid.content.value;

  return (
    <div className={style.playerItem}>
      <div className={style.playerNameAndStatus}>
        <span className={style.playerName}>{playerName}</span>
        <div className={style.playerStatus}>
          {leader && <img src="/star.svg" />}
        </div>
      </div>
      <span className={style.playerBid}>{bidText}</span>
    </div>
  );
};
