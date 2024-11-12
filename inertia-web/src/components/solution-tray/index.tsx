import { Solution } from 'inertia-core';
import classnames from 'classnames';
import { FlexCenter } from '../flex-center';
import { Tray } from '../tray';
import style from './style.module.scss';
import { FlagCircle } from '../svg/flag-circle';
import { getActorColor } from '../../utils/actor-colors';
import { ArrowCircleUp } from '../svg/arrow-circle-up';
import { DIRECTIONS } from '../../utils/direction';
import { ArrowCircleDown } from '../svg/arrow-circle-down';
import { ArrowCircleLeft } from '../svg/arrow-circle-left';
import { ArrowCircleRight } from '../svg/arrow-circle-right';

const DIRECTION_TO_COMPONENT = {
  [DIRECTIONS.Up]: <ArrowCircleUp />,
  [DIRECTIONS.Down]: <ArrowCircleDown />,
  [DIRECTIONS.Left]: <ArrowCircleLeft />,
  [DIRECTIONS.Right]: <ArrowCircleRight />,
};

export const SolutionTray = ({
  solution,
  expanded,
  selectedStep,
  onSelectStep,
}: {
  solution: Solution;
  expanded: boolean;
  selectedStep?: number;
  onSelectStep?: (idx: number) => void;
}) => {
  return (
    <div className={style.solutionTrayContainer}>
      <Tray inset expanded={expanded} transformOrigin="top left">
        <FlexCenter wrap justify="start">
          <div
            className={classnames(style.stepIcon, style.neutral, {
              [style.selected]: selectedStep === -1,
            })}
            onClick={() => {
              onSelectStep && onSelectStep(-1);
            }}
          >
            <FlagCircle />
          </div>
          {solution.map((step, idx) => {
            return (
              <div
                className={classnames(
                  style.stepIcon,
                  style[getActorColor(step.actor)],
                  {
                    [style.selected]: selectedStep === idx,
                    [style.selectable]: !!onSelectStep,
                  },
                )}
                key={idx}
                onClick={() => {
                  onSelectStep && onSelectStep(idx);
                }}
              >
                {DIRECTION_TO_COMPONENT[step.direction]}
              </div>
            );
          })}
        </FlexCenter>
      </Tray>
    </div>
  );
};
