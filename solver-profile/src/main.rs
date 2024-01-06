use inertia_core::board_generators::EmptyMiddleGoalBoardGenerator;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::Position;
use inertia_core::mechanics::PositionGenerator;
// use inertia_core::solvers::idas::deepening_search_to_depth;
use inertia_core::solvers::idas_nonrecursive::deepening_search_to_depth;
use inertia_core::solvers::SolutionStep;

fn main() {
  let position = EmptyMiddleGoalBoardGenerator::new().generate_position();
  let Position {
    walled_board,
    actor_squares,
    goal,
  } = position;
  let board = MoveBoard::from(&walled_board);

  let solution: Option<Vec<SolutionStep>> =
    deepening_search_to_depth(&board, goal, actor_squares, 45);
  dbg!(solution.map(|v| v.len()));
}
