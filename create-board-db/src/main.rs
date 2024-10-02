use inertia_core::board_generators::ClassicBoardGenerator;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::Position;
use inertia_core::mechanics::PositionGenerator;
use inertia_core::solvers::difficulty::get_solution_difficulty;
use inertia_core::solvers::idas_nonrecursive::deepening_search_to_depth;
use inertia_core::solvers::SolutionStep;
use sqlx::Connection;
use sqlx::SqliteConnection;
use std::cmp::max;
use std::collections::HashMap;

const DB_URL: &str = "sqlite:db/boards.db";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  let mut conn = SqliteConnection::connect(DB_URL).await?;

  let mut counts = HashMap::new();
  let mut max_length = 0;
  let mut iter = 0;
  loop {
    if iter % 100 == 0 {
      println!("##### ITERATION {} #####", iter);
      println!("Counts: {:?}", counts);
      println!("Max length: {}", max_length);
    }

    let position = ClassicBoardGenerator::new().generate_position();
    let Position {
      walled_board,
      actor_squares,
      goal,
    } = position;
    let board = MoveBoard::from(&walled_board);

    let solution: Vec<SolutionStep> =
      deepening_search_to_depth(&board, goal, actor_squares, 45).unwrap();

    if solution.len() == 0 {
      continue;
    }

    let difficulty = get_solution_difficulty(&solution);

    let count = counts
      .entry(difficulty)
      .and_modify(|x| *x += 1)
      .or_insert(1);

    let length = solution.len();
    max_length = max(max_length, length);

    iter += 1;

    if *count > 500 {
      continue;
    }

    sqlx::query(
      "insert into boards (board, solution, difficulty) values (?, ?, ?)",
    )
    .bind(position.to_compressed_byte_array().as_slice())
    .bind(serde_json::to_string(&solution).unwrap())
    .bind(u8::from(difficulty))
    .execute(&mut conn)
    .await?;
  }
}
