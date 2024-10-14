import classnames from 'classnames';
import { apply_solution, Position, SolutionStep } from 'inertia-core';
import { range } from 'lodash';
import { useMemo, useState } from 'preact/hooks';
import { Divider } from '../../components/divider';
import { ErrorPage } from '../../components/error-page';
import { FlexCenter } from '../../components/flex-center';
import { PanelTitle } from '../../components/panel-title';
import { PlayableBoard } from '../../components/playable-board';
import { AppControls } from '../../components/room-controls';
import { ACTOR_FLIP_ANIMATE_DURATION } from '../../components/simple-board';
import { Starfield } from '../../components/starfield';
import { ArrowCircleDown } from '../../components/svg/arrow-circle-down';
import { ArrowCircleLeft } from '../../components/svg/arrow-circle-left';
import { ArrowCircleRight } from '../../components/svg/arrow-circle-right';
import { ArrowCircleUp } from '../../components/svg/arrow-circle-up';
import { FlagCircle } from '../../components/svg/flag-circle';
import {
  ThemedButton,
  ThemedFormLine,
  ThemedInput,
} from '../../components/themed-form';
import { ThemedPanel } from '../../components/themed-panel';
import { Tray } from '../../components/tray';
import { RenderWhen } from '../../components/utils/RenderWhen';
import { DIRECTIONS } from '../../constants/direction';
import { NamedSolution } from '../../types';
import { getActorColor } from '../../utils/actor-colors';
import { useThrottledQueue } from '../../utils/hooks/use-throttled-queue';
import {
  useInitialUrlPositions,
  useUrlSyncedSolutionsState,
} from '../../utils/url-params';
import style from './style.module.scss';

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

  return <NonEmptyBoardExplorer initialPosition={position} />;
};

const NonEmptyBoardExplorer = ({
  initialPosition,
}: {
  initialPosition: Position;
}) => {
  const [solutions, setSolutions] = useUrlSyncedSolutionsState();
  const [activeSolutionIndex, setActiveSolutionIndex] = useState(-1);
  const [solutionStepIndex, setSolutionStepIndex] = useState(-1);

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
    ? apply_solution(initialPosition, appliedSolution)
    : initialPosition.actor_squares;

  const position = useMemo(
    () => ({
      walled_board: initialPosition.walled_board,
      goal: initialPosition.goal,
      actor_squares: actorSquares,
    }),
    [actorSquares, initialPosition.goal, initialPosition.walled_board],
  );

  const onMoveActor = (step: SolutionStep) => {
    setSolutionStepIndex((prev) => prev + 1);
    setSolutions((solutions) => {
      if (!activeSolution) {
        return [...solutions, { name: 'New solution', solution: [step] }];
      }
      return solutions.toSpliced(activeSolutionIndex, 1, {
        name: activeSolution.name,
        solution: appliedSolution ? [...appliedSolution, step] : [step],
      });
    });
    setActiveSolutionIndex((activeSolutionIndex) => {
      if (!activeSolution) {
        return solutions.length;
      }
      return activeSolutionIndex;
    });
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
                        setActiveSolutionIndex((activeSolutionIndex) => {
                          if (
                            activeSolutionIndex === idx ||
                            activeSolutionIndex === -1
                          ) {
                            return -1;
                          }
                          return Math.max(0, adjustedIndex);
                        });
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
                  setActiveSolutionIndex(solutions.length);
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
        <PlayableBoard
          position={position}
          interactive={!isAnimating}
          onMoveActor={onMoveActor}
        />
      </FlexCenter>
    </>
  );
};
