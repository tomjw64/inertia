use inertia_core::solvers::Difficulty;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub fn num_db_positions_for_difficulty(difficulty: Difficulty) -> usize {
  match difficulty {
    Difficulty::Easiest => 1000,
    Difficulty::Easy => 1000,
    Difficulty::Medium => 1500,
    Difficulty::Hard => 1500,
    Difficulty::Hardest => 1000,
  }
}

struct PositionDbCoordinates {
  difficulty: Difficulty,
  ordinal: usize,
}

pub fn get_random_db_position_coordinates() -> PositionDbCoordinates {
  let mut rand = rand::thread_rng();
  let difficulty = rand.gen();
  let ordinal = rand.gen_range(0..num_db_positions_for_difficulty(difficulty));
  PositionDbCoordinates {
    difficulty,
    ordinal,
  }
}

pub fn get_reproducible_random_db_position_coordinates(
  seed: usize,
) -> PositionDbCoordinates {
  let mut rand = StdRng::seed_from_u64(seed as u64);
  let difficulty = rand.gen();
  let ordinal = rand.gen_range(0..num_db_positions_for_difficulty(difficulty));
  PositionDbCoordinates {
    difficulty,
    ordinal,
  }
}
