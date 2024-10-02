import {
  apply_solution,
  Direction,
  ExpandedBitBoard,
  get_movement_for_actor,
  get_movement_ray_for_actor,
  Position,
  Square,
} from 'inertia-core';
import style from './style.module.scss';
import { ACTOR_FLIP_ANIMATE_DURATION } from '../../components/board';
import { FlexCenter } from '../../components/flex-center';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ErrorPage } from '../../components/error-page';
import { ThemedPanel } from '../../components/themed-panel';
import { Tray } from '../../components/tray';
import { PanelTitle } from '../../components/panel-title';
import { Divider } from '../../components/divider';
import { useMemo, useState } from 'preact/hooks';
import { ArrowCircleUp } from '../../components/svg/arrow-circle-up';
import { ArrowCircleDown } from '../../components/svg/arrow-circle-down';
import { ArrowCircleLeft } from '../../components/svg/arrow-circle-left';
import { ArrowCircleRight } from '../../components/svg/arrow-circle-right';
import { getActorColor } from '../../utils/actor-colors';
import { FlagCircle } from '../../components/svg/flag-circle';
import {
  ThemedButton,
  ThemedFormLine,
  ThemedInput,
} from '../../components/themed-form';
import { useThrottledQueue } from '../../utils/throttled-queue';
import { range } from 'lodash';
import classnames from 'classnames';
import {
  SimpleBoard,
  SquareMouseEvent,
  SquareRegionType,
} from '../../components/simple-board';
import {
  useInitialUrlPositions,
  useUrlSyncedSolutionsState,
} from '../../utils/url-params';
import { NamedSolution } from '../../types';
import { DIRECTIONS } from '../../constants/direction';
import { RenderWhen } from '../../components/utils/RenderWhen';
import { BoardSelection, useClickAwayDeselect } from '../../utils/selection';
import { fromSquares, union } from '../../utils/bitboard';

// defaultBoard = AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_

// testUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_&solution=Optimal%20Solution:DAABAQEBAQE
// shufflePuzzleUrl = http://inertia.localhost:8080/explore?position=_38AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA_3__fwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD_fxESISKI&solution=Optimal%20Solution:RgA0JdHL6vV0gJ_eJjm4TFMS6vV0gJ_eJjm4TFMS6vV0gJ_exg
// benchGen15Url = http://inertia.localhost:8080/explore?position=QBAIAQAAAABQAAAAADBAAUABAkBABAAAQAAAAAAgBBACAQACAAAAAAEAEAAIDEABQgEAAAACAAAgAAAgQAEQBCVsOTK4&solution=Optimal%3ADwCfZdTfbux1BA
// benchUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgOI&solution=Optimal%20Solution:KQC9NyxW8-Si8S1wNWF0JTBhdPU87Qk
// theXUrl = http://inertia.localhost:8080/explore?position=_B_4D_AH4APAAYAAAAAAAAAAAACAAMAB4APwB_gP_B_8H_gP8AfgA8ABgAAAAAAAAAAAAIAAwAHgA_AH-A_8H2QDBFPh&solution=Optimal%3ARgDfdTyftR7kncvwWY7k-ZKDYNF6TFIT--VkcJF0ZXhpIKBZGg
// gauntletUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFQEAEBHu&solution=Solution%3ASQAfDryp0QPPE5KbGgPfzxOSmxoD388TkpsaD5OKG9Kf2jvp_RoD&solution=My+solution%3AAAA
// gauntletButWorseUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFd8A_RHu&solution=Optimal%3APwCLAooCr4IKHgOb2t-eE4sDm9rfntPovDHxIMo76f0aAw
// 30 seconds!?!? ^^^
// gauntletButWorseWithGuardrailsUrl = http://inertia.localhost:8080/explore?position=CAAgACAAgQCBAAUCBQIVCBUIVSBVIFUBVQFVBVUFVRUIACAAIACBAIEABQIFAhUIFQhVIFUgVQFVAVUFVQVVFd8A_RHu&solution=Optimal%3AJgAhx1bWz05H385W1s9O31doSptX

const DIRECTION_TO_COMPONENT = {
  [DIRECTIONS.Up]: <ArrowCircleUp />,
  [DIRECTIONS.Down]: <ArrowCircleDown />,
  [DIRECTIONS.Left]: <ArrowCircleLeft />,
  [DIRECTIONS.Right]: <ArrowCircleRight />,
};

