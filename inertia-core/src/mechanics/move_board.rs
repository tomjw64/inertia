use core::fmt;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;

use crate::mechanics::BitBoard;

#[derive(Copy, Clone)]
pub struct MoveBoard {
  pub(crate) up_moves: [Square; 256],
  pub(crate) down_moves: [Square; 256],
  pub(crate) right_moves: [Square; 256],
  pub(crate) left_moves: [Square; 256],
}

impl MoveBoard {
  const fn get_empty_up() -> [Square; 256] {
    let mut result = [Square(0); 256];
    let mut i = 0u8;
    while i < 255 {
      result[i as usize] = Square(i % 16);
      i += 1;
    }
    result[i as usize] = Square(i % 16);
    result
  }

  const fn get_empty_down() -> [Square; 256] {
    let mut result = [Square(0); 256];
    let mut i = 0u8;
    while i < 255 {
      result[i as usize] = Square(240 + i % 16);
      i += 1;
    }
    result[i as usize] = Square(240 + i % 16);
    result
  }

  const fn get_empty_left() -> [Square; 256] {
    let mut result = [Square(0); 256];
    let mut i = 0u8;
    while i < 255 {
      result[i as usize] = Square(i / 16 * 16);
      i += 1;
    }
    result[i as usize] = Square(i / 16 * 16);
    result
  }

  const fn get_empty_right() -> [Square; 256] {
    let mut result = [Square(0); 256];
    let mut i = 0u8;
    while i < 255 {
      result[i as usize] = Square(i / 16 * 16 + 15);
      i += 1;
    }
    result[i as usize] = Square(i / 16 * 16 + 15);
    result
  }

  pub const EMPTY: Self = Self {
    up_moves: Self::get_empty_up(),
    down_moves: Self::get_empty_down(),
    left_moves: Self::get_empty_left(),
    right_moves: Self::get_empty_right(),
  };

  pub fn has_any_block_on(&self, square: Square) -> bool {
    [
      &self.up_moves,
      &self.down_moves,
      &self.left_moves,
      &self.right_moves,
    ]
    .iter()
    .any(|&squares| squares[square.0 as usize] == square)
  }

  pub fn get_unimpeded_movement_ray(
    &self,
    actor_square: Square,
    direction: Direction,
  ) -> BitBoard {
    let move_destination =
      self.get_unimpeded_move_destination(actor_square, direction);
    let mut ray = BitBoard::ZERO;
    match direction {
      Direction::Up => {
        for index in (move_destination.0..=actor_square.0).step_by(16) {
          ray.set_bit(index);
        }
      }
      Direction::Down => {
        for index in (actor_square.0..=move_destination.0).step_by(16) {
          ray.set_bit(index);
        }
      }
      Direction::Left => {
        for index in move_destination.0..=actor_square.0 {
          ray.set_bit(index);
        }
      }
      Direction::Right => {
        for index in actor_square.0..=move_destination.0 {
          ray.set_bit(index);
        }
      }
    };
    ray
  }

  pub fn get_movement_ray(
    &self,
    actor_square: Square,
    actor_squares: ActorSquares,
    direction: Direction,
  ) -> BitBoard {
    let move_destination =
      self.get_move_destination(actor_square, actor_squares, direction);
    let mut ray = BitBoard::ZERO;
    match direction {
      Direction::Up => {
        for index in (move_destination.0..=actor_square.0).step_by(16) {
          ray.set_bit(index);
        }
      }
      Direction::Down => {
        for index in (actor_square.0..=move_destination.0).step_by(16) {
          ray.set_bit(index);
        }
      }
      Direction::Left => {
        for index in move_destination.0..=actor_square.0 {
          ray.set_bit(index);
        }
      }
      Direction::Right => {
        for index in actor_square.0..=move_destination.0 {
          ray.set_bit(index);
        }
      }
    };
    ray
  }

  pub fn get_unimpeded_movement_ray_squares(
    &self,
    actor_square: Square,
    direction: Direction,
  ) -> Vec<Square> {
    let mut result = Vec::with_capacity(16);
    let move_destination =
      self.get_unimpeded_move_destination(actor_square, direction);
    match direction {
      Direction::Up => {
        for index in (move_destination.0..=actor_square.0).step_by(16).rev() {
          result.push(Square(index));
        }
      }
      Direction::Down => {
        for index in (actor_square.0..=move_destination.0).step_by(16) {
          result.push(Square(index));
        }
      }
      Direction::Left => {
        for index in (move_destination.0..=actor_square.0).rev() {
          result.push(Square(index));
        }
      }
      Direction::Right => {
        for index in actor_square.0..=move_destination.0 {
          result.push(Square(index));
        }
      }
    }
    result
  }

