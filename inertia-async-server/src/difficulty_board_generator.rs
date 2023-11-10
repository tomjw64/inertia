use inertia_core::mechanics::WalledBoardPosition;
use inertia_core::mechanics::WalledBoardPositionGenerator;
use inertia_core::solvers::difficulty::Difficulty;
use sqlx::Pool;
use sqlx::Sqlite;

#[derive(sqlx::FromRow, Debug)]
struct BoardBlobRow {
  board: Box<[u8]>,
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

impl WalledBoardPositionGenerator for DifficultyDbBoardGenerator {
  fn generate_position(&self) -> WalledBoardPosition {
    futures::executor::block_on(async {
      let mut conn = match self.db_pool.acquire().await {
        Ok(conn) => conn,
        Err(err) => {
          tracing::error!("Error fetching board: {}", err);
          return WalledBoardPosition::default();
        }
      };
      let row = match sqlx::query_as::<_, BoardBlobRow>(
        "SELECT board FROM boards WHERE difficulty >= ? AND difficulty <= ? ORDER BY random() LIMIT 1",
      )
      .bind(self.min_difficulty as u8)
      .bind(self.max_difficulty as u8)
      .fetch_one(&mut *conn)
      .await
      {
        Ok(row) => row,
        Err(err) => {
          tracing::error!("Error fetching board: {}", err);
          return WalledBoardPosition::default();
        }
      };
      let compressed_byte_array = match <[u8; 69]>::try_from(row.board.as_ref())
      {
        Ok(bytes) => bytes,
        Err(err) => {
          tracing::error!("Error fetching board: {}", err);
          return WalledBoardPosition::default();
        }
      };
      WalledBoardPosition::from_compressed_byte_array(&compressed_byte_array)
    })
  }
}
