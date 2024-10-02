use crate::mechanics::ActorSquares;

#[cfg(feature = "web")]
use tsify::declare;

// Change if you need values > 255
#[cfg_attr(feature = "web", declare)]
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
pub(crate) fn get_min_two<const N: usize>(
  vals: [HeuristicValue; N],
) -> [HeuristicValue; 2] {
  let mut min_two = [HeuristicValue::MAX, HeuristicValue::MAX];
  for val in vals {
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
pub(crate) fn get_min<const N: usize>(
  vals: [HeuristicValue; N],
) -> HeuristicValue {
  let mut min = HeuristicValue::MAX;
  for val in vals {
    if val < min {
      min = val
    }
  }
  min
}

// This should be branchless - check godbolt after editing :)
pub(crate) fn get_max<const N: usize>(
  vals: [HeuristicValue; N],
) -> HeuristicValue {
  let mut max = HeuristicValue::MIN;
  for val in vals {
    if val > max {
      max = val
    }
  }
  max
}
