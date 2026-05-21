import { useEffect, useState } from "react";
import { IanStage } from "./renderer/IanStage";
import { loadPetResourcePack, type PetResourcePack } from "./resources/resourceLoader";
import { useIanActions } from "./state/useIanActions";

export default function App() {
  const [resourcePack, setResourcePack] = useState<PetResourcePack | null>(null);
  const ian = useIanActions();

  useEffect(() => {
    void loadPetResourcePack("ian-alpaca").then(setResourcePack);
    void ian.sendEvent({ type: "app.started" });
  }, []);

  return (
    <IanStage
      resourcePack={resourcePack}
      viewState={ian.viewState}
      onIanClick={(point) => {
        void ian.sendEvent({ type: "mouse.click", ...point });
      }}
      onIanDoubleClick={(point) => {
        void ian.sendEvent({ type: "mouse.double_click", ...point });
      }}
      onDragEnd={(point) => {
        void ian.savePosition(point);
      }}
    />
  );
}
