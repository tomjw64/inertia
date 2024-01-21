import {
  apply_solution,
  decode_position,
  decode_solution,
  encode_solution,
} from 'inertia-wasm';
import style from './style.module.scss';
import { ACTOR_FLIP_ANIMATE_DURATION, Board } from '../../components/board';
import { FlexCenter } from '../../components/flex-center';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ErrorPage } from '../../components/error-page';
import { Direction, SolutionStep } from 'inertia-core';
import { ThemedPanel } from '../../components/themed-panel';
import { Tray } from '../../components/tray';
import { PanelTitle } from '../../components/panel-title';
import { Divider } from '../../components/divider';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
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
import debounce from 'lodash/debounce';
import { useThrottledQueue } from '../../utils/throttled-queue';
import range from 'lodash/range';
import classnames from 'classnames';

// defaultBoard =AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_

// testUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_&solution=Optimal%20Solution:DAABAQEBAQE

type ExplorerSolution = { name: string; solution: SolutionStep[] };

const DIRECTION_TO_COMPONENT = {
  [Direction.Up]: <ArrowCircleUp />,
  [Direction.Down]: <ArrowCircleDown />,
  [Direction.Left]: <ArrowCircleLeft />,
  [Direction.Right]: <ArrowCircleRight />,
};

const DEFAULT_EMPTY_SOLUTION = { name: 'My solution', solution: [] };

const debouncedSetUrlParams = debounce((params: URLSearchParams) => {
  const currentState = window.history.state;
  const currentUrl = window.location.href;
  const newUrl = currentUrl.split('?')[0] + '?' + params.toString();
  window.history.replaceState(currentState, '', newUrl);
}, 200);

const ExpandableSolution = ({
  solution,
  expanded,
  selectedStep,
  onDelete,
  onSelect,
  onSelectStep,
  onChangeName,
}: {
  solution: ExplorerSolution;
  expanded: boolean;
  selectedStep: number;
  onDelete: () => void;
  onSelect: () => void;
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
        <ThemedButton onClick={onSelect}>View</ThemedButton>
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
                    }
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
  const originalUrlParams = useRef<URLSearchParams>(
    new URLSearchParams(window.location.search)
  );
  const originalUrlPositionBytes = useMemo(
    () => originalUrlParams.current.get('position'),
    []
  );
  const originalUrlSolutions = useMemo(
    () =>
      originalUrlParams.current.getAll('solution').flatMap((param) => {
        const nameAndSolution = param.split(':');
        const name = nameAndSolution[0];
        const solutionBytes = nameAndSolution[1];
        const solution = decode_solution(solutionBytes);
        return solution ? [{ name, solution }] : [];
      }),
    []
  );
  const originalUrlHasEmptyLastSolution = useMemo(
    () =>
      originalUrlSolutions.length > 0 &&
      originalUrlSolutions[originalUrlSolutions.length - 1].solution.length ===
        0,
    [originalUrlSolutions]
  );

  const [solutions, setSolutions] = useState([
    ...originalUrlSolutions,
    ...(originalUrlHasEmptyLastSolution ? [] : [DEFAULT_EMPTY_SOLUTION]),
  ]);
  const [activeSolutionIndex, setActiveSolutionIndex] = useState(
    solutions.length - 1
  );
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

  useEffect(() => {
    const urlParams = new URLSearchParams();
    if (originalUrlPositionBytes) {
      urlParams.append('position', originalUrlPositionBytes);
    }
    for (const solution of solutions) {
      urlParams.append(
        'solution',
        `${solution.name}:${encode_solution(solution.solution)}`
      );
    }
    debouncedSetUrlParams(urlParams);
  }, [originalUrlPositionBytes, solutions]);

  if (!originalUrlPositionBytes) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>No board to show.</ErrorPage>
      </>
    );
  }

  const position = decode_position(originalUrlPositionBytes);
  if (!position) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>Could not parse position from url.</ErrorPage>
      </>
    );
  }

  const activeSolution = solutions[activeSolutionIndex];
  const appliedSolution = activeSolution.solution.slice(
    0,
    solutionStepIndex + 1
  );

  const actorSquares = apply_solution(position, appliedSolution);

  const onMoveActor = (step: SolutionStep) => {
    setSolutionStepIndex((prev) => prev + 1);
    setSolutions(
      solutions.toSpliced(activeSolutionIndex, 1, {
        name: activeSolution.name,
        solution: [...appliedSolution, step],
      })
    );
  };

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      <FlexCenter wrap>
        <ThemedPanel>
          <FlexCenter column>
            <PanelTitle>Board Explorer</PanelTitle>
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
                        })
                      );
                    }}
                    onDelete={() => {
                      const remainingSolutions = solutions.toSpliced(idx, 1);
                      setSolutions(
                        remainingSolutions.length > 0
                          ? remainingSolutions
                          : [DEFAULT_EMPTY_SOLUTION]
                      );
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
                  />
                );
              })}
            </FlexCenter>
            <Divider />
            <FlexCenter>
              <ThemedButton
                onClick={() => {
                  setSolutions([...solutions, DEFAULT_EMPTY_SOLUTION]);
                }}
              >
                New Solution
              </ThemedButton>
              <ThemedFormLine>
                <ThemedButton
                  onClick={() => {
                    setSolutionStepIndex((last) => Math.max(-1, last - 1));
                  }}
                >
                  &lt;
                </ThemedButton>
                <ThemedButton
                  onClick={() => {
                    setSolutionStepIndex((last) =>
                      Math.min(activeSolution.solution.length - 1, last + 1)
                    );
                  }}
                >
                  &gt;
                </ThemedButton>
              </ThemedFormLine>
              <ThemedButton
                onClick={() => {
                  setAnimationQueue(range(-1, activeSolution.solution.length));
                  processAnimationQueue();
                }}
              >
                Animate
              </ThemedButton>
            </FlexCenter>
          </FlexCenter>
        </ThemedPanel>
        <Board
          interactive={!isAnimating}
          walledBoard={position.walled_board}
          goal={position.goal}
          actorSquares={actorSquares}
          onMoveActor={onMoveActor}
        />
      </FlexCenter>
    </>
  );
};
