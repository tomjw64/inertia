import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { FlexCenter } from '../../components/flex-center';
import { PlayableBoard } from '../../components/playable-board';
import { AppControls } from '../../components/room-controls';
import { Starfield } from '../../components/starfield';
import { ThemedPanel } from '../../components/themed-panel';
import { defaultPosition } from '../../utils/board';
import { getBackendUrl } from '../../utils/backend';
import {
  apply_solution,
  CheckSolutionResult,
  decode_position,
  encode_solution,
  is_solution,
  Solution,
  SolutionStep,
} from 'inertia-core';
import { RenderWhen } from '../../components/utils/RenderWhen';
import { ErrorPage } from '../../components/error-page';
import { PanelTitle } from '../../components/panel-title';
import { Divider } from '../../components/divider';
import { ThemedButton } from '../../components/themed-form';
import { Tray } from '../../components/tray';
import { CHECK_SOLUTION_RESULTS } from '../../utils/check-solution-result';
import { mmssccFormat } from '../../utils/time';
import { useStopwatch } from '../../utils/hooks/use-stopwatch';
import style from './style.module.scss';
import { SolutionTray } from '../../components/solution-tray';
import { FullWidth } from '../../components/full-width';
import { Timer } from '../../components/timer';
import { usePopup } from '../../utils/popup';
import { Popup } from '../../components/popup';
import { shake } from '../../animations/shake';
import { BlockText } from '../../components/block-text';

type SolveResult = {
  time: number;
  outcome: CheckSolutionResult;
};

const RED_CIRCLE = '\u{1F534}';
const YELLOW_CIRCLE = '\u{1F7E1}';
const GREEN_CIRCLE = '\u{1F7E2}';
const QUESTION_MARK = '\u{2753}';
const STAR = '\u{2B50}';
const ROCKET = '\u{1F680}';
const ASTRONAUT = '\u{1F9D1}\u{200D}\u{1F680}';

const CHECK_SOLUTION_RESULT_TO_EMOJI = {
  [CHECK_SOLUTION_RESULTS.NotASolution]: RED_CIRCLE,
  [CHECK_SOLUTION_RESULTS.InferiorSolution]: YELLOW_CIRCLE,
  [CHECK_SOLUTION_RESULTS.ComparableSolution]: GREEN_CIRCLE,
  [CHECK_SOLUTION_RESULTS.SuperiorSolution]: QUESTION_MARK,
};

const resultToShareableLines = (result: SolveResult) => {
  return `${mmssccFormat(result.time)} - ${CHECK_SOLUTION_RESULT_TO_EMOJI[result.outcome]}`;
};

