// 'use crate::mechanics::BitBoard;
// use crate::mechanics::BlockBoard;
// use crate::mechanics::Direction;
// use crate::mechanics::Square;
// use crate::mechanics::DOWN_MOVE_RAYS;
// use crate::mechanics::LEFT_MOVE_RAYS;
// use crate::mechanics::RIGHT_MOVE_RAYS;
// use crate::mechanics::UP_MOVE_RAYS;

// use super::ActorSquares;
// use super::MoveBoard;'

// pub fn get_movement_ray(
//   board: &BlockBoard,
//   actor_square: Square,
//   occupied_squares: BitBoard,
//   direction: Direction,
// ) -> BitBoard {
//   let actor_bitboard = BitBoard::from(actor_square);

//   let (direction_rays, direction_blocks) = match direction {
//     Direction::Up => (&UP_MOVE_RAYS, board.up_blocks),
//     Direction::Down => (&DOWN_MOVE_RAYS, board.down_blocks),
//     Direction::Left => (&LEFT_MOVE_RAYS, board.left_blocks),
//     Direction::Right => (&RIGHT_MOVE_RAYS, board.right_blocks),
//   };

//   let unblocked_movement_ray = direction_rays[actor_square.0 as usize];

//   let blocks = (direction_blocks | occupied_squares) & !actor_bitboard;
//   let blocks_in_movement_ray = blocks & unblocked_movement_ray;

//   let controlling_block = match direction {
//     Direction::Up | Direction::Left => blocks_in_movement_ray.last_one(),
//     Direction::Down | Direction::Right => blocks_in_movement_ray.first_one(),
//   };
//   match controlling_block {
//     None => unblocked_movement_ray,
//     Some(block_idx) => {
//       unblocked_movement_ray & !direction_rays[block_idx as usize]
//     }
//   }
// }

// // Ray based movement seems unfortunately slower than the naive iterative
// // approach below, but it's pretty elegant :)
// // pub fn get_move_destination(
// //   board: &Board,
// //   actor_square: Square,
// //   occupied_squares: BitBoard,
// //   direction: Direction,
// // ) -> Square {
// //   let movement_ray =
// //     get_movement_ray(board, actor_square, occupied_squares, direction);

// //   let destination_cell_index = match direction {
// //     Direction::Up | Direction::Left => movement_ray.first_one(),
// //     Direction::Down | Direction::Right => movement_ray.last_one(),
// //   };

// //   destination_cell_index.map(Square).unwrap_or(actor_square)
// // }

// pub fn get_move_destination(
//   board: &BlockBoard,
//   actor_square: Square,
//   occupied_squares: BitBoard,
//   direction: Direction,
// ) -> Square {
//   let (increment, increment_pos, boundary, direction_blocks) = match direction {
//     Direction::Up => (16, false, actor_square.0 % 16, board.up_blocks),
//     Direction::Down => (16, true, 240 + actor_square.0 % 16, board.down_blocks),
//     Direction::Left => {
//       (1, false, (actor_square.0 / 16) * 16, board.left_blocks)
//     }
//     Direction::Right => {
//       (1, true, (actor_square.0 / 16) * 16 + 15, board.right_blocks)
//     }
//   };
//   let all_blocks = direction_blocks | occupied_squares;
//   let mut current = actor_square.0;
//   if increment_pos {
//     while current != boundary && !all_blocks.bit((current + increment) as usize)
//     {
//       current += increment;
//     }
//   } else {
//     while current != boundary && !all_blocks.bit((current - increment) as usize)
//     {
//       current -= increment;
//     }
//   }
//   Square(current)
// }

// #[cfg(test)]
// mod test {
//   use super::*;

//   #[test]
//   fn test_up_unblocked() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(250),
//         BitBoard::ZERO,
//         Direction::Up
//       ),
//       Square(10)
//     );
//   }

//   #[test]
//   fn test_up_blocked_by_occupied() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(250),
//         BitBoard::from(Square(26)),
//         Direction::Up
//       ),
//       Square(42)
//     );
//   }

//   #[test]
//   fn test_up_blocked_by_board() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard {
//           up_blocks: BitBoard::from(Square(26)),
//           ..BlockBoard::EMPTY
//         },
//         Square(250),
//         BitBoard::ZERO,
//         Direction::Up
//       ),
//       Square(42)
//     );
//   }

//   #[test]
//   fn test_up_blocked_by_multiple() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(250),
//         BitBoard::from(Square(26)) | BitBoard::from(Square(58)),
//         Direction::Up
//       ),
//       Square(74)
//     );
//   }

