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

export const getPlayerName = (): string => {
  const storedPlayerName = localStorage.getItem(KEY_PLAYER_NAME);
  if (storedPlayerName) {
    return storedPlayerName;
  }
  const playerName = generatePlayerName();
  savePlayerName(playerName);
  return playerName;
};

export const savePlayerName = (name: string) => {
  localStorage.setItem(KEY_PLAYER_NAME, name);
};

export const getPlayerId = (): number => {
  const storedPlayerId = localStorage.getItem(KEY_PLAYER_ID);
  if (storedPlayerId) {
    return parseInt(storedPlayerId, 10);
  }
  const playerId = generatePlayerId();
  localStorage.setItem(KEY_PLAYER_ID, playerId.toString());
  return playerId;
};

export const getPlayerReconnectKey = (): number => {
  const storedPlayerReconnectKey = localStorage.getItem(
    KEY_PLAYER_RECONNECT_KEY,
  );
  if (storedPlayerReconnectKey) {
    return parseInt(storedPlayerReconnectKey, 10);
  }
  const playerReconnectKey = generatePlayerReconnectKey();
  localStorage.setItem(KEY_PLAYER_RECONNECT_KEY, playerReconnectKey.toString());
  return playerReconnectKey;
};
