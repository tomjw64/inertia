const ACTOR_COLORS = ['red', 'blue', 'green', 'yellow'] as const;

export type ActorColor = (typeof ACTOR_COLORS)[number];

export const getActorColor = (actorIndex: number) => {
  return ACTOR_COLORS[actorIndex]!;
};

export const getActorIndex = (color: ActorColor) => {
  return ACTOR_COLORS.indexOf(color);
};
