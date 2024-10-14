import {
  apply_solution,
  RoomState,
  RoundSolving as RoundSolvingState,
  SolutionStep,
  ToClientMessage,
} from 'inertia-core';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { ErrorPage } from '../../components/error-page';
import { AppControls } from '../../components/room-controls';
import { RoundBidding } from '../../components/round-bidding';
import { RoundSolving } from '../../components/round-solving';
import { RoundSummary } from '../../components/round-summary';
import { ACTOR_FLIP_ANIMATE_DURATION } from '../../components/simple-board';
import { Starfield } from '../../components/starfield';
import { parseDifficulty } from '../../constants/difficulty';
import { defaultPosition } from '../../utils/board';
import { RoomWebSocket } from '../../utils/room-websocket';
import {
  getPlayerId,
  getPlayerName,
  getPlayerReconnectKey,
} from '../../utils/storage';
import { useThrottledQueue } from '../../utils/hooks/use-throttled-queue';

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
    null,
  );

  const initialPosition = useMemo(() => {
    if (roomState.type === 'None' || roomState.type === 'Closed') {
      return defaultPosition();
    } else if (roomState.type === 'RoundSummary') {
      return roomState.content.last_round_board ?? defaultPosition();
    } else {
      return roomState.content.board;
    }
    // Fine because we know the board only ever changes when the state type
    // changes and we don't want to put the board data here because useMemo uses
    // shallow equality.
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
  const [appliedServerSolution, setAppliedServerSolution] = useState<
    SolutionStep[]
  >([]);
  const [localSolution, setLocalSolution] = useState<SolutionStep[]>([]);

  const solver = (roomState as { content?: RoundSolvingState }).content?.solver;
  const isUserSolver = solver === userPlayerId;
  const useLocalSolution = isUserSolver;
  const solutionToApply = useLocalSolution
    ? localSolution
    : appliedServerSolution;

  const {
    clearQueue: clearAnimationQueue,
    processQueue: processAnimationQueue,
    setQueue: setAnimationQueue,
  } = useThrottledQueue<SolutionStep>({
    throttleMs: (ACTOR_FLIP_ANIMATE_DURATION + 0.1) * 1000,
    onData: (data) => {
      setAppliedServerSolution((previous) => [...previous, data]);
    },
  });

  useEffect(() => {
    // If the solution lengths are equal, nothing to do.
    if (serverSolution.length === appliedServerSolution.length) {
      return;
    }
    // If the solution to apply is smaller, apply it immediately
    if (serverSolution.length < appliedServerSolution.length) {
      clearAnimationQueue();
      setAppliedServerSolution(serverSolution);
      return;
    }
    // Otherwise, there are steps we haven't displayed yet. Queue them for animation
    setAnimationQueue(serverSolution.slice(appliedServerSolution.length));
    processAnimationQueue();
  }, [
    appliedServerSolution,
    serverSolution,
    clearAnimationQueue,
    processAnimationQueue,
    setAnimationQueue,
  ]);

  useEffect(() => {
    setLocalSolution([]);
  }, [roomState.type, solver]);

  const actorSquares = useMemo(() => {
    return apply_solution(initialPosition, solutionToApply);
  }, [solutionToApply, initialPosition]);

  const position = useMemo(
    () => ({
      walled_board: initialPosition.walled_board,
      goal: initialPosition.goal,
      actor_squares: actorSquares,
    }),
    [actorSquares, initialPosition.goal, initialPosition.walled_board],
  );

  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const minDifficulty = parseDifficulty(urlParams.get('minDifficulty'));
    const maxDifficulty = parseDifficulty(urlParams.get('maxDifficulty'));

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
          min_difficulty: minDifficulty ?? null,
          max_difficulty: maxDifficulty ?? null,
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

  const getRoundStateComponent = () => {
    if (roomState.type === RoomStateType.ROUND_SUMMARY) {
      return (
        <RoundSummary
          state={roomState.content}
          userPlayerId={userPlayerId}
          position={position}
          onStartRound={onStartRound}
        />
      );
    }

    if (roomState.type === RoomStateType.ROUND_START) {
      return (
        <RoundBidding
          state={roomState.content}
          userPlayerId={userPlayerId}
          position={position}
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
          position={position}
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
          position={position}
          countdownTimeLeft={countdownTimeLeft ?? 0}
          onYieldSolve={onYieldSolve}
          onMoveActor={onMoveActor}
        />
      );
    }

    if (roomState.type === RoomStateType.NONE) {
      return <ErrorPage>Nothing here.</ErrorPage>;
    }

    if (roomState.type === RoomStateType.CLOSED) {
      return <ErrorPage>Room closed.</ErrorPage>;
    }

    return <ErrorPage>Unknown state.</ErrorPage>;
  };

  const roundState = getRoundStateComponent();

  return (
    <>
      <Starfield numStars={500} speed={0.5} />
      <AppControls />
      {roundState}
    </>
  );
};
