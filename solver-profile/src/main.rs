use base64::engine::general_purpose;
use base64::Engine;
use inertia_core::board_generators::EmptyMiddleGoalBoardGenerator;
use inertia_core::mechanics::ActorSquares;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::Position;
use inertia_core::mechanics::PositionGenerator;
use inertia_core::mechanics::Square;
use inertia_core::solvers::astar;
use inertia_core::solvers::fixtures::GENERATED_WALLED_BOARD_15;
use inertia_core::solvers::get_min_num_actors_for_assist_value;
use inertia_core::solvers::idas;
use inertia_core::solvers::CrawlAwareImprovedHeuristicBoard;
use inertia_core::solvers::ExpensiveCrawlsBoard;
use inertia_core::solvers::MinAssistsBoard;
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
  let move_board = MoveBoard::from(&position.walled_board);
  let min_assists_board =
    MinAssistsBoard::from_move_board(&move_board, position.goal);
  dbg!(get_min_num_actors_for_assist_value(
    &move_board,
    &min_assists_board
  ));
  // let heuristic_board = ExpensiveCrawlsBoard::from_move_board(
  //   &MoveBoard::from(&position.walled_board),
  //   position.goal,
  // );

  // dbg!(heuristic_board);
}

fn main() {
  // empty
  // do_encoded_board("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgOI");
  // gauntlet
  // do_encoded_board("AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFQEAEBHu");
  // gauntletButWorse
  // do_encoded_board("AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFd8A_RHu");
  do_heuristic_board("AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFd8A_RHu");
  // gen15
  // do_encoded_board("AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFQEAEBHu");
  // do_empty_board();

  // do_heuristic_board("AAAAAAAAAAAAAAAAAAAkACAAJABJEggCSRJJEkEQSRJAEEASQAJAEgASQAIAEgASAAIAEgASAAIAEgASAAIAEgABAgPu");
}
