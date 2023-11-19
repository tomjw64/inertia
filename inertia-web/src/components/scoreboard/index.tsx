import { PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { PlayerListItem } from '../player-status';

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
              <PlayerListItem
                userPlayerId={userPlayerId}
                playerInfo={playerInfo}
              >
                <span>{playerInfo.player_score}</span>
              </PlayerListItem>
            );
          })}
        </div>
      </FlexCenter>
    </ThemedPanel>
  );
};
