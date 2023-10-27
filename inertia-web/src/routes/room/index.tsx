import { RoomState, ToClientMessage } from 'inertia-core';
import { useEffect, useRef, useState } from 'preact/hooks';
import { RoundSummary } from '../../components/round-summary';
import {
  getOrCreatePlayerId,
  getOrCreatePlayerName,
} from '../../utils/storage';
import { RoomWebSocket } from '../../utils/ws';
import { RoundBidding } from '../../components/round-bidding';
import { RoundSolving } from '../../components/round-solving';

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

  const [countdownTimeLeft, setCountdownTimeLeft] = useState<number | null>(
    null
  );

  useEffect(() => {
    websocket.current = new RoomWebSocket();
    const ws = websocket.current;
    ws.onOpen(() => {
      ws.send({
        type: 'Join',
        content: {
          player_name: getOrCreatePlayerName(),
          player_id: getOrCreatePlayerId(),
          player_reconnect_key: 1,
          room_id: roomId,
        },
      });
    });
    ws.onMessage((msg: ToClientMessage) => {
      if (msg.type === 'RoomUpdate') {
        setRoomState(msg.content);
      } else if (msg.type === 'CountdownUpdate') {
        setCountdownTimeLeft(msg.content.server_time_left_millis);
      }
    });
    return () => {
      ws.close();
    };
  }, [roomId]);

  const withWs = (f: (ws: RoomWebSocket) => void) => {
    const ws = websocket.current;
    if (ws == null) {
      return;
    }
    f(ws);
  };

  const onStartRound = () =>
    withWs((ws) => {
      ws.send({
        type: 'StartRound',
      });
    });

  const onBid = (bid: number) =>
    withWs((ws) => {
      ws.send({
        type: 'Bid',
        content: {
          bid_value: bid,
        },
      });
    });

  if (roomState.type === RoomStateType.ROUND_SUMMARY) {
    return (
      <RoundSummary state={roomState.content} onStartRound={onStartRound} />
    );
  }

  if (roomState.type === RoomStateType.ROUND_START) {
    return (
      <RoundBidding
        state={roomState.content}
        onBid={onBid}
        countdownTimeLeft={countdownTimeLeft ?? 0}
      />
    );
  }

  if (roomState.type === RoomStateType.ROUND_BIDDING) {
    return (
      <RoundBidding
        state={roomState.content}
        onBid={onBid}
        countdownTimeLeft={countdownTimeLeft ?? 0}
      />
    );
  }

  if (roomState.type === RoomStateType.ROUND_SOLVING) {
    return (
      <RoundSolving
        state={roomState.content}
        countdownTimeLeft={countdownTimeLeft ?? 0}
      />
    );
  }

  if (roomState.type === RoomStateType.NONE) {
    return <>Nothing here.</>;
  }

  if (roomState.type === RoomStateType.CLOSED) {
    roomState.content;
    return <>Room closed.</>;
  }

  return <span>Unknown Room State</span>;
};
