import { PlayerId, PlayerName } from 'inertia-core';

export const Lobby = ({
  roomId,
  players,
}: {
  roomId: number;
  players: Record<PlayerId, PlayerName>;
}) => {
  return (
    <div>
      <span>{JSON.stringify({ roomId, players })}</span>
      <button>Start Game</button>
    </div>
  );
};
