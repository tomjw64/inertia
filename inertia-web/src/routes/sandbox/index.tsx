import style from './style.module.scss';
import { Board } from '../../components/board';
import { Controls } from '../../components/controls';
import {
  ActorSquares,
  SolutionStep,
  WallGrid,
  WalledBoard,
} from 'inertia-core';
import { useEffect, useState } from 'preact/hooks';
import { get_movement_for_actor } from 'inertia-wasm';
import { Starfield } from '../../components/starfield';

export const Sandbox = () => {
  const vertical = [...Array(16)].map((_row) =>
    Array(15).fill(false)
  ) as WallGrid;
  const horizontal = [...Array(16)].map((_column) =>
    Array(15).fill(false)
  ) as WallGrid;

  const [walledBoard, setWalledBoard] = useState<WalledBoard>({
    vertical,
    horizontal,
    goal: 255,
  });

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

  const moveActor = (actorIndex: number, squareIndex: number) => {
    const newActorSquares = [...actorSquares] as ActorSquares;
    newActorSquares[actorIndex] = squareIndex;
    setActorSquares(newActorSquares);
  };

  const doSolutionStep = (step: SolutionStep) => {
    const destinationSquareIndex = get_movement_for_actor(
      {
        walled_board: walledBoard,
        actor_squares: actorSquares,
      },
      step.actor,
      step.direction
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
          actorSquares={actorSquares}
          moveActor={moveActor}
        />
        <Controls
          setWalledBoard={setWalledBoard}
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
