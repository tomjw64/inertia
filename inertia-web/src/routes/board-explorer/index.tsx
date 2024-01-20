import {
  apply_solution,
  decode_position,
  decode_solution,
  encode_solution,
} from 'inertia-wasm';
import { Board } from '../../components/board';
import { FlexCenter } from '../../components/flex-center';
import { RoomControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ErrorPage } from '../../components/error-page';
import { Direction, SolutionStep } from 'inertia-core';
import { ThemedPanel } from '../../components/themed-panel';
import { Tray } from '../../components/tray';
import { BlockText } from '../../components/block-text';
import { PanelTitle } from '../../components/panel-title';
import { Divider } from '../../components/divider';
import { useEffect, useState } from 'preact/hooks';
import { ArrowCircleUp } from '../../components/svg/arrow-circle-up';
import { ArrowCircleDown } from '../../components/svg/arrow-circle-down';
import { ArrowCircleLeft } from '../../components/svg/arrow-circle-left';
import { ArrowCircleRight } from '../../components/svg/arrow-circle-right';
import { getActorColor } from '../../utils/actor-colors';
import { set } from 'lodash';
import { LineStart } from '../../components/svg/line-start';
import { FlagCircle } from '../../components/svg/flag-circle';
import {
  ThemedButton,
  ThemedFormLine,
  ThemedInput,
} from '../../components/themed-form';

// defaultBoard =AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_

// testUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_&solution=Optimal%20Solution:DAABAQEBAQE

type ExplorerSolution = { name: string; solution: SolutionStep[] };

const DIRECTION_TO_COMPONENT = {
  [Direction.Up]: <ArrowCircleUp />,
  [Direction.Down]: <ArrowCircleDown />,
  [Direction.Left]: <ArrowCircleLeft />,
  [Direction.Right]: <ArrowCircleRight />,
};

const ExpandableSolution = ({
  solution,
  expanded,
  selectedStep,
  onSelect,
  onSelectStep,
}: {
  solution: ExplorerSolution;
  expanded: boolean;
  selectedStep: number;
  onSelect: () => void;
  onSelectStep: (idx: number) => void;
}) => {
  return (
    <div>
      <ThemedFormLine>
        <ThemedInput value={solution.name} />
        <ThemedButton onClick={onSelect}>View</ThemedButton>
        <ThemedButton>Delete</ThemedButton>
      </ThemedFormLine>
      <div style={{ fontSize: '2em', maxWidth: '11em' }}>
        <Tray expanded={expanded} transformOrigin="top left">
          <FlexCenter wrap justify="start">
            <div
              style={{
                color: '#333333',
                display: 'flex',
                cursor: 'pointer',
                ...(selectedStep === -1
                  ? {
                      borderBottom: '2px solid #333333',
                    }
                  : {}),
              }}
              onClick={() => {
                onSelectStep(-1);
              }}
            >
              <FlagCircle />
            </div>
            {solution.solution.map((step, idx) => {
              return (
                <div
                  key={idx}
                  style={{
                    color: getActorColor(step.actor),
                    display: 'flex',
                    cursor: 'pointer',
                    ...(idx === selectedStep
                      ? {
                          borderBottom: '2px solid #333333',
                        }
                      : {}),
                  }}
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
  const urlParams = new URLSearchParams(window.location.search);
  const positionBytes = urlParams.get('position');

  const solutionsFromParams = urlParams.getAll('solution').flatMap((param) => {
    const nameAndSolution = param.split(':');
    const name = nameAndSolution[0];
    const solutionBytes = nameAndSolution[1];
    const solution = decode_solution(solutionBytes);
    return solution ? [{ name, solution }] : [];
  });

  const [solutions, setSolutions] = useState([
    ...solutionsFromParams,
    { name: 'My Solution', solution: [] },
  ]);
  const [activeSolutionIndex, setActiveSolutionIndex] = useState(
    solutions.length - 1
  );
  const [solutionStepIndex, setSolutionStepIndex] = useState(-1);

  // WIP
  // useEffect(() => {
  //   const currentState = window.history.state;
  //   const currentUrl = window.location.href;
  //   const newParams = new URLSearchParams();
  //   newParams.append('position', positionBytes);
  //   const newUrl = currentUrl.split('?')[0];
  //   console.log(currentState, currentUrl);
  //   window.history.replaceState(currentState, '');
  // }, [positionBytes, solutions]);

  if (!positionBytes) {
    return (
      <>
        <Starfield numStars={500} speed={0.5} />
        <ErrorPage>No board to show.</ErrorPage>
      </>
    );
  }

  const position = decode_position(positionBytes);
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
      <RoomControls />
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
          </FlexCenter>
        </ThemedPanel>
        <Board
          interactive
          walledBoard={position.walled_board}
          goal={position.goal}
          actorSquares={actorSquares}
          onMoveActor={onMoveActor}
        />
      </FlexCenter>
    </>
  );
};
