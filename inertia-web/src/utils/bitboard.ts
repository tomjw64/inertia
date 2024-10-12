import { ExpandedBitBoard, Square } from 'inertia-core';
import { zip } from 'lodash';

export const empty = () => {
  return Array(256).fill(false) as ExpandedBitBoard;
};

export const union = (bitboards: ExpandedBitBoard[]) => {
  return bitboards.reduce(
    (acc, bitboard) =>
      zip(acc, bitboard).map(([a, b]) => a || b) as ExpandedBitBoard,
  );
};

export const fromSquares = (squares: Square[]) => {
  const board = empty();
  for (const square of squares) {
    board[square] = true;
  }
  return board;
};

export const removeSquares = (
  bitboard: ExpandedBitBoard,
  squares: Square[],
) => {
  const clone = [...bitboard];
  for (const square of squares) {
    clone[square] = false;
  }
  return clone;
};
