import { useEffect, useState } from "react";
import type { BehaviorMode, QuietHours } from "./protocol/generated";
import { IanStage } from "./renderer/IanStage";
import { loadPetResourcePack, type PetResourcePack } from "./resources/resourceLoader";
import {
  getIanSettings,
  saveBehaviorMode,
  saveCapabilityEnabled,
  saveCreatureSettings,
  saveQuietHours,
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
  const [quietHours, setQuietHours] = useState<QuietHours>({
    enabled: false,
    start_minute: 22 * 60,
    end_minute: 7 * 60,
  });
  const [creatureSettings, setCreatureSettings] = useState(createCreatureSettingsState());
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { savePosition, sendEvent, viewState } = useIanActions();

  useEffect(() => {
    void loadPetResourcePack("ian-alpaca").then(setResourcePack);
    void getIanSettings().then((state) => {
      setBehaviorMode(state.behavior_mode);
      setRemindersEnabled(state.reminders_enabled);
      setCapabilities(capabilityStateFromIanState(state));
      setQuietHours(state.quiet_hours);
      setCreatureSettings(creatureSettingsFromIanState(state));
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
      quietHours={quietHours}
      movementIntensity={creatureSettings.movementIntensity}
      bubbleFrequency={creatureSettings.bubbleFrequency}
      restBehavior={creatureSettings.restBehavior}
      surfaceScale={creatureSettings.surfaceScale}
      diagnosticsEnabled={creatureSettings.diagnosticsEnabled}
      isSettingsOpen={isSettingsOpen}
      onIanClick={(point) => {
        void sendEvent({ type: "mouse.click", ...point });
      }}
      onIanDoubleClick={(point) => {
        void sendEvent({ type: "mouse.double_click", ...point });
      }}
      onIanNear={(point) => {
        void sendEvent({ type: "mouse.near", ...point, now_ms: Date.now() });
      }}
      onIanLeave={(point) => {
        void sendEvent({ type: "mouse.leave", ...point });
      }}
      onSubmitMessage={(text) => {
        void sendEvent({ type: "dialogue.user_message", text });
      }}
      onBubbleInputStarted={() => {
        void sendEvent({ type: "bubble.input_started" });
      }}
      onBubbleInputEnded={() => {
        void sendEvent({ type: "bubble.input_ended" });
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
      onQuietHoursChange={(nextQuietHours) => {
        setQuietHours(nextQuietHours);
        void saveQuietHours(nextQuietHours).then((state) => {
          setQuietHours(state.quiet_hours);
        });
      }}
      onCreatureSettingsChange={(nextSettings) => {
        setCreatureSettings(nextSettings);
        void saveCreatureSettings({
          movement_intensity: nextSettings.movementIntensity,
          bubble_frequency: nextSettings.bubbleFrequency,
          rest_behavior: nextSettings.restBehavior,
          surface_scale: nextSettings.surfaceScale,
          diagnostics_enabled: nextSettings.diagnosticsEnabled,
        }).then((state) => {
          setCreatureSettings(creatureSettingsFromIanState(state));
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
        void sendEvent({ type: "mouse.drag_end", ...point });
        void savePosition(point);
      }}
      onDragStart={(point) => {
        void sendEvent({ type: "mouse.drag_start", ...point });
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

function createCreatureSettingsState() {
  return {
    movementIntensity: "normal",
    bubbleFrequency: "normal",
    restBehavior: "normal",
    surfaceScale: 1,
    diagnosticsEnabled: true,
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

function creatureSettingsFromIanState(state: Awaited<ReturnType<typeof getIanSettings>>) {
  return {
    movementIntensity: state.movement_intensity,
    bubbleFrequency: state.bubble_frequency,
    restBehavior: state.rest_behavior,
    surfaceScale: state.surface_scale,
    diagnosticsEnabled: state.diagnostics_enabled,
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