  pub fn get_unimpeded_move_destination(
    &self,
    actor_square: Square,
    direction: Direction,
  ) -> Square {
    match direction {
      Direction::Up => self.up_moves[actor_square.0 as usize],
      Direction::Down => self.down_moves[actor_square.0 as usize],
      Direction::Left => self.left_moves[actor_square.0 as usize],
      Direction::Right => self.right_moves[actor_square.0 as usize],
    }
  }

  pub fn get_all_actor_move_destinations(
    &self,
    actor_squares: ActorSquares,
  ) -> [[(Square, bool); 4]; 4] {
    let actor_unimpeded_up_moves = actor_squares
      .0
      .map(|square| (square, self.up_moves[square.0 as usize]));
    let actor_unimpeded_down_moves = actor_squares
      .0
      .map(|square| (square, self.down_moves[square.0 as usize]));
    let actor_unimpeded_left_moves = actor_squares
      .0
      .map(|square| (square, self.left_moves[square.0 as usize]));
    let actor_unimpeded_right_moves = actor_squares
      .0
      .map(|square| (square, self.right_moves[square.0 as usize]));

    let mut result = [[(Square(0), false); 4]; 4];
    for actor_index in 0..4 {
      let actor_square = actor_squares.0[actor_index];
      let actor_unimpeded_up_move = actor_unimpeded_up_moves[actor_index].1;
      let actor_unimpeded_down_move = actor_unimpeded_down_moves[actor_index].1;
      let actor_unimpeded_left_move = actor_unimpeded_left_moves[actor_index].1;
      let actor_unimpeded_right_move =
        actor_unimpeded_right_moves[actor_index].1;

      let up_dest = actor_unimpeded_up_moves
        .iter()
        .copied()
        .map(|(other_actor_square, other_actor_unimpeded_up_move)| {
          if actor_unimpeded_up_move == other_actor_unimpeded_up_move
            && other_actor_square < actor_square
          {
            Square(other_actor_square.0 + 16)
          } else {
            actor_unimpeded_up_move
          }
        })
        .max()
        .unwrap();
      let down_dest = actor_unimpeded_down_moves
        .iter()
        .copied()
        .map(|(other_actor_square, other_actor_unimpeded_down_move)| {
          if actor_unimpeded_down_move == other_actor_unimpeded_down_move
            && other_actor_square > actor_square
          {
            Square(other_actor_square.0 - 16)
          } else {
            actor_unimpeded_down_move
          }
        })
        .min()
        .unwrap();
      let left_dest = actor_unimpeded_left_moves
        .iter()
        .copied()
        .map(|(other_actor_square, other_actor_unimpeded_left_move)| {
          if actor_unimpeded_left_move == other_actor_unimpeded_left_move
            && other_actor_square < actor_square
          {
            Square(other_actor_square.0 + 1)
          } else {
            actor_unimpeded_left_move
          }
        })
        .max()
        .unwrap();
      let right_dest = actor_unimpeded_right_moves
        .iter()
        .copied()
        .map(|(other_actor_square, other_actor_unimpeded_right_move)| {
          if actor_unimpeded_right_move == other_actor_unimpeded_right_move
            && other_actor_square > actor_square
          {
            Square(other_actor_square.0 - 1)
          } else {
            actor_unimpeded_right_move
          }
        })
        .min()
        .unwrap();
      result[actor_index] = [
        (up_dest, up_dest == actor_unimpeded_up_move),
        (down_dest, down_dest == actor_unimpeded_down_move),
        (left_dest, left_dest == actor_unimpeded_left_move),
        (right_dest, right_dest == actor_unimpeded_right_move),
      ];
    }
    result
  }

