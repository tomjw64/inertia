import { RoomData } from 'inertia-core';
import { useEffect, useRef, useState } from 'preact/hooks';
import { Lobby } from '../../components/lobby';
import { getOrCreatePlayerName } from '../../utils/storage';

const RoomStateType = {
  LOBBY: 'Lobby',
  ROUND_SUMMARY: 'RoundSummary',
  ROUND_START: 'RoundStart',
  ROUND_BIDDING: 'RoundBidding',
  ROUND_SOLVING: 'RoundSolving',
} as const;

const WS_CONNECTION_URL = 'ws://127.0.0.1:8001/ws';

const buildDefaultRoomData = (roomId: number): RoomData => ({
  room_id: roomId,
  players: {},
  player_scores: {},
  players_connected: {},
  round_number: 0,
  state: {
    type: RoomStateType.LOBBY,
  },
});

export const Room = ({ roomId: roomIdString }: { roomId: string }) => {
  const websocket = useRef<WebSocket | null>(null);
  const roomId = parseInt(roomIdString);

  const [roomData, setRoomData] = useState<RoomData>(
    buildDefaultRoomData(roomId)
  );

  useEffect(() => {
    websocket.current = new WebSocket(WS_CONNECTION_URL);
    const ws = websocket.current;
    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          player_name: getOrCreatePlayerName(),
          player_id: Math.floor(Math.random() * 1000),
          player_reconnect_key: 1,
          room_id: roomId,
        })
      );
    };
    ws.onmessage = (msg: MessageEvent<string>) => {
      setRoomData(JSON.parse(msg.data));
    };
    return () => {
      ws.close();
    };
  }, [roomId]);

  const {
    players,
    state,
    // player_scores: playerScores,
    // round_number: roundNumber,
    // data_version: dataVersion,
  } = roomData;

  if (state.type === RoomStateType.LOBBY) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (state.type === RoomStateType.ROUND_SUMMARY) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (state.type === RoomStateType.ROUND_START) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (state.type === RoomStateType.ROUND_BIDDING) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (state.type === RoomStateType.ROUND_SOLVING) {
    return <Lobby {...{ roomId, players }} />;
  }

  return <span>{'Error: Unknown Room State'}</span>;
};
