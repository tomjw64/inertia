use std::collections::HashSet;
use std::ops::Range;

use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::mechanics::WalledBoardPosition;
use crate::mechanics::WalledBoardPositionGenerator;

pub struct ClassicBoardGenerator;

impl ClassicBoardGenerator {
  pub fn new() -> Self {
    Self
  }
}

impl Default for ClassicBoardGenerator {
  fn default() -> Self {
    ClassicBoardGenerator::new()
  }
}

fn add_central_box(walled_board: &mut WalledBoard) {
  walled_board.col_mut(7)[6] = true;
  walled_board.col_mut(8)[6] = true;

  walled_board.col_mut(7)[8] = true;
  walled_board.col_mut(8)[8] = true;

  walled_board.row_mut(7)[6] = true;
  walled_board.row_mut(8)[6] = true;

  walled_board.row_mut(7)[8] = true;
  walled_board.row_mut(8)[8] = true;
}

fn add_edge_walls<R: Rng>(rng: &mut R, walled_board: &mut WalledBoard) {
  (1..7)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.row_mut(0)[index] = true);
  (8..14)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.row_mut(0)[index] = true);

  (1..7)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.row_mut(15)[index] = true);
  (8..14)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.row_mut(15)[index] = true);

  (1..7)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.col_mut(0)[index] = true);
  (8..14)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.col_mut(0)[index] = true);

  (1..7)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.col_mut(15)[index] = true);
  (8..14)
    .choose(rng)
    .iter()
    .for_each(|&index| walled_board.col_mut(15)[index] = true);
}

fn add_midboard_corners<R: Rng>(rng: &mut R, walled_board: &mut WalledBoard) {
  let mut quad_handlers: [fn(&mut R, &mut WalledBoard); 4] = [
    add_quad1_corners,
    add_quad2_corners,
    add_quad3_corners,
    add_quad4_corners,
  ];
  quad_handlers.shuffle(rng);
  quad_handlers
    .iter()
    .for_each(|quad_handler| quad_handler(rng, walled_board));
}

fn add_quad1_corners<R: Rng>(rng: &mut R, walled_board: &mut WalledBoard) {
  let num_corners = (3..=5).choose(rng).unwrap();
  add_corners_in_range(rng, walled_board, num_corners, 1..8, 1..8);
}

fn add_quad2_corners<R: Rng>(rng: &mut R, walled_board: &mut WalledBoard) {
  let num_corners = (3..=5).choose(rng).unwrap();
  add_corners_in_range(rng, walled_board, num_corners, 1..8, 8..15);
}

fn add_quad3_corners<R: Rng>(rng: &mut R, walled_board: &mut WalledBoard) {
  let num_corners = (3..=5).choose(rng).unwrap();
  add_corners_in_range(rng, walled_board, num_corners, 8..15, 1..8);
}

fn add_quad4_corners<R: Rng>(rng: &mut R, walled_board: &mut WalledBoard) {
  let num_corners = (3..=5).choose(rng).unwrap();
  add_corners_in_range(rng, walled_board, num_corners, 8..15, 8..15);
}

fn add_corners_in_range<R: Rng>(
  rng: &mut R,
  walled_board: &mut WalledBoard,
  num_corners: usize,
  row_range: Range<usize>,
  col_range: Range<usize>,
) {
  let mut candidate_squares: HashSet<(usize, usize)> = HashSet::new();
  for row in row_range {
    for col in col_range.clone() {
      candidate_squares.insert((row, col));
    }
  }

  let mut num_corners_remaining = num_corners;

  while num_corners_remaining > 0 {
    let sampleable_candidates = Vec::from_iter(candidate_squares.iter());
    let candidate = sampleable_candidates.choose(rng);
    match candidate {
      None => break,
      Some(&&candidate) => {
        let (row, col) = candidate;
        candidate_squares.remove(&(row, col));

        let cannot_place = walled_board.col(col)[row - 1]
          || walled_board.col(col)[row]
          || walled_board.row(row)[col - 1]
          || walled_board.row(row)[col];
        if cannot_place {
          continue;
        }

        let mut vertical_candidates = vec![];
        let can_place_left = !walled_board.col(col - 1)[row - 1]
          && !walled_board.col(col - 1)[row]
          && !walled_board.row(row - 1)[col - 1]
          && !walled_board.row(row + 1)[col - 1];
        let can_place_right = !walled_board.col(col + 1)[row - 1]
          && !walled_board.col(col + 1)[row]
          && !walled_board.row(row - 1)[col]
          && !walled_board.row(row + 1)[col];
        if can_place_left {
          vertical_candidates.push(col - 1);
        }
        if can_place_right {
          vertical_candidates.push(col);
        }
        if vertical_candidates.is_empty() {
          continue;
        }

        let mut horizontal_candidates = vec![];
        let can_place_up = !walled_board.row(row - 1)[col - 1]
          && !walled_board.row(row - 1)[col]
          && !walled_board.col(col - 1)[row - 1]
          && !walled_board.col(col + 1)[row - 1];
        let can_place_down = !walled_board.row(row + 1)[col - 1]
          && !walled_board.row(row + 1)[col]
          && !walled_board.col(col - 1)[row]
          && !walled_board.col(col + 1)[row];
        if can_place_up {
          horizontal_candidates.push(row - 1);
        }
        if can_place_down {
          horizontal_candidates.push(row);
        }
        if horizontal_candidates.is_empty() {
          continue;
        }

        let vertical_block = *vertical_candidates.choose(rng).unwrap();
        walled_board.row_mut(row)[vertical_block] = true;

        let horizontal_block = *horizontal_candidates.choose(rng).unwrap();
        walled_board.col_mut(col)[horizontal_block] = true;

        num_corners_remaining -= 1;
      }
    }
  }
}

impl WalledBoardPositionGenerator for ClassicBoardGenerator {
  fn generate_position(&self) -> WalledBoardPosition {
    let mut walled_board = WalledBoard::EMPTY;
    let mut rng = thread_rng();

    add_central_box(&mut walled_board);
    add_edge_walls(&mut rng, &mut walled_board);
    add_midboard_corners(&mut rng, &mut walled_board);

    // Exclude central box for goal and actors. Squares: 119, 120, 135, 136
    let mut goal_and_actor_squares: [u8; 5] = [0, 0, 0, 0, 0];
    let mut goal_and_actor_candidates: Vec<u8> = Vec::new();
    goal_and_actor_candidates.extend(0..=118);
    goal_and_actor_candidates.extend(121..=134);
    goal_and_actor_candidates.extend(137..=255);
    goal_and_actor_candidates
      .iter()
      .cloned()
      .choose_multiple_fill(&mut rng, &mut goal_and_actor_squares);
    goal_and_actor_squares.shuffle(&mut rng);

    WalledBoardPosition {
      goal: Square(goal_and_actor_squares[0]),
      actor_squares: ActorSquares([
        Square(goal_and_actor_squares[1]),
        Square(goal_and_actor_squares[2]),
        Square(goal_and_actor_squares[3]),
        Square(goal_and_actor_squares[4]),
      ]),
      walled_board,
    }
  }
}
