import { PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';

export const PlayerPanel = ({
  players,
}: {
  players: Record<PlayerId, PlayerInfo>;
}) => {
  return (
    <div className={style.playerPanelWrapper}>
      <div className={style.playerPanelContent}>
        <div className={style.title}>Players</div>
        <Divider />
        <div className={style.playerPanelList}>
          {Object.values(players).map((playerInfo) => {
            return <PlayerItem data={playerInfo} />;
          })}
        </div>
      </div>
    </div>
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
