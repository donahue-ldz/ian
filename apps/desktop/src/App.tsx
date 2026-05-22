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
  clearMemoryCandidates,
  clearInteractionJournal,
  confirmMemoryCandidate,
  deleteMemoryCandidate,
  exportMemorySummary,
  getIanSettings,
  listMemoryCandidates,
  resetLocalSettings,
  saveActivePet,
  saveBehaviorMode,
  saveCapabilityEnabled,
  saveCreatureSettings,
  saveDeveloperSnooze,
  saveDeveloperWorkspace,
  saveDoNotDisturb,
  saveFindIanShortcutEnabled,
  savePrivacyOnboardingSeen,
  saveQuietHours,
  saveReminderSettings,
  saveRemindersEnabled,
  type MemoryCandidateView,
  type MemoryExportRecord,
} from "./lib/tauriBridge";
import {
  getDesktopCursorPosition,
  getDesktopScreenBounds,
  getDesktopWindowPosition,
  resolveSavedDragPosition,
  startDesktopWindowDrag,
} from "./lib/position";
import {
  registerFindIanShortcut,
  unregisterFindIanShortcut,
  type FindIanShortcutStatus,
} from "./lib/findIanShortcut";
import { useIanActions } from "./state/useIanActions";
import { viewStateForWindowContent } from "./state/ianActions";

const TICK_INTERVAL_MS = 15_000;
const POINTER_CHASE_INTERVAL_MS = 10_000;
const CLICK_CHASE_FALLBACK_DELAY_MS = 450;
const DEFAULT_RESOURCE_PACK_ID = "ian-adventurer";

type ClickChaseDependencies = {
  point: Position;
  isDesktopWindow: boolean;
  sendEvent: (event: IanEvent) => Promise<void> | void;
};

type LeaveChaseDependencies = {
  point: Position;
  isDesktopWindow: boolean;
  isChaseArmed: boolean;
  isSettingsOpen?: boolean;
  now: () => number;
  getCursorPosition: () => Promise<Position | null>;
  sendEvent: (event: IanEvent) => Promise<void> | void;
};

type ArmedChaseDependencies = Omit<LeaveChaseDependencies, "point">;

type SwitchPetResourcePackDependencies = {
  nextPetId: string;
  loadPetResourcePack: (id: string) => Promise<PetResourcePack>;
  saveActivePet: (id: string) => Promise<{
    active_pet_id: string;
    active_resource_pack: string;
  }>;
  setResourcePack: (pack: PetResourcePack) => void;
  setActivePetId: (id: string) => void;
};

type FindIanDependencies = {
  now: () => number;
  sendEvent: (event: IanEvent) => Promise<void> | void;
};

export async function findIan({ now, sendEvent }: FindIanDependencies): Promise<void> {
  await sendEvent({
    type: "system.shortcut_triggered",
    action: "find_ian",
    now_ms: now(),
  });
}

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
  isSettingsOpen,
  now,
  getCursorPosition,
  sendEvent,
}: LeaveChaseDependencies): Promise<boolean> {
  await sendEvent({ type: "mouse.leave", ...point });

  return sendArmedChaseCandidate({
    isDesktopWindow,
    isChaseArmed,
    isSettingsOpen,
    now,
    getCursorPosition,
    sendEvent,
  });
}

