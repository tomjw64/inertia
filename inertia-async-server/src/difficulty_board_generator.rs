use inertia_core::mechanics::SolvedPosition;
use inertia_core::mechanics::SolvedPositionGenerator;
use inertia_core::solvers::Difficulty;
use sqlx::SqlitePool;

use crate::db_utils::get_position_from_db_coordinates;
use crate::db_utils::get_random_db_position_coordinates_in_difficulty_range;

#[derive(Debug, Clone)]
pub struct DifficultyDbBoardGenerator {
  db_pool: SqlitePool,
  min_difficulty: Difficulty,
  max_difficulty: Difficulty,
}

impl DifficultyDbBoardGenerator {
  pub fn new(
    db_pool: SqlitePool,
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
      get_position_from_db_coordinates(
        &self.db_pool,
        get_random_db_position_coordinates_in_difficulty_range(
          self.min_difficulty,
          self.max_difficulty,
        ),
      )
      .await
      .unwrap_or_else(|err| {
        tracing::error!("Error fetching solved position: {}", err);
        return SolvedPosition::default();
      })
    })
  }
}
