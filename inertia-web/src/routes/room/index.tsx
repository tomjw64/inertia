import {
  RoomState,
  RoundSolving as RoundSolvingState,
  SolutionStep,
  ToClientMessage,
  WalledBoardPosition,
} from 'inertia-core';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { RoundSummary } from '../../components/round-summary';
import {
  getPlayerName,
  getPlayerReconnectKey,
  getPlayerId,
} from '../../utils/storage';
import { RoomWebSocket } from '../../utils/ws';
import { RoundBidding } from '../../components/round-bidding';
import { RoundSolving } from '../../components/round-solving';
import { defaultWalledBoardPosition } from '../../utils/board';
import { apply_solution } from 'inertia-wasm';
import { ACTOR_FLIP_ANIMATE_DURATION } from '../../components/board';

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

  const userPlayerId = useMemo(() => getPlayerId(), []);
  const userPlayerReconnectKey = useMemo(() => getPlayerReconnectKey(), []);
  const userPlayerName = useMemo(() => getPlayerName(), []);

  const [roomState, setRoomState] = useState<RoomState>({
    type: RoomStateType.NONE,
  });

  const [countdownTimeLeft, setCountdownTimeLeft] = useState<number | null>(
    null
  );

  const walledBoardPosition: WalledBoardPosition = useMemo(() => {
    if (roomState.type === 'None' || roomState.type === 'Closed') {
      return defaultWalledBoardPosition();
    } else if (roomState.type === 'RoundSummary') {
      return roomState.content.last_round_board ?? defaultWalledBoardPosition();
    } else {
      return roomState.content.board;
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [roomState.type]);

  const serverSolution: SolutionStep[] = useMemo(() => {
    if (roomState.type === 'RoundSolving') {
      return roomState.content.solution;
    } else if (roomState.type === 'RoundSummary') {
      return roomState.content.last_round_solution ?? [];
    } else {
      return [];
    }
  }, [roomState]);

  const [localSolution, setLocalSolution] = useState<SolutionStep[]>([]);

  const actorSquares = useMemo(() => {
    if (roomState.type === 'RoundSolving') {
      return apply_solution(walledBoardPosition, localSolution);
    } else if (roomState.type === 'RoundSummary') {
      return apply_solution(walledBoardPosition, serverSolution);
    } else {
      return walledBoardPosition.actor_squares;
    }
  }, [localSolution, roomState.type, serverSolution, walledBoardPosition]);

  const solver = (roomState.content as Partial<RoundSolvingState>)?.solver;
  useEffect(() => {
    setLocalSolution([]);
  }, [roomState.type, solver]);

  useEffect(() => {
    if (serverSolution.length > localSolution.length) {
      const stepToAdd = serverSolution[localSolution.length];
      const updated = [...localSolution, stepToAdd];
      setTimeout(() => {
        setLocalSolution(updated);
      }, ACTOR_FLIP_ANIMATE_DURATION + 0.1);
    }
  }, [serverSolution, localSolution]);

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

  const onMoveActor = (step: SolutionStep) => {
    const updated = [...localSolution, step];
    withWs((ws) => {
      ws.send({
        type: 'UpdateSolution',
        content: {
          solution: updated,
        },
      });
    });
    setLocalSolution(updated);
  };

  if (roomState.type === RoomStateType.ROUND_SUMMARY) {
    return (
      <RoundSummary
        state={roomState.content}
        userPlayerId={userPlayerId}
        actorSquares={actorSquares}
        onStartRound={onStartRound}
      />
    );
  }

  if (roomState.type === RoomStateType.ROUND_START) {
    return (
      <RoundBidding
        state={roomState.content}
        userPlayerId={userPlayerId}
        actorSquares={actorSquares}
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
        actorSquares={actorSquares}
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
        actorSquares={actorSquares}
        countdownTimeLeft={countdownTimeLeft ?? 0}
        onYieldSolve={onYieldSolve}
        onMoveActor={onMoveActor}
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
