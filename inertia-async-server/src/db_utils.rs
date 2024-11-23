use inertia_core::mechanics::CompressedPosition;
use inertia_core::mechanics::CompressedPositionConvertError;
use inertia_core::mechanics::Position;
use inertia_core::mechanics::PositionConvertError;
use inertia_core::mechanics::SolvedPosition;
use inertia_core::solvers::CompressedSolution;
use inertia_core::solvers::Difficulty;
use inertia_core::solvers::Solution;
use inertia_core::solvers::SolutionConvertError;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use sqlx::SqlitePool;
use thiserror::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct BoardBlobRow {
  pub position: Vec<u8>,
  pub solution: Vec<u8>,
}

pub fn num_db_positions_for_difficulty(difficulty: Difficulty) -> usize {
  match difficulty {
    Difficulty::Easiest => 1000,
    Difficulty::Easy => 1000,
    Difficulty::Medium => 1500,
    Difficulty::Hard => 1500,
    Difficulty::Hardest => 1000,
  }
}

pub struct PositionDbCoordinates {
  difficulty: Difficulty,
  ordinal: usize,
}

pub fn get_random_db_position_coordinates_in_difficulty_range(
  min_difficulty: Difficulty,
  max_difficulty: Difficulty,
) -> PositionDbCoordinates {
  let rand = rand::thread_rng();
  get_db_position_coordinates_in_difficulty_range_from_random(
    rand,
    min_difficulty,
    max_difficulty,
  )
}

pub fn get_reproducible_random_db_position_coordinates_in_difficulty_range(
  seed: u64,
  min_difficulty: Difficulty,
  max_difficulty: Difficulty,
) -> PositionDbCoordinates {
  let rand = StdRng::seed_from_u64(seed as u64);
  get_db_position_coordinates_in_difficulty_range_from_random(
    rand,
    min_difficulty,
    max_difficulty,
  )
}

pub fn get_db_position_coordinates_in_difficulty_range_from_random<T: Rng>(
  mut rand: T,
  min_difficulty: Difficulty,
  max_difficulty: Difficulty,
) -> PositionDbCoordinates {
  let difficulty = Difficulty::try_from(
    rand.gen_range(min_difficulty as u8..max_difficulty as u8),
  )
  .expect("guaranteed to be in range");
  let ordinal = rand.gen_range(0..num_db_positions_for_difficulty(difficulty));
  PositionDbCoordinates {
    difficulty,
    ordinal,
  }
}

#[derive(Error, Debug)]
pub enum DbPositionFetchError {
  #[error(transparent)]
  DbError(#[from] sqlx::Error),
  #[error("Failed to convert position blob: {0}")]
  CompressedPositionBlobConversionError(#[from] CompressedPositionConvertError),
  #[error("Failed to convert position blob: {0}")]
  PositionBlobConversionError(#[from] PositionConvertError),
  #[error("Failed to convert solution blob: {0}")]
  SolutionBlobConversionError(#[from] SolutionConvertError),
}

pub async fn get_position_from_db_coordinates(
  conn: &SqlitePool,
  coordinates: PositionDbCoordinates,
) -> Result<SolvedPosition, DbPositionFetchError> {
  let PositionDbCoordinates {
    difficulty,
    ordinal,
  } = coordinates;
  let row: BoardBlobRow = sqlx::query_as::<_, BoardBlobRow>(
    "SELECT position, solution FROM solved_positions WHERE difficulty = ? and difficulty_ordinal = ? LIMIT 1",
  )
  .bind(u8::from(difficulty))
  .bind(ordinal as u32)
  .fetch_one(conn)
  .await?;
  let position = Position::try_from(CompressedPosition(row.position))?;
  let solution = Solution::try_from(CompressedSolution(row.solution))?;
  Ok(SolvedPosition { position, solution })
}