export async function sendArmedChaseCandidate({
  isDesktopWindow,
  isChaseArmed,
  isSettingsOpen,
  now,
  getCursorPosition,
  sendEvent,
}: ArmedChaseDependencies): Promise<boolean> {
  if (!isDesktopWindow || !isChaseArmed || isSettingsOpen) {
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

export async function switchPetResourcePack({
  nextPetId,
  loadPetResourcePack,
  saveActivePet,
  setResourcePack,
  setActivePetId,
}: SwitchPetResourcePackDependencies): Promise<void> {
  const nextPack = await loadPetResourcePack(nextPetId);
  const state = await saveActivePet(nextPetId);
  setResourcePack(nextPack);
  setActivePetId(state.active_resource_pack);
}

export default function App() {
  const [resourcePack, setResourcePack] = useState<PetResourcePack | null>(null);
  const [activePetId, setActivePetId] = useState(DEFAULT_RESOURCE_PACK_ID);
  const [behaviorMode, setBehaviorMode] = useState<BehaviorMode>("normal");
  const [remindersEnabled, setRemindersEnabled] = useState(true);
  const [reminderIntervalMinutes, setReminderIntervalMinutes] = useState(90);
  const [doNotDisturb, setDoNotDisturb] = useState(false);
  const [privacyOnboardingSeen, setPrivacyOnboardingSeen] = useState(false);
  const [findIanShortcutEnabled, setFindIanShortcutEnabled] = useState(false);
  const [findIanShortcut, setFindIanShortcut] = useState("CommandOrControl+Shift+I");
  const [findIanShortcutStatus, setFindIanShortcutStatus] =
    useState<FindIanShortcutStatus>("idle");
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
  const [memoryCandidates, setMemoryCandidates] = useState<MemoryCandidateView[]>([]);
  const [memoryExportRecords, setMemoryExportRecords] = useState<MemoryExportRecord[]>([]);
  const [isMemoryLoading, setIsMemoryLoading] = useState(false);
  const [memoryError, setMemoryError] = useState<string | null>(null);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const pointerLeaveChaseArmed = useRef(false);
  const pointerChaseFallbackTimeout = useRef<number | null>(null);
  const { resetPosition, savePosition, sendEvent, viewState } = useIanActions();
  const isDesktopWindow = "__TAURI_INTERNALS__" in window;
  const stageViewState = viewStateForWindowContent(viewState, isDesktopWindow);

  useEffect(() => {
    void getIanSettings().then((state) => {
      setActivePetId(state.active_resource_pack);
      void loadPetResourcePack(state.active_resource_pack)
        .then(setResourcePack)
        .catch(() =>
          loadPetResourcePack(DEFAULT_RESOURCE_PACK_ID).then((fallbackPack) => {
            setResourcePack(fallbackPack);
            setActivePetId(DEFAULT_RESOURCE_PACK_ID);
          }),
        );
      setBehaviorMode(state.behavior_mode);
      setRemindersEnabled(state.reminders_enabled);
      setReminderIntervalMinutes(state.reminder_interval_minutes);
      setDoNotDisturb(state.do_not_disturb);
      setPrivacyOnboardingSeen(state.privacy_onboarding_seen);
      setFindIanShortcutEnabled(state.find_ian_shortcut_enabled);
      setFindIanShortcut(state.find_ian_shortcut);
      setCapabilities(capabilityStateFromIanState(state));
      setQuietHours(state.quiet_hours);
      setDeveloperWorkspace(state.developer_workspace);
      setDeveloperSnooze(state.developer_snooze);
      setCreatureSettings(creatureSettingsFromIanState(state));
    });
    void refreshMemoryCandidates({
      setMemoryCandidates,
      setIsMemoryLoading,
      setMemoryError,
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
    if (!isDesktopWindow || isSettingsOpen) {
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
  }, [isDesktopWindow, isSettingsOpen, sendEvent]);

  useEffect(() => {
    if (!findIanShortcutEnabled) {
      void unregisterFindIanShortcut(findIanShortcut).then(() =>
        setFindIanShortcutStatus("idle"),
      );
      return;
    }

    let cancelled = false;
    void registerFindIanShortcut(findIanShortcut, () => {
      void findIan({ now: Date.now, sendEvent });
    }).then((status) => {
      if (!cancelled) {
        setFindIanShortcutStatus(status);
      }
    });

    return () => {
      cancelled = true;
      void unregisterFindIanShortcut(findIanShortcut);
    };
  }, [findIanShortcut, findIanShortcutEnabled, sendEvent]);

  return (
    <IanStage
      resourcePack={resourcePack}
      viewState={stageViewState}
      behaviorMode={behaviorMode}
      remindersEnabled={remindersEnabled}
      reminderIntervalMinutes={reminderIntervalMinutes}
      doNotDisturb={doNotDisturb}
      byomEnabled={capabilities.byom}
      byomKeyConfigured={capabilities.byomKeyConfigured}
      gitMetadataEnabled={capabilities.gitMetadata}
      buildTestEventsEnabled={capabilities.buildTestEvents}
      keyboardRhythmEnabled={capabilities.keyboardRhythm}
      activeAppPresenceEnabled={capabilities.activeAppPresence}
      privacyOnboardingSeen={privacyOnboardingSeen}
      findIanShortcutEnabled={findIanShortcutEnabled}
      findIanShortcut={findIanShortcut}
      findIanShortcutStatus={findIanShortcutStatus}
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
      activePetId={activePetId}
      memoryCandidates={memoryCandidates}
      memoryExportRecords={memoryExportRecords}
      isMemoryLoading={isMemoryLoading}
      memoryError={memoryError}
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
              isSettingsOpen,
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
          isSettingsOpen,
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
      onReminderIntervalMinutesChange={(intervalMinutes) => {
        setReminderIntervalMinutes(intervalMinutes);
        void saveReminderSettings(remindersEnabled, intervalMinutes).then((state) => {
          setRemindersEnabled(state.reminders_enabled);
          setReminderIntervalMinutes(state.reminder_interval_minutes);
        });
      }}
      onDoNotDisturbChange={(enabled) => {
        setDoNotDisturb(enabled);
        void saveDoNotDisturb(enabled).then((state) => {
          setDoNotDisturb(state.do_not_disturb);
        });
      }}
      onPrivacyOnboardingSeenChange={(seen) => {
        setPrivacyOnboardingSeen(seen);
        void savePrivacyOnboardingSeen(seen).then((state) => {
          setPrivacyOnboardingSeen(state.privacy_onboarding_seen);
        });
      }}
      onFindIan={() => {
        void findIan({ now: Date.now, sendEvent });
      }}
      onFindIanShortcutEnabledChange={(enabled) => {
        setFindIanShortcutEnabled(enabled);
        setFindIanShortcutStatus(enabled ? "idle" : "idle");
        void saveFindIanShortcutEnabled(enabled).then((state) => {
          setFindIanShortcutEnabled(state.find_ian_shortcut_enabled);
          setFindIanShortcut(state.find_ian_shortcut);
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
      onPetChange={(nextPetId) => {
        void switchPetResourcePack({
          nextPetId,
          loadPetResourcePack,
          saveActivePet,
          setResourcePack,
          setActivePetId,
        }).catch(() => undefined);
      }}
      onResetAppearance={() => {
        const defaults = createCreatureSettingsState();
        setCreatureSettings(defaults);
        void saveCreatureSettings({
          movement_intensity: defaults.movementIntensity,
          bubble_frequency: defaults.bubbleFrequency,
          rest_behavior: defaults.restBehavior,
          playful_energy: defaults.playfulEnergy,
          playful_snoozed_until_ms: defaults.playfulSnoozedUntilMs,
          surface_scale: defaults.surfaceScale,
          diagnostics_enabled: defaults.diagnosticsEnabled,
        }).then((state) => {
          setCreatureSettings(creatureSettingsFromIanState(state));
        });
        void switchPetResourcePack({
          nextPetId: DEFAULT_RESOURCE_PACK_ID,
          loadPetResourcePack,
          saveActivePet,
          setResourcePack,
          setActivePetId,
        }).catch(() => undefined);
      }}
      onResetPosition={() => {
        void resetPosition();
      }}
      onConfirmMemoryCandidate={(id) => {
        setMemoryError(null);
        void confirmMemoryCandidate(id)
          .then(setMemoryCandidates)
          .catch(() => setMemoryError("记忆候选操作失败"));
      }}
      onDeleteMemoryCandidate={(id) => {
        setMemoryError(null);
        void deleteMemoryCandidate(id)
          .then(setMemoryCandidates)
          .catch(() => setMemoryError("记忆候选操作失败"));
      }}
      onClearMemoryCandidates={() => {
        setMemoryError(null);
        void clearMemoryCandidates()
          .then(setMemoryCandidates)
          .catch(() => setMemoryError("记忆候选操作失败"));
      }}
      onRefreshMemoryExport={() => {
        setMemoryError(null);
        void exportMemorySummary()
          .then(setMemoryExportRecords)
          .catch(() => setMemoryError("记忆摘要导出失败"));
      }}
      onClearInteractionJournal={() => {
        setMemoryError(null);
        void clearInteractionJournal().catch(() => setMemoryError("本地日志清理失败"));
      }}
      onResetLocalSettings={() => {
        void resetLocalSettings().then((state) => {
          setBehaviorMode(state.behavior_mode);
          setRemindersEnabled(state.reminders_enabled);
          setReminderIntervalMinutes(state.reminder_interval_minutes);
          setDoNotDisturb(state.do_not_disturb);
          setPrivacyOnboardingSeen(state.privacy_onboarding_seen);
          setFindIanShortcutEnabled(state.find_ian_shortcut_enabled);
          setFindIanShortcut(state.find_ian_shortcut);
          setCapabilities(capabilityStateFromIanState(state));
          setQuietHours(state.quiet_hours);
          setDeveloperWorkspace(state.developer_workspace);
          setDeveloperSnooze(state.developer_snooze);
          setCreatureSettings(creatureSettingsFromIanState(state));
        });
      }}
      onMomentDebugTrigger={(kind) => {
        void sendEvent({ type: "moment.debug_trigger", kind, now_ms: Date.now() });
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
        if (capability === "byom" && enabled && !capabilities.byomKeyConfigured) {
          return;
        }
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
      }}
      onDragStart={(point) => {
        void sendEvent({ type: "mouse.drag_start", ...point });
        if (isDesktopWindow) {
          void startDesktopWindowDrag().catch(() => undefined);
        }
      }}
    />
  );
}

function createCapabilityState() {
  return {
    byom: false,
    byomKeyConfigured: false,
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
    byomKeyConfigured: state.byom_key_configured,
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

async function refreshMemoryCandidates({
  setMemoryCandidates,
  setIsMemoryLoading,
  setMemoryError,
}: {
  setMemoryCandidates: (candidates: MemoryCandidateView[]) => void;
  setIsMemoryLoading: (loading: boolean) => void;
  setMemoryError: (error: string | null) => void;
}) {
  setIsMemoryLoading(true);
  setMemoryError(null);
  try {
    setMemoryCandidates(await listMemoryCandidates());
  } catch {
    setMemoryError("记忆候选读取失败");
  } finally {
    setIsMemoryLoading(false);
  }
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
