use base64::engine::general_purpose;
use base64::Engine;
use inertia_core::board_generators::ClassicBoardGenerator;
use inertia_core::mechanics::PositionGenerator;
use inertia_core::solvers::astar::solve_position;
use inertia_core::solvers::get_solution_internal_difficulty;
use inertia_core::solvers::Difficulty;
use inertia_core::solvers::SolutionStep;
use sqlx::Connection;
use sqlx::SqliteConnection;
use std::ops::DerefMut;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

const DB_URL: &str = "sqlite:db/positions.db";

const BATCH_SIZE: usize = 1000;
const POSITIONS_PER_INTERNAL_DIFFICULTY: usize = 500;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  let conn = Arc::new(Mutex::new(SqliteConnection::connect(DB_URL).await?));

  let internal_difficulty_counts = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
  ];
  let difficulty_counts = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
  ];
  let max_length = AtomicUsize::new(0);
  let mut iter = 0;

  loop {
    tokio_scoped::scope(|scope| {
      for _ in 0..BATCH_SIZE {
        scope.spawn(async {
          let thread_conn = conn.clone();
          let position = ClassicBoardGenerator::new().generate_position();
          let position_bytes = position.to_compressed_byte_array();

          let start = Instant::now();
          let solution: Vec<SolutionStep> =
            solve_position(position, 45).unwrap();
          let solve_millis = start.elapsed().as_millis();
          if solve_millis > 3000 {
            println!("Position took {}ms to solve: {:?}", solve_millis, general_purpose::URL_SAFE_NO_PAD.encode(position_bytes));
          }

          if solution.len() == 0 {
            return;
          }

          let internal_difficulty = get_solution_internal_difficulty(&solution);
          let difficulty = Difficulty::from_internal_difficulty(internal_difficulty);

          let internal_difficulty_count = internal_difficulty_counts[internal_difficulty]
            .fetch_add(1, Ordering::SeqCst);

          if internal_difficulty_count >= POSITIONS_PER_INTERNAL_DIFFICULTY {
            return;
          }

          let difficulty_count = difficulty_counts[u8::from(difficulty) as usize]
            .fetch_add(1, Ordering::SeqCst);

          sqlx::query(
            "insert into solved_positions (position, solution, difficulty, difficulty_ordinal) values (?, ?, ?, ?)",
          )
          .bind(position_bytes.as_slice())
          .bind(serde_json::to_string(&solution).unwrap())
          .bind(u8::from(difficulty))
          .bind(difficulty_count as u32)
          .execute(thread_conn.lock().await.deref_mut())
          .await
          .unwrap();

          let length = solution.len();
          max_length.fetch_max(length, Ordering::SeqCst);
        });
      }
      iter += BATCH_SIZE;
    });
    println!("##### ITERATION {} #####", iter);
    println!("Counts: {:?}", internal_difficulty_counts);
    println!(
      "Max length: {}",
      max_length.load(std::sync::atomic::Ordering::SeqCst)
    );
    if internal_difficulty_counts.iter().all(|count| {
      count.load(Ordering::SeqCst) >= POSITIONS_PER_INTERNAL_DIFFICULTY
    }) {
      break;
    }
  }
  Ok(())
}