const NEW_SOLUTION_NAME = 'New solution';
const NEW_SOLUTION = { name: NEW_SOLUTION_NAME, solution: [] };

const ExpandableSolution = ({
  solution,
  expanded,
  selectedStep,
  onDelete,
  onSelect,
  onHide,
  onSelectStep,
  onChangeName,
}: {
  solution: NamedSolution;
  expanded: boolean;
  selectedStep: number;
  onDelete: () => void;
  onSelect: () => void;
  onHide: () => void;
  onSelectStep: (idx: number) => void;
  onChangeName: (name: string) => void;
}) => {
  return (
    <div>
      <ThemedFormLine>
        <ThemedInput
          onInput={(e) => {
            onChangeName(e.currentTarget.value);
          }}
          value={solution.name}
        />
        <RenderWhen when={expanded}>
          <ThemedButton onClick={onHide}>Hide</ThemedButton>
        </RenderWhen>
        <RenderWhen when={!expanded}>
          <ThemedButton onClick={onSelect}>View</ThemedButton>
        </RenderWhen>
        <ThemedButton onClick={onDelete}>Delete</ThemedButton>
      </ThemedFormLine>
      <div className={style.solutionTrayContainer}>
        <Tray inset expanded={expanded} transformOrigin="top left">
          <FlexCenter wrap justify="start">
            <div
              className={classnames(style.stepIcon, style.neutral, {
                [style.selected]: selectedStep === -1,
              })}
              onClick={() => {
                onSelectStep(-1);
              }}
            >
              <FlagCircle />
            </div>
            {solution.solution.map((step, idx) => {
              return (
                <div
                  className={classnames(
                    style.stepIcon,
                    style[getActorColor(step.actor)],
                    {
                      [style.selected]: selectedStep === idx,
                    },
                  )}
                  key={idx}
                  onClick={() => {
                    onSelectStep(idx);
                  }}
                >
                  {DIRECTION_TO_COMPONENT[step.direction]}
                </div>
              );
            })}
          </FlexCenter>
        </Tray>
      </div>
    </div>
  );
};

export const BoardExplorer = () => {
  const positions = useInitialUrlPositions();
  const position = positions[0]?.position;

  if (!position) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>Could not parse position from url.</ErrorPage>
      </>
    );
  }

  return <NonEmptyBoardExplorer position={position} />;
};