export const Daily = () => {
  const [fetchError, setFetchError] = useState(false);
  const [date, setDate] = useState('');
  const [initialPosition, setInitialPosition] = useState(defaultPosition());
  const [solution, setSolution] = useState<Solution>([]);
  const [results, setResults] = useState<SolveResult[]>([]);
  const [isOptimalSolutionFound, setIsOptimalSolutionFound] = useState(false);
  const { timeMillis } = useStopwatch({ paused: isOptimalSolutionFound });

  const [isLegendExpanded, setIsLegendExpanded] = useState(false);
  const legendExpandIcon = isLegendExpanded
    ? '/contract-arrow.svg'
    : '/expand-arrow.svg';

  const sharePopupElement = useRef<HTMLDivElement>(null);
  const popup = usePopup();

  const resetButton = useRef<HTMLDivElement>(null);
  const emphasizeReset = () => shake(resetButton.current);
  const shareButton = useRef<HTMLDivElement>(null);
  const emphasizeShare = () => shake(shareButton.current);

  const actorSquares = apply_solution(initialPosition, solution);
  const isSolution = is_solution(initialPosition, solution);
  const shareableResultLines = [
    `${ROCKET}${ASTRONAUT} Inertia Daily ${ASTRONAUT}${ROCKET}`,
    `${STAR} ${date} ${STAR}`,
    ...results.map(resultToShareableLines),
  ];

  const position = useMemo(
    () => ({
      ...initialPosition,
      actor_squares: actorSquares,
    }),
    [actorSquares, initialPosition],
  );

  useEffect(() => {
    let interval: ReturnType<typeof setInterval> | undefined;
    if (isSolution) {
      const emphasis = isOptimalSolutionFound ? emphasizeShare : emphasizeReset;
      interval = setInterval(emphasis, 2000);
    }
    return () => clearInterval(interval);
  }, [isOptimalSolutionFound, isSolution]);

  useEffect(() => {
    const getDailyPosition = async () => {
      const response = await fetch(getBackendUrl('daily'));
      const data = await response.json();
      const decodedResponsePosition = decode_position(data.position);
      if (!decodedResponsePosition) {
        setFetchError(true);
        return;
      }
      setDate(data.date);
      setInitialPosition(decodedResponsePosition);
    };
    getDailyPosition().catch((err) => {
      console.error(err);
      setFetchError(true);
    });
  }, []);

  const onMoveActor = (step: SolutionStep) => {
    const updatedSolution = [...solution, step];
    if (is_solution(initialPosition, updatedSolution)) {
      const checkSolution = async () => {
        const response = await fetch(getBackendUrl('check-daily'), {
          method: 'PUT',
          body: encode_solution(updatedSolution),
        });
        const data = await response.json();
        const result = data.result as CheckSolutionResult;
        if (result === CHECK_SOLUTION_RESULTS.ComparableSolution) {
          setIsOptimalSolutionFound(true);
        }
        setResults((currentResults) => [
          ...currentResults,
          {
            time: timeMillis.peek(),
            outcome: result,
          },
        ]);
      };
      checkSolution().catch((err) => console.log(err));
    }
    setSolution(updatedSolution);
  };

  const handleReset = () => {
    if (!isSolution) {
      setResults([
        ...results,
        {
          time: timeMillis.peek(),
          outcome: CHECK_SOLUTION_RESULTS.NotASolution,
        },
      ]);
    }
    setSolution([]);
  };

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      <RenderWhen when={fetchError}>
        <ErrorPage>Server error. Try again later.</ErrorPage>;
      </RenderWhen>
      <RenderWhen when={!fetchError}>
        <FlexCenter wrap>
          <ThemedPanel>
            <FlexCenter column>
              <PanelTitle>Daily Puzzle - {date} </PanelTitle>
              <Divider text="Timer" />
              <Timer time={timeMillis} paused={isOptimalSolutionFound} />
              <Divider text="Solution" />
              <FullWidth>
                <SolutionTray solution={solution} expanded />
              </FullWidth>
              <RenderWhen when={results.length > 0}>
                <Divider text="Results" />
                <Tray expanded inset>
                  {shareableResultLines.map((result) => (
                    <p className={style.shareableResultLine} key={result}>
                      {result}
                    </p>
                  ))}
                </Tray>
                <FlexCenter column>
                  <ThemedButton
                    onClick={() => setIsLegendExpanded(!isLegendExpanded)}
                  >
                    Legend
                    <img src={legendExpandIcon} />
                  </ThemedButton>
                  <Tray inset expanded={isLegendExpanded}>
                    <BlockText>{RED_CIRCLE}: No solution found</BlockText>
                    <BlockText>{YELLOW_CIRCLE}: Solution found</BlockText>
                    <BlockText>{GREEN_CIRCLE}: Best solution found</BlockText>
                  </Tray>
                </FlexCenter>
              </RenderWhen>
              <Divider />
              <FlexCenter>
                <div ref={resetButton}>
                  <ThemedButton
                    disabled={solution.length === 0 || isOptimalSolutionFound}
                    onClick={handleReset}
                  >
                    Reset
                  </ThemedButton>
                </div>
                <div>
                  <div ref={shareButton}>
                    <ThemedButton
                      disabled={results.length === 0}
                      onClick={() => {
                        navigator.clipboard
                          .writeText(shareableResultLines.join('\n'))
                          .then(() => popup(sharePopupElement.current))
                          .catch((e) => {
                            throw new Error(
                              'Failed to write to clipboard: ',
                              e,
                            );
                          });
                      }}
                    >
                      Share Results
                    </ThemedButton>
                  </div>
                  <Popup ref={sharePopupElement}>
                    Results copied to clipboard!
                  </Popup>
                </div>
              </FlexCenter>
            </FlexCenter>
          </ThemedPanel>
          <PlayableBoard
            position={position}
            interactive={!isSolution}
            onMoveActor={onMoveActor}
          />
        </FlexCenter>
      </RenderWhen>
    </>
  );
};
