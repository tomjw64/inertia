import { WallGrid, WalledBoard, Position } from 'inertia-core';

export const emptyBoard = (): WalledBoard => {
  const vertical = [...Array(16)].map((_row) =>
    Array(15).fill(false)
  ) as WallGrid;
  const horizontal = [...Array(16)].map((_column) =>
    Array(15).fill(false)
  ) as WallGrid;
  return {
    vertical,
    horizontal,
  };
};

export const defaultPosition = (): Position => {
  return {
    walled_board: emptyBoard(),
    goal: 255,
    actor_squares: [0, 1, 2, 3],
  };
};
