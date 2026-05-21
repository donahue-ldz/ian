import { useEffect, useRef, useState } from "react";
import type {
  BehaviorMode,
  DeveloperSnooze,
  DeveloperWorkspace,
  IanEvent,
  PlayfulEnergy,
  Position,
  QuietHours,
} from "./protocol/generated";
import { IanStage } from "./renderer/IanStage";
import { loadPetResourcePack, type PetResourcePack } from "./resources/resourceLoader";
import {
  getIanSettings,
  saveBehaviorMode,
  saveCapabilityEnabled,
  saveCreatureSettings,
  saveDeveloperSnooze,
  saveDeveloperWorkspace,
  saveQuietHours,
  saveRemindersEnabled,
} from "./lib/tauriBridge";
import {
  getDesktopCursorPosition,
  getDesktopScreenBounds,
  getDesktopWindowPosition,
  moveDesktopWindow,
  resolveSavedDragPosition,
  startDesktopWindowDrag,
} from "./lib/position";
import { useIanActions } from "./state/useIanActions";
import { viewStateForWindowContent } from "./state/ianActions";

const TICK_INTERVAL_MS = 15_000;
const POINTER_CHASE_INTERVAL_MS = 10_000;
const CLICK_CHASE_FALLBACK_DELAY_MS = 450;
const DEFAULT_RESOURCE_PACK_ID = "ian-puppy";

type ClickChaseDependencies = {
  point: Position;
  isDesktopWindow: boolean;
  sendEvent: (event: IanEvent) => Promise<void> | void;
};

type LeaveChaseDependencies = {
  point: Position;
  isDesktopWindow: boolean;
  isChaseArmed: boolean;
  now: () => number;
  getCursorPosition: () => Promise<Position | null>;
  sendEvent: (event: IanEvent) => Promise<void> | void;
};

type ArmedChaseDependencies = Omit<LeaveChaseDependencies, "point">;

export async function sendIanClickAndArmChase({
  point,
  isDesktopWindow,
  sendEvent,
}: ClickChaseDependencies): Promise<boolean> {
  await sendEvent({ type: "mouse.click", ...point });
  return isDesktopWindow;
}

export async function sendIanLeaveWithArmedChase({
  point,
  isDesktopWindow,
  isChaseArmed,
  now,
  getCursorPosition,
  sendEvent,
}: LeaveChaseDependencies): Promise<boolean> {
  await sendEvent({ type: "mouse.leave", ...point });

  return sendArmedChaseCandidate({
    isDesktopWindow,
    isChaseArmed,
    now,
    getCursorPosition,
    sendEvent,
  });
}

