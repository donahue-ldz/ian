import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type Dispatch,
  type SetStateAction,
} from "react";
import type {
  IanAction,
  IanEvent,
  MovementSpeed,
  Position,
} from "../protocol/generated";
import {
  getIanState,
  resetIanPosition,
  saveIanPosition,
  sendIanEvent,
} from "../lib/tauriBridge";
import { getDesktopWindowPosition, moveDesktopWindow } from "../lib/position";
import {
  createInitialIanViewState,
  expireBubbleIfNeeded,
  expireRunAroundIfNeeded,
  reduceIanActions,
  type IanViewState,
} from "./ianActions";

export function useIanActions() {
  const [viewState, setViewState] = useState<IanViewState>(() =>
    createInitialIanViewState(),
  );
  const movementSequenceGuard = useRef(createMovementSequenceGuard());

  const applyActions = useCallback((actions: IanAction[]) => {
    void applyActionSequence(actions, setViewState, movementSequenceGuard.current);
  }, []);

  useEffect(() => {
    void getIanState().then((state) => {
      applyActions([{ type: "state.sync", state }]);
    });
  }, [applyActions]);

  useEffect(() => {
    if (viewState.behavior !== "running" || viewState.runAroundUntil === 0) {
      return;
    }

    const timeout = window.setTimeout(() => {
      setViewState((current) => expireRunAroundIfNeeded(current));
    }, Math.max(viewState.runAroundUntil - Date.now(), 0));

    return () => window.clearTimeout(timeout);
  }, [viewState.behavior, viewState.runAroundUntil]);

  useEffect(() => {
    if (!viewState.bubble.isOpen || viewState.bubble.visibleUntil === null) {
      return;
    }

    const timeout = window.setTimeout(() => {
      setViewState((current) => expireBubbleIfNeeded(current));
    }, Math.max(viewState.bubble.visibleUntil - Date.now(), 0));

    return () => window.clearTimeout(timeout);
  }, [viewState.bubble.isOpen, viewState.bubble.visibleUntil]);

  const sendEvent = useCallback(
    async (event: IanEvent) => {
      const actions = await sendIanEvent(event);
      applyActions(actions);
    },
    [applyActions],
  );

  const savePosition = useCallback(
    async (position: Position) => {
      const actions = await saveIanPosition(position);
      applyActions(actions);
    },
    [applyActions],
  );

  const resetPosition = useCallback(async () => {
    const actions = await resetIanPosition();
    applyActions(actions);
  }, [applyActions]);

  return {
    viewState,
    sendEvent,
    savePosition,
    resetPosition,
  };
}

async function applyActionSequence(
  actions: IanAction[],
  setViewState: Dispatch<SetStateAction<IanViewState>>,
  movementGuard: MovementSequenceGuard,
) {
  let movementIndex = 0;
  const isMovementBatch = actions.some(isMovementAction);
  const sequenceId = movementGuard.startBatch(actions);

  for (const action of actions) {
    if (isMovementBatch && !movementGuard.isCurrent(sequenceId)) {
      return;
    }

    setViewState((current) => reduceIanActions(current, [action]));

    if (action.type === "movement.move_to") {
      await moveDesktopWindowSmoothly(
        { x: action.x, y: action.y },
        action.speed,
        movementIndex,
        () => movementGuard.isCurrent(sequenceId),
      );
      movementIndex += 1;
    }
  }
}

async function moveDesktopWindowSmoothly(
  target: Position,
  speed: MovementSpeed,
  movementIndex: number,
  isCurrent = () => true,
) {
  const start = await getDesktopWindowPosition();
  if (!isCurrent()) {
    return;
  }

  if (!start) {
    await moveDesktopWindow(target);
    return;
  }

  const durationMs = durationForDesktopMovement(start, target, speed, movementIndex);
  const frames = planDesktopMovementFrames(start, target, speed, movementIndex);
  const frameDelayMs = Math.max(
    16,
    Math.round(durationMs / frames.length),
  );

  for (const frame of frames) {
    if (!isCurrent()) {
      return;
    }

    await moveDesktopWindow(frame);
    await delay(frameDelayMs);
  }
}

type MovementSequenceGuard = {
  startBatch: (actions: IanAction[]) => number;
  isCurrent: (sequenceId: number) => boolean;
};

export function createMovementSequenceGuard(): MovementSequenceGuard {
  let currentSequenceId = 0;

  return {
    startBatch(actions: IanAction[]) {
      if (actions.some(isMovementAction)) {
        currentSequenceId += 1;
      }

      return currentSequenceId;
    },
    isCurrent(sequenceId: number) {
      return sequenceId === currentSequenceId;
    },
  };
}

function isMovementAction(action: IanAction): boolean {
  return action.type === "movement.move_to";
}

export function durationForSpeed(
  speed: MovementSpeed,
  movementIndex = 0,
): number {
  const variation = [-25, 20, -10, 30, 0][movementIndex % 5] ?? 0;

  switch (speed) {
    case "fast":
      return 850 + variation;
    case "slow":
      return 1800 + variation;
    default:
      return 1250 + variation;
  }
}

export function durationForDesktopMovement(
  from: Position,
  to: Position,
  speed: MovementSpeed,
  movementIndex = 0,
): number {
  const distance = Math.hypot(to.x - from.x, to.y - from.y);
  const pixelsPerSecond = speed === "fast" ? 280 : speed === "slow" ? 55 : 140;
  const distanceDuration = Math.round((distance / pixelsPerSecond) * 1000);

  return Math.max(durationForSpeed(speed, movementIndex), distanceDuration);
}

export function planDesktopMovementFrames(
  from: Position,
  to: Position,
  speed: MovementSpeed,
  movementIndex = 0,
): Position[] {
  const frameCount = Math.max(
    12,
    Math.round(durationForDesktopMovement(from, to, speed, movementIndex) / 33),
  );

  return Array.from({ length: frameCount }, (_, index) => {
    const progress = (index + 1) / frameCount;
    const eased =
      progress < 0.5
        ? 2 * progress * progress
        : 1 - Math.pow(-2 * progress + 2, 2) / 2;

    if (index === frameCount - 1) {
      return to;
    }

    return {
      x: from.x + (to.x - from.x) * eased,
      y: from.y + (to.y - from.y) * eased,
    };
  });
}

function delay(durationMs: number): Promise<void> {
  return new Promise((resolve) => window.setTimeout(resolve, durationMs));
}
