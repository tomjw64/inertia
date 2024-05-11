import style from './style.module.scss';
import { Board } from '../../components/board';
import { Controls } from '../../components/controls';
import { ActorSquares, SolutionStep, Square, WalledBoard } from 'inertia-core';
import { useEffect, useState } from 'preact/hooks';
import { get_movement_for_actor } from 'inertia-wasm';
import { Starfield } from '../../components/starfield';
import { emptyBoard } from '../../utils/board';

export const Sandbox = () => {
  const [goal, setGoal] = useState<Square>(255);

  const [walledBoard, setWalledBoard] = useState<WalledBoard>(emptyBoard());

  const [initialActorSquares, setInitialActorSquares] = useState<ActorSquares>([
    0, 1, 2, 3,
  ]);
  const [actorSquares, setActorSquares] =
    useState<ActorSquares>(initialActorSquares);

  const [simulation, setSimulation] = useState<Array<SolutionStep>>([]);

  const [numStars, setNumStars] = useState<number>(500);
  const [starSpeed, setStarSpeed] = useState<number>(2);

  if (simulation.length) {
    setTimeout(() => {
      doSolutionStep(simulation.shift()!);
    }, 300);
  }

  useEffect(() => {
    setActorSquares(initialActorSquares);
  }, [initialActorSquares]);

  const onMoveActor = ({ actor, direction }: SolutionStep) => {
    const destinationSquareIndex = get_movement_for_actor(
      {
        walled_board: walledBoard,
        actor_squares: actorSquares,
        goal,
      },
      actor,
      direction,
    );
    const newActorSquares = [...actorSquares] as ActorSquares;
    newActorSquares[actor] = destinationSquareIndex;
    setActorSquares(newActorSquares);
  };

  const doSolutionStep = (step: SolutionStep) => {
    const destinationSquareIndex = get_movement_for_actor(
      {
        walled_board: walledBoard,
        actor_squares: actorSquares,
        goal,
      },
      step.actor,
      step.direction,
    );
    const newActorSquares = [...actorSquares] as ActorSquares;
    newActorSquares[step.actor] = destinationSquareIndex;
    setActorSquares(newActorSquares);
  };

  const simulateMoveSequence = async (sequence: Array<SolutionStep>) => {
    setSimulation(sequence);
  };

  return (
    <>
      <Starfield numStars={numStars} speed={starSpeed} />
      <div class={style.room}>
        <Board
          walledBoard={walledBoard}
          goal={goal}
          actorSquares={actorSquares}
          onMoveActor={onMoveActor}
          interactive
        />
        <Controls
          setWalledBoard={setWalledBoard}
          setGoal={setGoal}
          setInitialActorSquares={setInitialActorSquares}
          initialActorSquares={initialActorSquares}
          setActorSquares={setActorSquares}
          simulateMoveSequence={simulateMoveSequence}
          numStars={numStars}
          setNumStars={setNumStars}
          starSpeed={starSpeed}
          setStarSpeed={setStarSpeed}
        />
      </div>
    </>
  );
};