export async function sendArmedChaseCandidate({
  isDesktopWindow,
  isChaseArmed,
  now,
  getCursorPosition,
  sendEvent,
}: ArmedChaseDependencies): Promise<boolean> {
  if (!isDesktopWindow || !isChaseArmed) {
    return false;
  }

  const cursorPosition = await getCursorPosition();
  if (!cursorPosition) {
    return false;
  }

  await sendEvent({
    type: "mouse.chase_candidate",
    ...cursorPosition,
    now_ms: now(),
  });
  return false;
}

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
  const [developerWorkspace, setDeveloperWorkspace] = useState<DeveloperWorkspace>({
    bound: false,
    workspace_id: null,
    display_name: null,
    root_path: null,
    enabled: false,
  });
  const [developerSnooze, setDeveloperSnooze] = useState<DeveloperSnooze>({
    enabled: false,
    until_ms: null,
    reason: null,
  });
  const [creatureSettings, setCreatureSettings] = useState(createCreatureSettingsState());
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const desktopDragOrigin = useRef<{ x: number; y: number } | null>(null);
  const pointerLeaveChaseArmed = useRef(false);
  const pointerChaseFallbackTimeout = useRef<number | null>(null);
  const { savePosition, sendEvent, viewState } = useIanActions();
  const isDesktopWindow = "__TAURI_INTERNALS__" in window;
  const stageViewState = viewStateForWindowContent(viewState, isDesktopWindow);

  useEffect(() => {
    void loadPetResourcePack(DEFAULT_RESOURCE_PACK_ID).then(setResourcePack);
    void getIanSettings().then((state) => {
      setBehaviorMode(state.behavior_mode);
      setRemindersEnabled(state.reminders_enabled);
      setCapabilities(capabilityStateFromIanState(state));
      setQuietHours(state.quiet_hours);
      setDeveloperWorkspace(state.developer_workspace);
      setDeveloperSnooze(state.developer_snooze);
      setCreatureSettings(creatureSettingsFromIanState(state));
    });
    void sendEvent({ type: "app.started" });
    void sendDesktopScreenBounds(sendEvent);
  }, [sendEvent]);

  useEffect(() => {
    const interval = window.setInterval(() => {
      void sendDesktopScreenBounds(sendEvent);
      void sendEvent({ type: "time.tick", now_ms: Date.now() });
    }, TICK_INTERVAL_MS);

    return () => window.clearInterval(interval);
  }, [sendEvent]);

  useEffect(() => {
    if (!isDesktopWindow) {
      return;
    }

    const interval = window.setInterval(() => {
      void getDesktopCursorPosition().then((position) => {
        if (!position) {
          return;
        }

        void sendEvent({
          type: "mouse.chase_candidate",
          ...position,
          now_ms: Date.now(),
        });
      });
    }, POINTER_CHASE_INTERVAL_MS);

    return () => window.clearInterval(interval);
  }, [isDesktopWindow, sendEvent]);

  return (
    <IanStage
      resourcePack={resourcePack}
      viewState={stageViewState}
      behaviorMode={behaviorMode}
      remindersEnabled={remindersEnabled}
      byomEnabled={capabilities.byom}
      gitMetadataEnabled={capabilities.gitMetadata}
      buildTestEventsEnabled={capabilities.buildTestEvents}
      keyboardRhythmEnabled={capabilities.keyboardRhythm}
      activeAppPresenceEnabled={capabilities.activeAppPresence}
      quietHours={quietHours}
      developerWorkspace={developerWorkspace}
      developerSnooze={developerSnooze}
      movementIntensity={creatureSettings.movementIntensity}
      bubbleFrequency={creatureSettings.bubbleFrequency}
      restBehavior={creatureSettings.restBehavior}
      playfulEnergy={creatureSettings.playfulEnergy}
      playfulSnoozedUntilMs={creatureSettings.playfulSnoozedUntilMs}
      surfaceScale={creatureSettings.surfaceScale}
      diagnosticsEnabled={creatureSettings.diagnosticsEnabled}
      isSettingsOpen={isSettingsOpen}
      isDesktopWindow={isDesktopWindow}
      onIanClick={(point) => {
        if (pointerChaseFallbackTimeout.current !== null) {
          window.clearTimeout(pointerChaseFallbackTimeout.current);
          pointerChaseFallbackTimeout.current = null;
        }

        void sendIanClickAndArmChase({
          point,
          isDesktopWindow,
          sendEvent,
        }).then((isArmed) => {
          pointerLeaveChaseArmed.current = isArmed;
          if (!isArmed) {
            return;
          }

          pointerChaseFallbackTimeout.current = window.setTimeout(() => {
            void sendArmedChaseCandidate({
              isDesktopWindow,
              isChaseArmed: pointerLeaveChaseArmed.current,
              now: Date.now,
              getCursorPosition: getDesktopCursorPosition,
              sendEvent,
            }).then((nextArmed) => {
              pointerLeaveChaseArmed.current = nextArmed;
              pointerChaseFallbackTimeout.current = null;
            });
          }, CLICK_CHASE_FALLBACK_DELAY_MS);
        });
      }}
      onIanDoubleClick={(point) => {
        void sendEvent({ type: "mouse.double_click", ...point });
      }}
      onIanNear={(point) => {
        void sendEvent({ type: "mouse.near", ...point, now_ms: Date.now() });
      }}
      onIanLeave={(point) => {
        if (pointerChaseFallbackTimeout.current !== null) {
          window.clearTimeout(pointerChaseFallbackTimeout.current);
          pointerChaseFallbackTimeout.current = null;
        }

        void sendIanLeaveWithArmedChase({
          point,
          isDesktopWindow,
          isChaseArmed: pointerLeaveChaseArmed.current,
          now: Date.now,
          getCursorPosition: getDesktopCursorPosition,
          sendEvent,
        }).then((isArmed) => {
          pointerLeaveChaseArmed.current = isArmed;
        });
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
      onDeveloperWorkspaceChange={(workspace) => {
        setDeveloperWorkspace(workspace);
        void saveDeveloperWorkspace(workspace).then((state) => {
          setDeveloperWorkspace(state.developer_workspace);
        });
      }}
      onDeveloperSnoozeChange={(snooze) => {
        setDeveloperSnooze(snooze);
        void saveDeveloperSnooze(snooze).then((state) => {
          setDeveloperSnooze(state.developer_snooze);
        });
      }}
      onCreatureSettingsChange={(nextSettings) => {
        setCreatureSettings(nextSettings);
        void saveCreatureSettings({
          movement_intensity: nextSettings.movementIntensity,
          bubble_frequency: nextSettings.bubbleFrequency,
          rest_behavior: nextSettings.restBehavior,
          playful_energy: nextSettings.playfulEnergy,
          playful_snoozed_until_ms: nextSettings.playfulSnoozedUntilMs,
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
        void (async () => {
          const desktopPosition = isDesktopWindow
            ? await getDesktopWindowPosition()
            : null;
          const dragEndPosition = resolveSavedDragPosition(point, desktopPosition);
          await sendEvent({ type: "mouse.drag_end", ...dragEndPosition });
          await savePosition(dragEndPosition);
        })();
        desktopDragOrigin.current = null;
      }}
      onDragMove={(offset) => {
        if (!isDesktopWindow || !desktopDragOrigin.current) {
          return;
        }

        void moveDesktopWindow({
          x: desktopDragOrigin.current.x + offset.x,
          y: desktopDragOrigin.current.y + offset.y,
        });
      }}
      onDragStart={(point) => {
        void sendEvent({ type: "mouse.drag_start", ...point });
        desktopDragOrigin.current = viewState.position;
        if (isDesktopWindow) {
          void startDesktopWindowDrag().catch(() => undefined);
          void getDesktopWindowPosition().then((desktopPosition) => {
            if (desktopPosition) {
              desktopDragOrigin.current = desktopPosition;
            }
          });
        }
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
    playfulEnergy: "normal" as PlayfulEnergy,
    playfulSnoozedUntilMs: null as number | null,
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
    playfulEnergy: state.playful_energy,
    playfulSnoozedUntilMs: state.playful_snoozed_until_ms ?? null,
    surfaceScale: state.surface_scale,
    diagnosticsEnabled: state.diagnostics_enabled,
  };
}

async function sendDesktopScreenBounds(
  sendEvent: (event: IanEvent) => Promise<void> | void,
) {
  const bounds = await getDesktopScreenBounds();
  if (!bounds) {
    return;
  }

  await sendEvent({ type: "screen.bounds", ...bounds });
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
