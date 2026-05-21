import type { IanAction, MovementSpeed, Position } from "../protocol/generated";
import { formatBubbleText } from "../renderer/bubbleModel";

const DEFAULT_BUBBLE_DURATION_MS = 2400;

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
    visibleUntil: number | null;
  };
  isBubbleInputActive: boolean;
  position: Position;
  movementTarget: (Position & { speed: MovementSpeed }) | null;
  appearanceScale: number;
  visualEffect: { name: string; intensity: string; visibleUntil: number } | null;
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
      visibleUntil: null,
    },
    isBubbleInputActive: false,
    position: { x: 0, y: 0 },
    movementTarget: null,
    appearanceScale: 1,
    visualEffect: null,
    lastMovementAt: 0,
    runAroundUntil: 0,
  };
}

export function viewStateForWindowContent(
  state: IanViewState,
  isDesktopWindow: boolean,
): IanViewState {
  if (!isDesktopWindow) {
    return state;
  }

  return {
    ...state,
    position: { x: 0, y: 0 },
    movementTarget: state.movementTarget
      ? { ...state.movementTarget, x: 0, y: 0 }
      : null,
  };
}

export function reduceIanActions(
  state: IanViewState,
  actions: IanAction[],
  now = Date.now(),
): IanViewState {
  const hasStateSync = actions.some((action) => action.type === "state.sync");
  const willEndBubbleInput = actions.some(
    (action) =>
      action.type === "state.sync" && !action.state.is_bubble_input_active,
  );

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
        if (next.isBubbleInputActive && hasStateSync && !willEndBubbleInput) {
          return next;
        }

        return {
          ...next,
          bubble: {
            isOpen: true,
            text: formatBubbleText(action.text),
            mood: action.mood ?? null,
            visibleUntil:
              now + (action.duration_ms ?? DEFAULT_BUBBLE_DURATION_MS),
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
            text: null,
            visibleUntil: null,
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
      case "behavior.zoomies":
        return {
          ...next,
          animation: {
            name: "zoomies",
            loop: true,
          },
          behavior: "zooming",
          runAroundUntil: now + action.duration_ms,
        };
      case "effect.play":
        return {
          ...next,
          visualEffect: {
            name: action.name,
            intensity: action.intensity,
            visibleUntil: now + action.duration_ms,
          },
        };
      case "appearance.scale_to":
        return {
          ...next,
          appearanceScale: action.scale,
        };
      case "playful.state":
      case "playful.diagnostic":
        return next;
      case "state.sync":
        return {
          ...next,
          animation: {
            name: action.state.current_animation,
            loop: true,
          },
          behavior: action.state.current_behavior,
          isBubbleInputActive: action.state.is_bubble_input_active,
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

export function expireBubbleIfNeeded(
  state: IanViewState,
  now = Date.now(),
): IanViewState {
  if (
    !state.bubble.isOpen ||
    state.bubble.visibleUntil === null ||
    state.isBubbleInputActive ||
    now < state.bubble.visibleUntil
  ) {
    return state;
  }

  return {
    ...state,
    bubble: {
      isOpen: false,
      text: null,
      mood: null,
      visibleUntil: null,
    },
  };
}

export function expireRunAroundIfNeeded(
  state: IanViewState,
  now = Date.now(),
): IanViewState {
  if (
    !["running", "zooming"].includes(state.behavior) ||
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
    case "rest":
      return "resting";
    case "walk":
      return "walking";
    case "happy":
      return "happy";
    case "zoomies":
      return "zooming";
    case "sleep":
      return "sleeping";
    default:
      return current;
  }
}
