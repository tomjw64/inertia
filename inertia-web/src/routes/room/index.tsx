import { RoomData } from 'inertia-core';
import { useRef, useState } from 'preact/hooks';
import { Lobby } from '../../components/lobby';

const RoomStateType = {
  LOBBY: 'Lobby',
  ROUND_SUMMARY: 'RoundSummary',
  ROUND_START: 'RoundStart',
  ROUND_BIDDING: 'RoundBidding',
  ROUND_SOLVING: 'RoundSolving',
} as const;

const buildDefaultRoomData = (roomId: number): RoomData => ({
  room_id: roomId,
  players: [],
  player_scores: [],
  round_number: 0,
  data_version: 0,
  state: {
    type: RoomStateType.LOBBY,
  },
});

export const Room = ({ roomId }: { roomId: number }) => {
  const { current: _ws } = useRef<WebSocket>();
  const [roomData, _setRoomData] = useState<RoomData>(
    buildDefaultRoomData(roomId)
  );

  const {
    players,
    // player_scores: playerScores,
    // round_number: roundNumber,
    // data_version: dataVersion,
  } = roomData;

  if (roomData.state.type === RoomStateType.LOBBY) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (roomData.state.type === RoomStateType.ROUND_SUMMARY) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (roomData.state.type === RoomStateType.ROUND_START) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (roomData.state.type === RoomStateType.ROUND_BIDDING) {
    return <Lobby {...{ roomId, players }} />;
  }

  if (roomData.state.type === RoomStateType.ROUND_SOLVING) {
    return <Lobby {...{ roomId, players }} />;
  }

  return <span>{'Error: Unknown Room State'}</span>;
};
