import {
  generatePlayerId,
  generatePlayerName,
  generatePlayerReconnectKey,
} from './player-gen';

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
const KEY_PLAYER_RECONNECT_KEY = 'playerReconnectKey';

export const getOrCreatePlayerName = (): string => {
  const storedPlayerName = localStorage.getItem(KEY_PLAYER_NAME);
  if (storedPlayerName) {
    return storedPlayerName;
  }
  return generatePlayerName();
};

export const getOrCreatePlayerId = (): number => {
  const storedPlayerId = localStorage.getItem(KEY_PLAYER_ID);
  if (storedPlayerId) {
    return parseInt(storedPlayerId, 10);
  }
  return generatePlayerId();
};

export const getOrCreatePlayerReconnectKey = (): number => {
  const storedPlayerReconnectKey = localStorage.getItem(
    KEY_PLAYER_RECONNECT_KEY
  );
  if (storedPlayerReconnectKey) {
    return parseInt(storedPlayerReconnectKey, 10);
  }
  return generatePlayerReconnectKey();
};
