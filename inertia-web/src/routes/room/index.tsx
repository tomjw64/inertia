import { RoomState, ToClientMessage } from 'inertia-core';
import { useEffect, useRef, useState } from 'preact/hooks';
import { RoundSummary } from '../../components/round-summary';
import { getOrCreatePlayerName } from '../../utils/storage';
import { RoomWebSocket } from '../../utils/ws';

const RoomStateType = {
  NONE: 'None',
  CLOSED: 'Closed',
  ROUND_SUMMARY: 'RoundSummary',
  ROUND_START: 'RoundStart',
  ROUND_BIDDING: 'RoundBidding',
  ROUND_SOLVING: 'RoundSolving',
} as const;

const buildDefaultRoomData = (_roomId: number): RoomState => ({
  type: RoomStateType.NONE,
});

export const Room = ({ roomId: roomIdString }: { roomId: string }) => {
  const websocket = useRef<RoomWebSocket | null>(null);
  const roomId = parseInt(roomIdString);

  const [roomState, setRoomState] = useState<RoomState>(
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
      setRoomState(msg.content);
    });
    return () => {
      ws.close();
    };
  }, [roomId]);

  if (roomState.type === RoomStateType.ROUND_SUMMARY) {
    return <RoundSummary data={roomState.content} />;
  }

  // if (roomState.type === RoomStateType.ROUND_START) {
  //   return <RoundSummary {...{ roomId, players }} />;
  // }

  // if (roomState.type === RoomStateType.ROUND_BIDDING) {
  //   return <RoundSummary {...{ roomId, players }} />;
  // }

  // if (roomState.type === RoomStateType.ROUND_SOLVING) {
  //   return <RoundSummary {...{ roomId, players }} />;
  // }

  if (roomState.type === RoomStateType.NONE) {
    return <>Nothing here.</>;
  }

  if (roomState.type === RoomStateType.CLOSED) {
    roomState.content;
    return <>Room closed.</>;
  }

  return <span>Unknown Room State</span>;
};
