import type { IanAction } from "../protocol/generated";

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
        };
      case "movement.move_to":
        return next;
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
