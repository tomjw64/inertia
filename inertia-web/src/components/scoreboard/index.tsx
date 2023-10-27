import { PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';

export const Scoreboard = ({
  players,
}: {
  players: Record<PlayerId, PlayerInfo>;
}) => {
  return (
    <ThemedPanel>
      <FlexCenter column>
        <PanelTitle>Scoreboard</PanelTitle>
        <Divider />
        <div className={style.playerList}>
          {Object.values(players).map((playerInfo) => {
            return <PlayerItem data={playerInfo} />;
          })}
        </div>
      </FlexCenter>
    </ThemedPanel>
  );
};

const PlayerItem = ({ data }: { data: PlayerInfo }) => {
  return (
    <div className={style.playerItem}>
      <span className={style.playerName}>{data.player_name}</span>
      <span className={style.playerScore}>{data.player_score}</span>
    </div>
  );
};
