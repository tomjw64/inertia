use inertia_core::board_generators::EmptyMiddleGoalBoardGenerator;
use inertia_core::mechanics::BlockBoard;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::WalledBoardPosition;
use inertia_core::mechanics::WalledBoardPositionGenerator;
// use inertia_core::solvers::search_idas_indexing::deepening_search_to_depth;
use inertia_core::solvers::search_idas_indexing_moveboard::deepening_search_to_depth;
use inertia_core::solvers::SolutionStep;

fn main() {
  let position = EmptyMiddleGoalBoardGenerator::new().generate_position();
  let WalledBoardPosition {
    walled_board,
    actor_squares,
    goal,
  } = position;
  // let board = BlockBoard::from(&walled_board);
  let board = MoveBoard::from(&walled_board);

  let solution: Option<Vec<SolutionStep>> =
    deepening_search_to_depth(&board, goal, actor_squares, 45);
  dbg!(solution.map(|v| v.len()));
}
