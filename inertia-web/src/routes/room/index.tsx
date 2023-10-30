import { RoomState, ToClientMessage } from 'inertia-core';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { RoundSummary } from '../../components/round-summary';
import {
  getOrCreatePlayerId,
  getOrCreatePlayerName,
  getOrCreatePlayerReconnectKey,
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

export const Room = ({ roomId: roomIdString }: { roomId: string }) => {
  const websocket = useRef<RoomWebSocket | null>(null);
  const roomId = parseInt(roomIdString);

  const userPlayerId = useMemo(() => getOrCreatePlayerId(), []);
  const userPlayerReconnectKey = useMemo(
    () => getOrCreatePlayerReconnectKey(),
    []
  );
  const userPlayerName = useMemo(() => getOrCreatePlayerName(), []);

  const [roomState, setRoomState] = useState<RoomState>({
    type: RoomStateType.NONE,
  });

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
          player_name: userPlayerName,
          player_id: userPlayerId,
          player_reconnect_key: userPlayerReconnectKey,
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
  }, [userPlayerId, userPlayerName, userPlayerReconnectKey, roomId]);

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

  const onReadyBid = () =>
    withWs((ws) => {
      ws.send({
        type: 'ReadyBid',
      });
    });

  const onUnreadyBid = () =>
    withWs((ws) => {
      ws.send({
        type: 'UnreadyBid',
      });
    });

  const onYieldSolve = () =>
    withWs((ws) => {
      ws.send({
        type: 'YieldSolve',
      });
    });

  if (roomState.type === RoomStateType.ROUND_SUMMARY) {
    return (
      <RoundSummary
        state={roomState.content}
        userPlayerId={userPlayerId}
        onStartRound={onStartRound}
      />
    );
  }

  if (roomState.type === RoomStateType.ROUND_START) {
    return (
      <RoundBidding
        state={roomState.content}
        userPlayerId={userPlayerId}
        onBid={onBid}
        onReadyBid={onReadyBid}
        onUnreadyBid={onUnreadyBid}
        countdownTimeLeft={countdownTimeLeft ?? 0}
      />
    );
  }

  if (roomState.type === RoomStateType.ROUND_BIDDING) {
    return (
      <RoundBidding
        state={roomState.content}
        userPlayerId={userPlayerId}
        onBid={onBid}
        onReadyBid={onReadyBid}
        onUnreadyBid={onUnreadyBid}
        countdownTimeLeft={countdownTimeLeft ?? 0}
      />
    );
  }

  if (roomState.type === RoomStateType.ROUND_SOLVING) {
    return (
      <RoundSolving
        state={roomState.content}
        userPlayerId={userPlayerId}
        countdownTimeLeft={countdownTimeLeft ?? 0}
        onYieldSolve={onYieldSolve}
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
