import { RoomData, ToClientMessage } from 'inertia-core';
import { useEffect, useRef, useState } from 'preact/hooks';
import { Lobby } from '../../components/lobby';
import { getOrCreatePlayerName } from '../../utils/storage';
import { RoomWebSocket } from '../../utils/ws';

const RoomStateType = {
  LOBBY: 'Lobby',
  ROUND_SUMMARY: 'RoundSummary',
  ROUND_START: 'RoundStart',
  ROUND_BIDDING: 'RoundBidding',
  ROUND_SOLVING: 'RoundSolving',
} as const;

const buildDefaultRoomData = (roomId: number): RoomData => ({
  room_id: roomId,
  players: {},
  player_scores: {},
  players_connected: {},
  round_number: 0,
  state: {
    type: RoomStateType.ROUND_SUMMARY,
  },
});

export const Room = ({ roomId: roomIdString }: { roomId: string }) => {
  const websocket = useRef<RoomWebSocket | null>(null);
  const roomId = parseInt(roomIdString);

  const [roomData, setRoomData] = useState<RoomData>(
    buildDefaultRoomData(roomId)
  );

  useEffect(() => {
    websocket.current = new RoomWebSocket();
    const ws = websocket.current;
    ws.onOpen(() => {
      ws.send({
        type: 'Join',
        content: {
          player_name: getOrCreatePlayerName(),
          player_id: Math.floor(Math.random() * 1000),
          player_reconnect_key: 1,
          room_id: roomId,
        },
      });
    });
    ws.onMessage((msg: ToClientMessage) => {
      console.log(msg);
      setRoomData(msg.content);
    });
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
