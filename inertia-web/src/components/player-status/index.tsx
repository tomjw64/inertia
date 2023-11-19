import { PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { ComponentChild, ComponentChildren } from 'preact';
import { RenderWhen } from '../utils/RenderWhen';

export const PlayerListItem = ({
  userPlayerId,
  playerInfo,
  children,
  modifier,
}: {
  userPlayerId: PlayerId;
  playerInfo: PlayerInfo;
  children?: ComponentChildren;
  modifier?: ComponentChild;
}) => {
  const isPlayer = playerInfo.player_id === userPlayerId;

  const playerItemClassNames = [style.playerItem];
  if (isPlayer) {
    playerItemClassNames.push(style.isPlayer);
  }

  return (
    <div className={playerItemClassNames.join(' ')}>
      <div className={style.playerNameAndStatus}>
        <div className={style.playerMetaStatus}>
          <RenderWhen when={playerInfo.player_connected}>
            <img className={style.connectedIcon} src="/connected.svg" />
          </RenderWhen>
          <RenderWhen when={!playerInfo.player_connected}>
            <img src="/disconnected.svg" />
          </RenderWhen>
        </div>
        <span className={style.playerName}>{playerInfo.player_name}</span>
        <div className={style.playerModifier}>{modifier}</div>
      </div>
      <div className={style.playerData}>{children}</div>
    </div>
  );
};
