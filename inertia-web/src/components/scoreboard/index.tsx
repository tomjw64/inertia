import { PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';

export const Scoreboard = ({
  userPlayerId,
  players,
}: {
  userPlayerId: PlayerId;
  players: Record<PlayerId, PlayerInfo>;
}) => {
  return (
    <ThemedPanel>
      <FlexCenter column>
        <PanelTitle>Scoreboard</PanelTitle>
        <Divider />
        <div className={style.playerList}>
          {Object.values(players).map((playerInfo) => {
            return (
              <PlayerItem
                data={playerInfo}
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
  isPlayer,
  data,
}: {
  isPlayer: boolean;
  data: PlayerInfo;
}) => {
  const playerItemClassNames = [style.playerItem];
  if (isPlayer) {
    playerItemClassNames.push(style.isPlayer);
  }
  return (
    <div className={playerItemClassNames.join(' ')}>
      <span className={style.playerName}>{data.player_name}</span>
      <span className={style.playerScore}>{data.player_score}</span>
    </div>
  );
};
