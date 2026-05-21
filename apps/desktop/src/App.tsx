import { useEffect, useState } from "react";
import type { BehaviorMode } from "./protocol/generated";
import { IanStage } from "./renderer/IanStage";
import { loadPetResourcePack, type PetResourcePack } from "./resources/resourceLoader";
import {
  getIanSettings,
  saveBehaviorMode,
  saveCapabilityEnabled,
  saveRemindersEnabled,
} from "./lib/tauriBridge";
import { moveDesktopWindow } from "./lib/position";
import { useIanActions } from "./state/useIanActions";

const TICK_INTERVAL_MS = 15_000;

export default function App() {
  const [resourcePack, setResourcePack] = useState<PetResourcePack | null>(null);
  const [behaviorMode, setBehaviorMode] = useState<BehaviorMode>("normal");
  const [remindersEnabled, setRemindersEnabled] = useState(true);
  const [capabilities, setCapabilities] = useState(createCapabilityState());
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { savePosition, sendEvent, viewState } = useIanActions();

  useEffect(() => {
    void loadPetResourcePack("ian-alpaca").then(setResourcePack);
    void getIanSettings().then((state) => {
      setBehaviorMode(state.behavior_mode);
      setRemindersEnabled(state.reminders_enabled);
      setCapabilities(capabilityStateFromIanState(state));
      void moveDesktopWindow(state.position);
    });
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
      remindersEnabled={remindersEnabled}
      byomEnabled={capabilities.byom}
      gitMetadataEnabled={capabilities.gitMetadata}
      buildTestEventsEnabled={capabilities.buildTestEvents}
      keyboardRhythmEnabled={capabilities.keyboardRhythm}
      activeAppPresenceEnabled={capabilities.activeAppPresence}
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
      onRemindersEnabledChange={(enabled) => {
        setRemindersEnabled(enabled);
        void saveRemindersEnabled(enabled).then((state) => {
          setRemindersEnabled(state.reminders_enabled);
        });
      }}
      onCapabilityEnabledChange={(capability, enabled) => {
        setCapabilities((current) => ({
          ...current,
          [capabilityToStateKey(capability)]: enabled,
        }));
        void saveCapabilityEnabled(capability, enabled).then((state) => {
          setCapabilities(capabilityStateFromIanState(state));
        });
      }}
      onDragEnd={(point) => {
        void savePosition(point);
      }}
    />
  );
}

function createCapabilityState() {
  return {
    byom: false,
    gitMetadata: false,
    buildTestEvents: false,
    keyboardRhythm: false,
    activeAppPresence: false,
  };
}

function capabilityStateFromIanState(state: Awaited<ReturnType<typeof getIanSettings>>) {
  return {
    byom: state.byom_enabled,
    gitMetadata: state.git_metadata_enabled,
    buildTestEvents: state.build_test_events_enabled,
    keyboardRhythm: state.keyboard_rhythm_enabled,
    activeAppPresence: state.active_app_presence_enabled,
  };
}

function capabilityToStateKey(capability: string): keyof ReturnType<typeof createCapabilityState> {
  switch (capability) {
    case "byom":
      return "byom";
    case "git_metadata":
      return "gitMetadata";
    case "build_test_events":
      return "buildTestEvents";
    case "keyboard_rhythm":
      return "keyboardRhythm";
    case "active_app_presence":
      return "activeAppPresence";
    default:
      return "byom";
  }
}
