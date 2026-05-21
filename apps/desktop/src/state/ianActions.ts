import type { IanAction, MovementSpeed, Position } from "../protocol/generated";

export type IanViewState = {
  animation: {
    name: string;
    loop: boolean;
  };
  behavior: string;
  bubble: {
    isOpen: boolean;
    text: string | null;
    mood: string | null;
  };
  position: Position;
  movementTarget: (Position & { speed: MovementSpeed }) | null;
  lastMovementAt: number;
  runAroundUntil: number;
};

export function createInitialIanViewState(): IanViewState {
  return {
    animation: {
      name: "idle",
      loop: true,
    },
    behavior: "idle",
    bubble: {
      isOpen: false,
      text: null,
      mood: null,
    },
    position: { x: 0, y: 0 },
    movementTarget: null,
    lastMovementAt: 0,
    runAroundUntil: 0,
  };
}

export function reduceIanActions(
  state: IanViewState,
  actions: IanAction[],
  now = Date.now(),
): IanViewState {
  return actions.reduce<IanViewState>((next, action) => {
    switch (action.type) {
      case "animation.play":
        return {
          ...next,
          animation: {
            name: action.name,
            loop: action.looped,
          },
          behavior: behaviorForAnimation(next.behavior, action.name),
        };
      case "speech.show":
        return {
          ...next,
          bubble: {
            isOpen: true,
            text: action.text,
            mood: action.mood ?? null,
          },
        };
      case "bubble.open":
        return {
          ...next,
          bubble: {
            ...next.bubble,
            isOpen: true,
          },
        };
      case "bubble.close":
        return {
          ...next,
          bubble: {
            ...next.bubble,
            isOpen: false,
          },
        };
      case "behavior.run_around":
        return {
          ...next,
          animation: {
            name: "run",
            loop: true,
          },
          behavior: "running",
          runAroundUntil: now + action.duration_ms,
        };
      case "state.sync":
        return {
          ...next,
          animation: {
            name: action.state.current_animation,
            loop: true,
          },
          behavior: action.state.current_behavior,
          position: action.state.position,
        };
      case "movement.move_to":
        return {
          ...next,
          position: {
            x: action.x,
            y: action.y,
          },
          movementTarget: {
            x: action.x,
            y: action.y,
            speed: action.speed,
          },
          lastMovementAt: now,
        };
    }
  }, state);
}

export function expireRunAroundIfNeeded(
  state: IanViewState,
  now = Date.now(),
): IanViewState {
  if (
    state.behavior !== "running" ||
    state.runAroundUntil === 0 ||
    now < state.runAroundUntil
  ) {
    return state;
  }

  return {
    ...state,
    animation: {
      name: "idle",
      loop: true,
    },
    behavior: "idle",
    runAroundUntil: 0,
  };
}

function behaviorForAnimation(current: string, animation: string): string {
  switch (animation) {
    case "idle":
      return "idle";
    case "walk":
      return "walking";
    case "happy":
      return "happy";
    case "sleep":
      return "sleeping";
    default:
      return current;
  }
}