  pub fn get_move_destination(
    &self,
    actor_square: Square,
    actor_squares: ActorSquares,
    direction: Direction,
  ) -> Square {
    let actor_square_row = actor_square.0 / 16;
    let actor_square_col = actor_square.0 % 16;
    match direction {
      Direction::Up => {
        let unimpeded_move = self.up_moves[actor_square.0 as usize];
        if unimpeded_move == actor_square {
          return unimpeded_move;
        }
        let mut max = unimpeded_move.0;
        for square in actor_squares.0 {
          if square.0 % 16 == actor_square_col
            && square.0 < actor_square.0
            && square.0 >= max
          {
            max = square.0 + 16;
          }
        }
        Square(max)
      }
      Direction::Down => {
        let unimpeded_move = self.down_moves[actor_square.0 as usize];
        if unimpeded_move == actor_square {
          return unimpeded_move;
        }
        let mut min = unimpeded_move.0;
        for square in actor_squares.0 {
          if square.0 % 16 == actor_square_col
            && square.0 > actor_square.0
            && square.0 <= min
          {
            min = square.0 - 16;
          }
        }
        Square(min)
      }
      Direction::Left => {
        let unimpeded_move = self.left_moves[actor_square.0 as usize];
        if unimpeded_move == actor_square {
          return unimpeded_move;
        }
        let mut max = unimpeded_move.0;
        for square in actor_squares.0 {
          if square.0 / 16 == actor_square_row
            && square.0 < actor_square.0
            && square.0 >= max
          {
            max = square.0 + 1;
          }
        }
        Square(max)
      }
      Direction::Right => {
        let unimpeded_move = self.right_moves[actor_square.0 as usize];
        if unimpeded_move == actor_square {
          return unimpeded_move;
        }
        let mut min = unimpeded_move.0;
        for square in actor_squares.0 {
          if square.0 / 16 == actor_square_row
            && square.0 > actor_square.0
            && square.0 <= min
          {
            min = square.0 - 1;
          }
        }
        Square(min)
      }
    }
  }
}

impl fmt::Debug for MoveBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let strings = self.up_moves.map(|x| format!("{:03}", x.0));
    let rows = strings.chunks(16).collect::<Vec<_>>();
    f.write_str("\n")?;
    for row in rows {
      f.write_str("[")?;
      f.write_str(&row.join(", "))?;
      f.write_str("]\n")?;
    }
    let strings = self.down_moves.map(|x| format!("{:03}", x.0));
    let rows = strings.chunks(16).collect::<Vec<_>>();
    f.write_str("\n")?;
    for row in rows {
      f.write_str("[")?;
      f.write_str(&row.join(", "))?;
      f.write_str("]\n")?;
    }
    let strings = self.left_moves.map(|x| format!("{:03}", x.0));
    let rows = strings.chunks(16).collect::<Vec<_>>();
    f.write_str("\n")?;
    for row in rows {
      f.write_str("[")?;
      f.write_str(&row.join(", "))?;
      f.write_str("]\n")?;
    }
    let strings = self.right_moves.map(|x| format!("{:03}", x.0));
    let rows = strings.chunks(16).collect::<Vec<_>>();
    f.write_str("\n")?;
    for row in rows {
      f.write_str("[")?;
      f.write_str(&row.join(", "))?;
      f.write_str("]\n")?;
    }
    Ok(())
  }
}

