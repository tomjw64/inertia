import { generatePlayerName } from './player-name';

export const getOrCreatePlayer = (): {
  playerName: string;
  playerId: number;
  playerReconnectKey: number;
} => {
  return {
    playerName: '',
    playerId: 0,
    playerReconnectKey: 0,
  };
};

const KEY_PLAYER_NAME = 'playerName';
const KEY_PLAYER_ID = 'playerId';
const KEY_PLAYER_RECONNECT = 'playerReconnectKey';

export const getOrCreatePlayerName = (): string => {
  const storedPlayerName = localStorage.getItem(KEY_PLAYER_NAME);
  if (storedPlayerName) {
    return storedPlayerName;
  }
  return generatePlayerName();
};
