import style from './style.module.scss';
import { RoundStart as RoundStartState } from 'inertia-core';
import { CountdownPanel } from '../countdown-panel';
import { Starfield } from '../starfield';
import { Board } from '../board';

export const RoundStart = ({
  state,
  initialCountdownTimeLeft,
}: {
  state: RoundStartState;
  initialCountdownTimeLeft: number;
}) => {
  return (
    <div>
      <Starfield numStars={500} speed={2} />
      <div class={style.room}>
        <Board
          walledBoard={state.board.walled_board}
          goal={state.board.goal}
          actorSquares={state.board.actor_squares}
          moveActor={() => {}}
        />
        <CountdownPanel initialCountdownTimeLeft={initialCountdownTimeLeft} />
      </div>
    </div>
  );
};