impl From<&WalledBoard> for MoveBoard {
  fn from(walled_board: &WalledBoard) -> Self {
    let mut move_board = MoveBoard {
      up_moves: [Square(0); 256],
      down_moves: [Square(0); 256],
      right_moves: [Square(0); 256],
      left_moves: [Square(0); 256],
    };

    for (column, walls) in walled_board.horizontal.iter().enumerate() {
      let mut row = 15;
      for (wall, &present) in walls.iter().enumerate().rev() {
        if present {
          while row > wall {
            move_board.up_moves[Square::from_row_col(row, column).0 as usize] =
              Square::from_row_col(wall + 1, column);
            row -= 1
          }
        }
      }
      for remaining_row in 0..=row {
        move_board.up_moves
          [Square::from_row_col(remaining_row, column).0 as usize] =
          Square::from_row_col(0, column);
      }
    }

    for (column, walls) in walled_board.horizontal.iter().enumerate() {
      let mut row = 0;
      for (wall, &present) in walls.iter().enumerate() {
        if present {
          while row <= wall {
            move_board.down_moves
              [Square::from_row_col(row, column).0 as usize] =
              Square::from_row_col(wall, column);
            row += 1
          }
        }
      }
      for remaining_row in row..=15 {
        move_board.down_moves
          [Square::from_row_col(remaining_row, column).0 as usize] =
          Square::from_row_col(15, column);
      }
    }

    for (row, walls) in walled_board.vertical.iter().enumerate() {
      let mut column = 15;
      for (wall, &present) in walls.iter().enumerate().rev() {
        if present {
          while column > wall {
            move_board.left_moves
              [Square::from_row_col(row, column).0 as usize] =
              Square::from_row_col(row, wall + 1);
            column -= 1
          }
        }
      }
      for remaining_column in 0..=column {
        move_board.left_moves
          [Square::from_row_col(row, remaining_column).0 as usize] =
          Square::from_row_col(row, 0);
      }
    }

    for (row, walls) in walled_board.vertical.iter().enumerate() {
      let mut column = 0;
      for (wall, &present) in walls.iter().enumerate() {
        if present {
          while column <= wall {
            move_board.right_moves
              [Square::from_row_col(row, column).0 as usize] =
              Square::from_row_col(row, wall);
            column += 1
          }
        }
      }
      for remaining_column in column..=15 {
        move_board.right_moves
          [Square::from_row_col(row, remaining_column).0 as usize] =
          Square::from_row_col(row, 15);
      }
    }

    move_board
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_up_unblocked() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(250),
        ActorSquares([Square(0); 4]),
        Direction::Up
      ),
      Square(10)
    );
  }

  #[test]
  fn test_up_blocked_by_occupied() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(250),
        ActorSquares([Square(26); 4]),
        Direction::Up
      ),
      Square(42)
    );
  }

  #[test]
  fn test_up_blocked_by_multiple() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(250),
        ActorSquares([Square(26), Square(26), Square(58), Square(58)]),
        Direction::Up
      ),
      Square(74)
    );
  }

  #[test]
  fn test_up_zero() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(0),
        ActorSquares([Square(255); 4]),
        Direction::Up
      ),
      Square(0)
    );
  }

  #[test]
  fn test_up_max() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(255),
        ActorSquares([Square(0); 4]),
        Direction::Up
      ),
      Square(15)
    );
  }

  #[test]
  fn test_down_unblocked() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(10),
        ActorSquares([Square(0); 4]),
        Direction::Down
      ),
      Square(250)
    );
  }

  #[test]
  fn test_down_blocked_by_occupied() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(10),
        ActorSquares([Square(42); 4]),
        Direction::Down
      ),
      Square(26)
    );
  }

  #[test]
  fn test_down_blocked_by_multiple() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(10),
        ActorSquares([Square(234), Square(234), Square(202), Square(202)]),
        Direction::Down
      ),
      Square(186)
    );
  }

  #[test]
  fn test_down_zero() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(0),
        ActorSquares([Square(255); 4]),
        Direction::Down
      ),
      Square(240)
    );
  }

  #[test]
  fn test_down_max() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(255),
        ActorSquares([Square(0); 4]),
        Direction::Down
      ),
      Square(255)
    );
  }

  #[test]
  fn test_left_unblocked() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(79),
        ActorSquares([Square(0); 4]),
        Direction::Left
      ),
      Square(64)
    );
  }

  #[test]
  fn test_left_blocked_by_occupied() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(79),
        ActorSquares([Square(68); 4]),
        Direction::Left
      ),
      Square(69)
    );
  }

  #[test]
  fn test_left_blocked_by_multiple() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(79),
        ActorSquares([Square(68), Square(68), Square(73), Square(73)]),
        Direction::Left
      ),
      Square(74)
    );
  }

  #[test]
  fn test_left_zero() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(0),
        ActorSquares([Square(255); 4]),
        Direction::Left
      ),
      Square(0)
    );
  }

  #[test]
  fn test_left_max() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(255),
        ActorSquares([Square(0); 4]),
        Direction::Left
      ),
      Square(240)
    );
  }

  #[test]
  fn test_right_unblocked() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(64),
        ActorSquares([Square(0); 4]),
        Direction::Right
      ),
      Square(79)
    );
  }

  #[test]
  fn test_right_blocked_by_occupied() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(64),
        ActorSquares([Square(70); 4]),
        Direction::Right
      ),
      Square(69)
    );
  }

  #[test]
  fn test_right_blocked_by_multiple() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(64),
        ActorSquares([Square(66), Square(66), Square(70), Square(70)]),
        Direction::Right
      ),
      Square(65)
    );
  }

  #[test]
  fn test_right_zero() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(0),
        ActorSquares([Square(255); 4]),
        Direction::Right
      ),
      Square(15)
    );
  }

  #[test]
  fn test_right_max() {
    assert_eq!(
      MoveBoard::EMPTY.get_move_destination(
        Square(255),
        ActorSquares([Square(0); 4]),
        Direction::Right
      ),
      Square(255)
    );
  }
}
