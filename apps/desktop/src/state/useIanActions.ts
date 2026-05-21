import { useCallback, useEffect, useState } from "react";
import type { IanEvent, Position } from "../protocol/generated";
import { getIanState, saveIanPosition, sendIanEvent } from "../lib/tauriBridge";
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

  const applyActions = useCallback((actions: Parameters<typeof reduceIanActions>[1]) => {
    setViewState((current) => reduceIanActions(current, actions));
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
