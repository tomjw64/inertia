import { PlayerBid, PlayerBids, PlayerId, PlayerInfo } from 'inertia-core';
import style from './style.module.scss';
import { Divider } from '../divider';
import { ThemedPanel } from '../themed-panel';
import { FlexCenter } from '../flex-center';
import { PanelTitle } from '../panel-title';
import { get_next_solver } from 'inertia-wasm';
import { RenderWhen } from '../utils/RenderWhen';
import { PlayerListItem } from '../player-status';
import { Tray } from '../tray';
import { useState } from 'preact/hooks';
import { BlockText } from '../block-text';

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
  const [isTrayExpanded, setIsTrayExpanded] = useState(
    Object.keys(players).length <= 4
  );
  const leader = playerBids == null ? undefined : get_next_solver(playerBids);

  return (
    <div onClick={() => setIsTrayExpanded(!isTrayExpanded)}>
      <ThemedPanel>
        <FlexCenter column>
          <PanelTitle>Bids</PanelTitle>
          <Divider />
          {/* TODO: Show Leader even if not expanded. Adjust icon sizes (inline svg at 1em?) */}
          <RenderWhen when={!isTrayExpanded}>
            <BlockText>Click to expand</BlockText>
          </RenderWhen>
          <Tray expanded={isTrayExpanded}>
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
          </Tray>
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
