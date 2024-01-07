use inertia_core::mechanics::Position;
use inertia_core::mechanics::SolvedPosition;
use inertia_core::mechanics::SolvedPositionGenerator;
use inertia_core::solvers::difficulty::Difficulty;
use sqlx::Pool;
use sqlx::Sqlite;

#[derive(sqlx::FromRow, Debug)]
struct BoardBlobRow {
  board: Box<[u8]>,
  solution: String,
}

#[derive(Debug, Clone)]
pub struct DifficultyDbBoardGenerator {
  db_pool: Pool<Sqlite>,
  min_difficulty: Difficulty,
  max_difficulty: Difficulty,
}

impl DifficultyDbBoardGenerator {
  pub fn new(
    db_pool: Pool<Sqlite>,
    min_difficulty: Difficulty,
    max_difficulty: Difficulty,
  ) -> Self {
    Self {
      db_pool,
      min_difficulty,
      max_difficulty,
    }
  }
}

impl SolvedPositionGenerator for DifficultyDbBoardGenerator {
  fn generate_solved_position(&self) -> SolvedPosition {
    futures::executor::block_on(async {
      let mut conn = match self.db_pool.acquire().await {
        Ok(conn) => conn,
        Err(err) => {
          tracing::error!("Error fetching board: {}", err);
          return SolvedPosition::default();
        }
      };
      let row = match sqlx::query_as::<_, BoardBlobRow>(
        "SELECT board, solution FROM boards WHERE difficulty >= ? AND difficulty <= ? ORDER BY random() LIMIT 1",
      )
      .bind(self.min_difficulty as u8)
      .bind(self.max_difficulty as u8)
      .fetch_one(&mut *conn)
      .await
      {
        Ok(row) => row,
        Err(err) => {
          tracing::error!("Error fetching board: {}", err);
          return SolvedPosition::default();
        }
      };
      let board_compressed_byte_array =
        match <[u8; 69]>::try_from(row.board.as_ref()) {
          Ok(bytes) => bytes,
          Err(err) => {
            tracing::error!("Error fetching board: {}", err);
            return SolvedPosition::default();
          }
        };
      SolvedPosition {
        position: Position::from_compressed_byte_array(
          &board_compressed_byte_array,
        ),
        solution: serde_json::from_str(row.solution.as_str()).unwrap(),
      }
    })
  }
}
