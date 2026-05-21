import { useEffect, useState } from "react";
import { IanStage } from "./renderer/IanStage";
import { loadPetResourcePack, type PetResourcePack } from "./resources/resourceLoader";
import { useIanActions } from "./state/useIanActions";

const TICK_INTERVAL_MS = 15_000;

export default function App() {
  const [resourcePack, setResourcePack] = useState<PetResourcePack | null>(null);
  const { savePosition, sendEvent, viewState } = useIanActions();

  useEffect(() => {
    void loadPetResourcePack("ian-alpaca").then(setResourcePack);
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
      onDragEnd={(point) => {
        void savePosition(point);
      }}
    />
  );
}