//   #[test]
//   fn test_up_zero() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(0),
//         BitBoard::ZERO,
//         Direction::Up
//       ),
//       Square(0)
//     );
//   }

//   #[test]
//   fn test_up_max() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(255),
//         BitBoard::ZERO,
//         Direction::Up
//       ),
//       Square(15)
//     );
//   }

//   #[test]
//   fn test_down_unblocked() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(10),
//         BitBoard::ZERO,
//         Direction::Down
//       ),
//       Square(250)
//     );
//   }

//   #[test]
//   fn test_down_blocked_by_occupied() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(10),
//         BitBoard::from(Square(42)),
//         Direction::Down
//       ),
//       Square(26)
//     );
//   }

//   #[test]
//   fn test_down_blocked_by_board() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard {
//           down_blocks: BitBoard::from(Square(42)),
//           ..BlockBoard::EMPTY
//         },
//         Square(10),
//         BitBoard::ZERO,
//         Direction::Down
//       ),
//       Square(26)
//     );
//   }

//   #[test]
//   fn test_down_blocked_by_multiple() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(10),
//         BitBoard::from(Square(234)) | BitBoard::from(Square(202)),
//         Direction::Down
//       ),
//       Square(186)
//     );
//   }

//   #[test]
//   fn test_down_zero() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(0),
//         BitBoard::ZERO,
//         Direction::Down
//       ),
//       Square(240)
//     );
//   }

//   #[test]
//   fn test_down_max() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(255),
//         BitBoard::ZERO,
//         Direction::Down
//       ),
//       Square(255)
//     );
//   }

//   #[test]
//   fn test_left_unblocked() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(79),
//         BitBoard::ZERO,
//         Direction::Left
//       ),
//       Square(64)
//     );
//   }

//   #[test]
//   fn test_left_blocked_by_occupied() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(79),
//         BitBoard::from(Square(68)),
//         Direction::Left
//       ),
//       Square(69)
//     );
//   }

//   #[test]
//   fn test_left_blocked_by_board() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard {
//           left_blocks: BitBoard::from(Square(68)),
//           ..BlockBoard::EMPTY
//         },
//         Square(79),
//         BitBoard::ZERO,
//         Direction::Left
//       ),
//       Square(69)
//     );
//   }

//   #[test]
//   fn test_left_blocked_by_multiple() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(79),
//         BitBoard::from(Square(68)) | BitBoard::from(Square(73)),
//         Direction::Left
//       ),
//       Square(74)
//     );
//   }

//   #[test]
//   fn test_left_zero() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(0),
//         BitBoard::ZERO,
//         Direction::Left
//       ),
//       Square(0)
//     );
//   }

//   #[test]
//   fn test_left_max() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(255),
//         BitBoard::ZERO,
//         Direction::Left
//       ),
//       Square(240)
//     );
//   }

//   #[test]
//   fn test_right_unblocked() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(64),
//         BitBoard::ZERO,
//         Direction::Right
//       ),
//       Square(79)
//     );
//   }

//   #[test]
//   fn test_right_blocked_by_occupied() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(64),
//         BitBoard::from(Square(70)),
//         Direction::Right
//       ),
//       Square(69)
//     );
//   }

//   #[test]
//   fn test_right_blocked_by_board() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard {
//           right_blocks: BitBoard::from(Square(70)),
//           ..BlockBoard::EMPTY
//         },
//         Square(64),
//         BitBoard::ZERO,
//         Direction::Right
//       ),
//       Square(69)
//     );
//   }

//   #[test]
//   fn test_right_blocked_by_multiple() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(64),
//         BitBoard::from(Square(70)) | BitBoard::from(Square(66)),
//         Direction::Right
//       ),
//       Square(65)
//     );
//   }

//   #[test]
//   fn test_right_zero() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(0),
//         BitBoard::ZERO,
//         Direction::Right
//       ),
//       Square(15)
//     );
//   }

//   #[test]
//   fn test_right_max() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard::EMPTY,
//         Square(255),
//         BitBoard::ZERO,
//         Direction::Right
//       ),
//       Square(255)
//     );
//   }

//   #[test]
//   fn test_move_on_block() {
//     assert_eq!(
//       get_move_destination(
//         &BlockBoard {
//           down_blocks: BitBoard::from(Square(0)),
//           ..BlockBoard::EMPTY
//         },
//         Square(00),
//         BitBoard::ZERO,
//         Direction::Down
//       ),
//       Square(240)
//     );
//   }
// }
