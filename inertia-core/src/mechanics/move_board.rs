use core::fmt;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;

#[derive(Copy, Clone)]
pub struct MoveBoard {
  pub(crate) up_moves: [Square; 256],
  pub(crate) down_moves: [Square; 256],
  pub(crate) right_moves: [Square; 256],
  pub(crate) left_moves: [Square; 256],
}

impl MoveBoard {
  pub const EMPTY: MoveBoard = MoveBoard {
    up_moves: [Square(0); 256],
    down_moves: [Square(0); 256],
    right_moves: [Square(0); 256],
    left_moves: [Square(0); 256],
  };

  pub fn get_move_destination(
    &self,
    actor_square: Square,
    actor_squares: ActorSquares,
    direction: Direction,
  ) -> Square {
    let direction_moves = match direction {
      Direction::Up => &self.up_moves,
      Direction::Down => &self.down_moves,
      Direction::Left => &self.left_moves,
      Direction::Right => &self.right_moves,
    };
    let unimpeded_move = direction_moves[actor_square.0 as usize];

    match direction {
      Direction::Up => {
        if unimpeded_move == actor_square
          || unimpeded_move.0 + 16 == actor_square.0
        {
          return unimpeded_move;
        }
        for square in actor_squares.0 {
          if square.0 % 16 == actor_square.0 % 16
            && square.0 < actor_square.0
            && square.0 > unimpeded_move.0
          {
            return square;
          }
        }
      }
      Direction::Down => {
        if unimpeded_move == actor_square
          || unimpeded_move.0 - 16 == actor_square.0
        {
          return unimpeded_move;
        }
        for square in actor_squares.0 {
          if square.0 % 16 == actor_square.0 % 16
            && square.0 > actor_square.0
            && square.0 < unimpeded_move.0
          {
            return square;
          }
        }
      }
      Direction::Left => {
        if unimpeded_move == actor_square
          || unimpeded_move.0 + 1 == actor_square.0
        {
          return unimpeded_move;
        }
        for square in actor_squares.0 {
          if square.0 / 16 == actor_square.0 / 16
            && square.0 < actor_square.0
            && square.0 > unimpeded_move.0
          {
            return square;
          }
        }
      }
      Direction::Right => {
        if unimpeded_move == actor_square
          || unimpeded_move.0 - 1 == actor_square.0
        {
          return unimpeded_move;
        }
        for square in actor_squares.0 {
          if square.0 / 16 == actor_square.0 / 16
            && square.0 > actor_square.0
            && square.0 < unimpeded_move.0
          {
            return square;
          }
        }
      }
    };

    unimpeded_move
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
      let last_square = Square::from_row_col(row, column);
      move_board.right_moves[last_square.0 as usize] = last_square;
    }

    for (row, walls) in walled_board.vertical.iter().enumerate() {
      let mut column = 15;
      for (wall, &present) in walls.iter().enumerate().rev() {
        if present {
          while column > wall {
            move_board.left_moves
              [Square::from_row_col(row, column).0 as usize] =
              Square::from_row_col(row, wall);
            column -= 1
          }
        }
      }
      let last_square = Square::from_row_col(row, column);
      move_board.left_moves[last_square.0 as usize] = last_square;
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
      let last_square = Square::from_row_col(row, column);
      move_board.down_moves[last_square.0 as usize] = last_square;
    }

    for (column, walls) in walled_board.horizontal.iter().enumerate() {
      let mut row = 15;
      for (wall, &present) in walls.iter().enumerate().rev() {
        if present {
          while column > wall {
            move_board.left_moves
              [Square::from_row_col(row, column).0 as usize] =
              Square::from_row_col(wall, column);
            row -= 1
          }
        }
      }
      let last_square = Square::from_row_col(row, column);
      move_board.left_moves[last_square.0 as usize] = last_square;
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
