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
import { useState } from 'preact/hooks';
import { ArrowCircleUp } from '../../components/svg/arrow-circle-up';
import { ArrowCircleDown } from '../../components/svg/arrow-circle-down';
import { ArrowCircleLeft } from '../../components/svg/arrow-circle-left';
import { ArrowCircleRight } from '../../components/svg/arrow-circle-right';
import { getActorColor } from '../../utils/actor-colors';

// defaultBoard =AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_

// testUrl = http://inertia.localhost:8080/explore?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_&solution=my%20solution:DAABAQEBAQE

type ExplorerSolution = { name: string; solution: SolutionStep[] };

const DIRECTION_TO_COMPONENT = {
  [Direction.Up]: <ArrowCircleUp />,
  [Direction.Down]: <ArrowCircleDown />,
  [Direction.Left]: <ArrowCircleLeft />,
  [Direction.Right]: <ArrowCircleRight />,
};

const ExpandableSolution = ({ solution }: { solution: ExplorerSolution }) => {
  const [expanded, setExpanded] = useState(false);
  return (
    <div>
      <BlockText>
        <span style={{ marginRight: '1em', fontWeight: 'bold' }}>
          {solution.name}
        </span>
        <a onClick={() => setExpanded(!expanded)}>
          {expanded ? 'Hide' : 'Show'}
        </a>
      </BlockText>
      <div style={{ fontSize: '2em', maxWidth: '11em' }}>
        <Tray expanded={expanded} transformOrigin="top left">
          <FlexCenter wrap justify="start">
            {solution.solution.map((step) => {
              return (
                <div
                  style={{
                    color: getActorColor(step.actor),
                    display: 'flex',
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
  // const [activeSolutionIndex, setActiveSolutionIndex] = useState(0);
  const urlParams = new URLSearchParams(window.location.search);

  const [testSolution, setTestSolution] = useState<SolutionStep[]>([]);

  const positionBytes = urlParams.get('position');
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

  const solutions = urlParams.getAll('solution').flatMap((param) => {
    const nameAndSolution = param.split(':');
    const name = nameAndSolution[0];
    const solutionBytes = nameAndSolution[1];
    const solution = decode_solution(solutionBytes);
    return solution ? [{ name, solution }] : [];
  });

  solutions.push({ name: 'Test solution', solution: testSolution });

  console.log(encode_solution(testSolution));

  const actorSquares = apply_solution(position, testSolution);

  const onMoveActor = (step: SolutionStep) => {
    setTestSolution([...testSolution, step]);
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
              {solutions.map((solution) => {
                return <ExpandableSolution solution={solution} />;
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
