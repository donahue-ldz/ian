import { useCallback, useEffect, useState, type Dispatch, type SetStateAction } from "react";
import type {
  IanAction,
  IanEvent,
  MovementSpeed,
  Position,
} from "../protocol/generated";
import { getIanState, saveIanPosition, sendIanEvent } from "../lib/tauriBridge";
import { moveDesktopWindow } from "../lib/position";
import {
  createInitialIanViewState,
  expireRunAroundIfNeeded,
  reduceIanActions,
  type IanViewState,
} from "./ianActions";

export function useIanActions() {
  const [viewState, setViewState] = useState<IanViewState>(() =>
    createInitialIanViewState(),
  );

  const applyActions = useCallback((actions: IanAction[]) => {
    void applyActionSequence(actions, setViewState);
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

  return {
    viewState,
    sendEvent,
    savePosition,
  };
}

async function applyActionSequence(
  actions: IanAction[],
  setViewState: Dispatch<SetStateAction<IanViewState>>,
) {
  for (const action of actions) {
    setViewState((current) => reduceIanActions(current, [action]));

    if (action.type === "movement.move_to") {
      await moveDesktopWindow({ x: action.x, y: action.y });
      await delay(durationForSpeed(action.speed));
    }
  }
}

function durationForSpeed(speed: MovementSpeed): number {
  switch (speed) {
    case "fast":
      return 180;
    case "slow":
      return 420;
    default:
      return 280;
  }
}

function delay(durationMs: number): Promise<void> {
  return new Promise((resolve) => window.setTimeout(resolve, durationMs));
}
