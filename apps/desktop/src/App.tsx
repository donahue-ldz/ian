import { useEffect, useState } from "react";
import type { BehaviorMode } from "./protocol/generated";
import { IanStage } from "./renderer/IanStage";
import { loadPetResourcePack, type PetResourcePack } from "./resources/resourceLoader";
import { getIanSettings, saveBehaviorMode } from "./lib/tauriBridge";
import { useIanActions } from "./state/useIanActions";

const TICK_INTERVAL_MS = 15_000;

export default function App() {
  const [resourcePack, setResourcePack] = useState<PetResourcePack | null>(null);
  const [behaviorMode, setBehaviorMode] = useState<BehaviorMode>("normal");
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { savePosition, sendEvent, viewState } = useIanActions();

  useEffect(() => {
    void loadPetResourcePack("ian-alpaca").then(setResourcePack);
    void getIanSettings().then((state) => setBehaviorMode(state.behavior_mode));
    void sendEvent({ type: "app.started" });
  }, [sendEvent]);

  useEffect(() => {
    const interval = window.setInterval(() => {
      void sendEvent({ type: "time.tick", now_ms: Date.now() });
    }, TICK_INTERVAL_MS);

    return () => window.clearInterval(interval);
  }, [sendEvent]);

  return (
    <IanStage
      resourcePack={resourcePack}
      viewState={viewState}
      behaviorMode={behaviorMode}
      isSettingsOpen={isSettingsOpen}
      onIanClick={(point) => {
        void sendEvent({ type: "mouse.click", ...point });
      }}
      onIanDoubleClick={(point) => {
        void sendEvent({ type: "mouse.double_click", ...point });
      }}
      onIanNear={(point) => {
        void sendEvent({ type: "mouse.near", ...point });
      }}
      onSubmitMessage={(text) => {
        void sendEvent({ type: "dialogue.user_message", text });
      }}
      onSettingsToggle={() => setIsSettingsOpen((current) => !current)}
      onSettingsClose={() => setIsSettingsOpen(false)}
      onBehaviorModeChange={(mode) => {
        setBehaviorMode(mode);
        void saveBehaviorMode(mode).then((state) => {
          setBehaviorMode(state.behavior_mode);
        });
      }}
      onDragEnd={(point) => {
        void savePosition(point);
      }}
    />
  );
}
