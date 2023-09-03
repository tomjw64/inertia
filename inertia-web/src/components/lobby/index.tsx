import { PlayerName } from 'inertia-core';

export const Lobby = ({
  roomId,
  players,
}: {
  roomId: number;
  players: PlayerName[];
}) => {
  return <span>{JSON.stringify({ roomId, players })}</span>;
};
