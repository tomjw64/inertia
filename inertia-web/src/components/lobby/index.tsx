import { PlayerName } from 'inertia-core';

export const Lobby = ({
  roomId,
  players,
}: {
  roomId: number;
  players: PlayerName[];
}) => {
  return (
    <div>
      <span>{JSON.stringify({ roomId, players })}</span>
      <button>Start Game</button>
    </div>
  );
};
