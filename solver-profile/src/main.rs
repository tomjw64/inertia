use base64::engine::general_purpose;
use base64::Engine;
use inertia_core::board_generators::EmptyMiddleGoalBoardGenerator;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::Position;
use inertia_core::mechanics::PositionGenerator;
use inertia_core::solvers::astar;
use inertia_core::solvers::idas;
use inertia_core::solvers::CrawlAwareImprovedHeuristicBoard;
use inertia_core::solvers::SolutionStep;

fn do_encoded_board(position_b64: &str) {
  let bytes = general_purpose::URL_SAFE_NO_PAD
    .decode(position_b64)
    .unwrap();
  let position =
    Position::from_compressed_byte_array(&bytes[0..69].try_into().unwrap());
  let solution: Option<Vec<SolutionStep>> =
    astar::solve_position(position, 255);
  dbg!(solution.map(|v| v.len()));
}

fn do_empty_board() {
  let position = EmptyMiddleGoalBoardGenerator::new().generate_position();
  let solution: Option<Vec<SolutionStep>> = astar::solve_position(position, 45);
  dbg!(solution.map(|v| v.len()));
}

fn do_heuristic_board(position_b64: &str) {
  let bytes = general_purpose::URL_SAFE_NO_PAD
    .decode(position_b64)
    .unwrap();
  let position =
    Position::from_compressed_byte_array(&bytes[0..69].try_into().unwrap());
  let heuristic_board = CrawlAwareImprovedHeuristicBoard::from_move_board(
    &MoveBoard::from(&position.walled_board),
    position.goal,
  );
  dbg!(heuristic_board);
}

fn main() {
  // do_heuristic_board("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgP_");
  do_encoded_board("AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFQEAEBHu");
  // do_empty_board();
}
