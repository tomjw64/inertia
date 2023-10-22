import style from './style.module.scss';
import { RoundSummary as RoundSummaryState } from 'inertia-core';
import { PlayerPanel } from '../player-panel';
import { Starfield } from '../starfield';

export const RoundSummary = ({
  state,
  onStartGame,
}: {
  state: RoundSummaryState;
  onStartGame: () => void;
}) => {
  return (
    <div>
      <Starfield numStars={500} speed={2} />
      <div class={style.room}>
        <PlayerPanel players={state.meta.player_info} />
        <button onClick={onStartGame}>Start Game</button>
      </div>
    </div>
  );
};
