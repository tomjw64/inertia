use crate::mechanics::ActorSquares;

// Change if you need values > 255
pub type HeuristicValue = u8;

pub trait Heuristic {
  fn get_heuristic(&self, actor_squares: ActorSquares) -> HeuristicValue;

  // Should always return a value greater than or equal to get_heuristic (as
  // this function is soley for optimization). Thus, returning the result of
  // get_heuristic is always fine, though unoptomized.
  fn get_heuristic_for_target_actor(
    &self,
    actor_squares: ActorSquares,
    actor_index: usize,
  ) -> HeuristicValue {
    let _ = actor_index; // Not needed for default implementation
    self.get_heuristic(actor_squares)
  }
}

// This should be branchless - check godbolt after editing :)
pub(crate) fn get_min_two(bytes: [HeuristicValue; 4]) -> [HeuristicValue; 2] {
  let mut min_two = [HeuristicValue::MAX, HeuristicValue::MAX];
  for val in bytes {
    if val < min_two[1] {
      if val < min_two[0] {
        min_two = [val, min_two[0]]
      } else {
        min_two = [min_two[0], val]
      }
    }
  }
  min_two
}

// This should be branchless - check godbolt after editing :)
pub(crate) fn get_min(bytes: [HeuristicValue; 4]) -> HeuristicValue {
  let mut min = HeuristicValue::MAX;
  for val in bytes {
    if val < min {
      min = val
    }
  }
  min
}