const NonEmptyBoardExplorer = ({ position }: { position: Position }) => {
  const [solutions, setSolutions] = useUrlSyncedSolutionsState();
  const [activeSolutionIndex, setActiveSolutionIndex] = useState(-1);
  const [solutionStepIndex, setSolutionStepIndex] = useState(-1);

  const [selection, setSelection] = useState(BoardSelection.NONE);
  useClickAwayDeselect(setSelection);

  const {
    processQueue: processAnimationQueue,
    setQueue: setAnimationQueue,
    isProcessing: isAnimating,
  } = useThrottledQueue<number>({
    throttleMs: (ACTOR_FLIP_ANIMATE_DURATION + 0.1) * 1000,
    onData: (data) => {
      setSolutionStepIndex(data);
    },
  });

  const activeSolution = solutions[activeSolutionIndex];
  const appliedSolution = activeSolution?.solution?.slice(
    0,
    solutionStepIndex + 1,
  );

  const actorSquares = appliedSolution
    ? apply_solution(position, appliedSolution)
    : position.actor_squares;

  const movementRaySquares = useMemo(() => {
    return Object.values(DIRECTIONS)
      .map((direction) => {
        return {
          [direction]: get_movement_ray_for_actor(
            {
              ...position,
              actor_squares: actorSquares,
            },
            selection,
            direction,
          ),
        } as Record<Direction, ExpandedBitBoard>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [position, actorSquares, selection]);
  const indicatorSquares = useMemo(
    () => union(Object.values(movementRaySquares)),
    [movementRaySquares],
  );

  const movementSquares = useMemo(() => {
    return Object.values(DIRECTIONS)
      .map((direction) => {
        return {
          [direction]: get_movement_for_actor(
            {
              ...position,
              actor_squares: actorSquares,
            },
            selection,
            direction,
          ),
        } as Record<Direction, Square>;
      })
      .reduce((prev, acc) => ({ ...acc, ...prev }));
  }, [position, actorSquares, selection]);
  const emphasizedIndicatorSquares = useMemo(
    () => fromSquares(Object.values(movementSquares)),
    [movementSquares],
  );

  const handleClickRegion = (event: SquareMouseEvent) => {
    const { squareIndex, region } = event;

    const selectingActorIndex = position.actor_squares.indexOf(squareIndex);
    if (region === SquareRegionType.CENTER && selectingActorIndex !== -1) {
      setSelection(
        selection === selectingActorIndex
          ? BoardSelection.NONE
          : selectingActorIndex,
      );
      return;
    }

    for (const direction of Object.values(DIRECTIONS)) {
      if (movementRaySquares[direction][squareIndex]) {
        const step = {
          actor: selection,
          direction: direction,
        };
        setSolutionStepIndex((prev) => prev + 1);
        setSolutions((solutions) => {
          if (!activeSolution) {
            return [...solutions, { name: 'New solution', solution: [step] }];
          }
          solutions.toSpliced(activeSolutionIndex, 1, {
            name: activeSolution.name,
            solution: appliedSolution ? [...appliedSolution, step] : [step],
          });
          return solutions;
        });
      }
    }
  };

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      <FlexCenter wrap>
        <ThemedPanel>
          <FlexCenter column>
            <PanelTitle>Board Explorer</PanelTitle>
            <RenderWhen when={solutions.length > 0}>
              <Divider />

              <FlexCenter column align="flex-start">
                {solutions.map((solution, idx) => {
                  return (
                    <ExpandableSolution
                      key={idx}
                      solution={solution}
                      expanded={activeSolutionIndex === idx}
                      selectedStep={solutionStepIndex}
                      onChangeName={(name) => {
                        setSolutions(
                          solutions.toSpliced(idx, 1, {
                            name,
                            solution: solution.solution,
                          }),
                        );
                      }}
                      onDelete={() => {
                        const remainingSolutions = solutions.toSpliced(idx, 1);
                        setSolutions(remainingSolutions);
                        const adjustedIndex =
                          activeSolutionIndex < idx
                            ? activeSolutionIndex
                            : activeSolutionIndex - 1;
                        setActiveSolutionIndex(Math.max(0, adjustedIndex));
                      }}
                      onSelectStep={(idx) => {
                        setSolutionStepIndex(idx);
                      }}
                      onSelect={() => {
                        setActiveSolutionIndex(idx);
                        setSolutionStepIndex(-1);
                      }}
                      onHide={() => {
                        setActiveSolutionIndex(-1);
                        setSolutionStepIndex(-1);
                      }}
                    />
                  );
                })}
              </FlexCenter>
            </RenderWhen>
            <Divider />
            <FlexCenter>
              <ThemedButton
                onClick={() => {
                  setSolutions([...solutions, NEW_SOLUTION]);
                }}
              >
                New Solution
              </ThemedButton>
              <ThemedFormLine>
                <ThemedButton
                  disabled={!activeSolution}
                  onClick={() => {
                    setSolutionStepIndex((last) => Math.max(-1, last - 1));
                  }}
                >
                  &lt;
                </ThemedButton>
                <ThemedButton
                  disabled={!activeSolution}
                  onClick={() => {
                    setSolutionStepIndex((last) =>
                      Math.min(activeSolution!.solution.length - 1, last + 1),
                    );
                  }}
                >
                  &gt;
                </ThemedButton>
              </ThemedFormLine>
              <ThemedButton
                disabled={!activeSolution}
                onClick={() => {
                  setAnimationQueue(range(-1, activeSolution!.solution.length));
                  processAnimationQueue();
                }}
              >
                Animate
              </ThemedButton>
            </FlexCenter>
          </FlexCenter>
        </ThemedPanel>
        <SimpleBoard
          walledBoard={position.walled_board}
          goal={position.goal}
          actorSquares={actorSquares}
          selection={selection}
          onClickRegion={handleClickRegion}
          indicatorSquares={indicatorSquares}
          emphasizedIndicatorSquares={emphasizedIndicatorSquares}
        />
      </FlexCenter>
    </>
  );
};
