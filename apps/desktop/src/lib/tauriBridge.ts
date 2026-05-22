import { invoke } from "@tauri-apps/api/core";
import type {
  AnimationName,
  BehaviorMode,
  BuildTestStatus,
  DeveloperSnooze,
  DeveloperWorkspace,
  IanAction,
  IanEvent,
  IanState,
  Position,
  QuietHours,
} from "../protocol/generated";

const browserFallbackState: IanState = {
  active_pet_id: "ian-puppy",
  current_behavior: "idle",
  current_animation: "idle",
  position: { x: 0, y: 0 },
  active_resource_pack: "ian-puppy",
  behavior_mode: "normal",
  reminders_enabled: true,
  reminder_interval_minutes: 90,
  do_not_disturb: false,
  byom_enabled: false,
  byom_key_configured: false,
  git_metadata_enabled: false,
  build_test_events_enabled: false,
  keyboard_rhythm_enabled: false,
  active_app_presence_enabled: false,
  privacy_onboarding_seen: false,
  find_ian_shortcut_enabled: false,
  find_ian_shortcut: "CommandOrControl+Shift+I",
  home_anchor: { x: 0, y: 0 },
  screen_bounds: null,
  last_user_interaction_ms: 0,
  quiet_hours: { enabled: false, start_minute: 22 * 60, end_minute: 7 * 60 },
  movement_intensity: "normal",
  bubble_frequency: "normal",
  rest_behavior: "normal",
  playful_energy: "normal",
  playful_state: "idle",
  playful_state_until_ms: null,
  playful_snoozed_until_ms: null,
  last_playful_diagnostic: null,
  surface_scale: 1,
  diagnostics_enabled: true,
  day_phase: "day",
  is_dragging: false,
  is_bubble_input_active: false,
  developer_workspace: {
    bound: false,
    workspace_id: null,
    display_name: null,
    root_path: null,
    enabled: false,
  },
  developer_snooze: {
    enabled: false,
    until_ms: null,
    reason: null,
  },
  active_app_category: null,
};
export type MemoryCandidateView = {
  id: number;
  tags: string[];
  status: "candidate" | "confirmed" | string;
  created_at_ms: number;
  confirmed_at_ms?: number | null;
};

export type MemoryExportRecord = MemoryCandidateView;

const browserFallbackMemoryCandidates: MemoryCandidateView[] = [];
const DESKTOP_TICK_WINDOW_SECS = 15;
let browserInteractionCount = 0;
let browserAttentionAvailableAfterMs = 0;

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function sendIanEvent(event: IanEvent): Promise<IanAction[]> {
  if (isTauriRuntime()) {
    return invoke<IanAction[]>("handle_ian_event", { event });
  }

  if (event.type === "mouse.near") {
    if (event.now_ms < browserAttentionAvailableAfterMs) {
      return [];
    }
    browserAttentionAvailableAfterMs = event.now_ms + 5_000;
    return [{ type: "animation.play", name: "happy", looped: false }];
  }

  if (event.type === "bubble.input_started") {
    browserFallbackState.is_bubble_input_active = true;
    return [{ type: "state.sync", state: browserFallbackState }];
  }

  if (event.type === "active_app.presence") {
    browserFallbackState.active_app_category = event.category;
    return [{ type: "state.sync", state: browserFallbackState }];
  }

  if (event.type === "screen.bounds") {
    browserFallbackState.screen_bounds = {
      x: event.x,
      y: event.y,
      width: event.width,
      height: event.height,
    };
    return [{ type: "state.sync", state: browserFallbackState }];
  }

  if (event.type === "time.tick") {
    if (browserFallbackState.do_not_disturb) {
      browserFallbackState.current_animation = "idle";
      browserFallbackState.current_behavior = "idle";
      return [{ type: "animation.play", name: "idle", looped: true }];
    }

    const second = Math.floor(event.now_ms / 1000);
    let name: AnimationName = "idle";
    let looped = true;

    if (
      browserFallbackState.playful_energy === "high" &&
      browserFallbackState.movement_intensity !== "reduced" &&
      browserFallbackState.behavior_mode !== "quiet" &&
      !browserFallbackState.is_bubble_input_active &&
      isInTickWindow(second, 180)
    ) {
      browserFallbackState.current_animation = "zoomies";
      browserFallbackState.current_behavior = "zooming";
      browserFallbackState.playful_state = "cooling_down";
      browserFallbackState.playful_state_until_ms = event.now_ms + 60_000;
      browserFallbackState.last_playful_diagnostic = {
        timestamp_ms: event.now_ms,
        reason: "idle_surprise",
        result: "triggered",
        cooldown_key: "zoomies",
        chosen_reaction_key: "zoomies_path",
      };
      return [
        {
          type: "playful.diagnostic",
          ...browserFallbackState.last_playful_diagnostic,
        },
        { type: "behavior.zoomies", duration_ms: 3200, reason: "idle_surprise" },
        {
          type: "effect.play",
          name: "speed_lines",
          intensity: "high",
          duration_ms: 900,
        },
        { type: "animation.play", name: "zoomies", looped: true },
        { type: "state.sync", state: browserFallbackState },
      ];
    }

    if (isInTickWindow(second, 90)) {
      name = "sleep";
    } else if (
      browserFallbackState.behavior_mode !== "quiet" &&
      isInTickWindow(second, 45)
    ) {
      name = "rest";
    }

    browserFallbackState.current_animation = name;
    browserFallbackState.current_behavior =
      name === "sleep" ? "sleeping" : name === "rest" ? "resting" : "idle";

    const actions: IanAction[] = [
      { type: "animation.play", name, looped },
      { type: "state.sync", state: browserFallbackState },
    ];

    if (
      name === "idle" &&
      browserFallbackState.movement_intensity !== "reduced" &&
      browserFallbackState.behavior_mode !== "quiet" &&
      !browserFallbackState.is_bubble_input_active &&
      second >= 30 &&
      second % 30 < DESKTOP_TICK_WINDOW_SECS
    ) {
      actions.splice(1, 0, {
        type: "effect.play",
        name: "tail_wag",
        intensity: "low",
        duration_ms: 900,
      });
    }

    return actions;
  }

  if (
    event.type === "developer.git_status_changed" ||
    event.type === "developer.build_test_summary" ||
    event.type === "keyboard.rhythm"
  ) {
    if (
      event.type !== "keyboard.rhythm" &&
      !browserFallbackState.developer_workspace.bound
    ) {
      return [{ type: "state.sync", state: browserFallbackState }];
    }
    if (
      browserFallbackState.developer_snooze.enabled &&
      (!browserFallbackState.developer_snooze.until_ms ||
        Date.now() < browserFallbackState.developer_snooze.until_ms)
    ) {
      return [{ type: "state.sync", state: browserFallbackState }];
    }
    return [
      { type: "bubble.open" },
      {
        type: "speech.show",
        text: event.type === "developer.build_test_summary" ? "过啦。" : "收好啦。",
        mood: "calm",
        duration_ms: 2400,
      },
      { type: "animation.play", name: "happy", looped: false },
    ];
  }

  if (event.type === "bubble.input_ended") {
    browserFallbackState.is_bubble_input_active = false;
    return [{ type: "state.sync", state: browserFallbackState }];
  }

  if (event.type === "mouse.leave") {
    return [{ type: "animation.play", name: "idle", looped: true }];
  }

  if (event.type === "mouse.double_click") {
    return [
      { type: "behavior.run_around", duration_ms: 1800 },
      { type: "animation.play", name: "run", looped: true },
      {
        type: "movement.move_to",
        x: browserFallbackState.position.x + 80,
        y: browserFallbackState.position.y,
        speed: "fast",
        profile: "playful",
      },
      {
        type: "movement.move_to",
        x: browserFallbackState.home_anchor.x,
        y: browserFallbackState.home_anchor.y,
        speed: "fast",
        profile: "settle",
      },
      {
        type: "playful.state",
        state: "settling",
        until_ms: Date.now() + 1200,
      },
      {
        type: "effect.play",
        name: "blush_puff",
        intensity: "low",
        duration_ms: 800,
      },
      { type: "animation.play", name: "idle", looped: true },
    ];
  }

  if (event.type === "mouse.click") {
    if (browserFallbackState.current_animation === "sleep") {
      browserFallbackState.current_animation = "idle";
      browserFallbackState.current_behavior = "idle";
      return [
        { type: "animation.play", name: "idle", looped: true },
        { type: "state.sync", state: browserFallbackState },
      ];
    }

    browserInteractionCount += 1;
    if (browserInteractionCount % 4 === 3) {
      return [
        { type: "animation.play", name: "happy", looped: false },
        {
          type: "effect.play",
          name: "sparkle_pop",
          intensity: "low",
          duration_ms: 700,
        },
      ];
    }
    const text =
      browserInteractionCount >= 4 && browserInteractionCount % 4 === 0
        ? "有点痒，我挪一下。"
        : browserInteractionCount >= 2
          ? "再摸摸也可以。"
          : "我在这儿。";

    const actions: IanAction[] = [
      { type: "bubble.open" },
      { type: "speech.show", text, mood: "calm", duration_ms: 2400 },
      { type: "animation.play", name: "happy", looped: false },
      {
        type: "effect.play",
        name:
          browserInteractionCount >= 4 && browserInteractionCount % 4 === 0
            ? "blush_puff"
            : "heart_pop",
        intensity: "low",
        duration_ms:
          browserInteractionCount >= 4 && browserInteractionCount % 4 === 0
            ? 800
            : 900,
      },
    ];

    if (browserInteractionCount >= 4 && browserInteractionCount % 4 === 0) {
      actions.push({
        type: "movement.move_to",
        x: browserFallbackState.position.x + 18,
        y: browserFallbackState.position.y,
        speed: "slow",
        profile: "gentle",
      });
    }

    return actions;
  }

  if (event.type === "mouse.drag_start") {
    browserFallbackState.is_dragging = true;
    return [
      { type: "animation.play", name: "happy", looped: false },
      { type: "state.sync", state: browserFallbackState },
    ];
  }

  if (event.type === "mouse.drag_end") {
    browserFallbackState.is_dragging = false;
    browserFallbackState.position = { x: event.x, y: event.y };
    return [
      {
        type: "movement.move_to",
        x: event.x,
        y: event.y,
        speed: "normal",
        profile: "settle",
      },
      { type: "speech.show", text: "放这里。", mood: "calm", duration_ms: 1600 },
      { type: "state.sync", state: browserFallbackState },
    ];
  }

  if (event.type === "mouse.chase_candidate") {
    return [{ type: "state.sync", state: browserFallbackState }];
  }

  if (event.type === "moment.debug_trigger") {
    const diagnostic = {
      type: "playful.diagnostic" as const,
      timestamp_ms: event.now_ms,
      reason: event.kind,
      result: "diagnostic_triggered",
      cooldown_key: event.kind,
      chosen_reaction_key: `${event.kind}_sequence`,
    };

    if (event.kind === "rare_idle_surprise") {
      if (browserFallbackState.do_not_disturb) {
        return [{ ...diagnostic, result: "blocked_dnd", chosen_reaction_key: null }];
      }
      if (browserFallbackState.movement_intensity === "reduced") {
        return [
          { ...diagnostic, result: "blocked_reduced_motion", chosen_reaction_key: null },
        ];
      }
    }

    const actionsByKind: Record<string, IanAction[]> = {
      find_ian_entrance: [
        diagnostic,
        { type: "movement.move_to", x: 48, y: 48, speed: "fast", profile: "playful" },
        { type: "bubble.open" },
        { type: "speech.show", text: "我在这儿。", mood: "calm", duration_ms: 1800 },
        { type: "effect.play", name: "find_beacon", intensity: "low", duration_ms: 1200 },
      ],
      pointer_curiosity: [
        diagnostic,
        { type: "animation.play", name: "wave", looped: false },
        {
          type: "movement.move_to",
          x: browserFallbackState.position.x + 24,
          y: browserFallbackState.position.y,
          speed: "slow",
          profile: "playful",
        },
        { type: "effect.play", name: "sparkle_pop", intensity: "low", duration_ms: 600 },
      ],
      drag_carry: [
        diagnostic,
        { type: "appearance.scale_to", scale: 0.92, duration_ms: 180 },
        { type: "animation.play", name: "affection", looped: false },
      ],
      drop_settle: [
        diagnostic,
        { type: "movement.move_to", x: 24, y: 16, speed: "normal", profile: "settle" },
        { type: "speech.show", text: "放这里。", mood: "calm", duration_ms: 1200 },
      ],
      rare_idle_surprise: [
        diagnostic,
        { type: "animation.play", name: "wave", looped: false },
        { type: "effect.play", name: "tail_wag", intensity: "low", duration_ms: 800 },
      ],
      memory_echo: [
        diagnostic,
        { type: "bubble.open" },
        {
          type: "speech.show",
          text: "我记得你喜欢安静一点。",
          mood: "calm",
          duration_ms: 1800,
        },
      ],
    };

    return actionsByKind[event.kind] ?? [
      {
        ...diagnostic,
        reason: "moment_debug",
        result: "blocked_unknown_kind",
        cooldown_key: null,
        chosen_reaction_key: null,
      },
    ];
  }

  if (event.type === "system.shortcut_triggered" && event.action === "find_ian") {
    browserFallbackState.position = { x: 48, y: 48 };
    return [
      { type: "movement.move_to", x: 48, y: 48, speed: "fast", profile: "playful" },
      { type: "bubble.open" },
      { type: "speech.show", text: "我在这儿。", mood: "calm", duration_ms: 2200 },
      { type: "effect.play", name: "find_beacon", intensity: "low", duration_ms: 1600 },
      { type: "animation.play", name: "happy", looped: false },
      { type: "state.sync", state: browserFallbackState },
    ];
  }

  if (event.type === "dialogue.user_message") {
    browserFallbackState.is_bubble_input_active = false;
    const text =
      event.text.toLowerCase().includes("water") || event.text.includes("水")
        ? "喝水水。"
        : "我在这儿。";

    return [
      { type: "bubble.open" },
      { type: "speech.show", text, mood: "calm", duration_ms: 2600 },
      { type: "animation.play", name: "happy", looped: false },
      { type: "state.sync", state: browserFallbackState },
    ];
  }

  return [{ type: "state.sync", state: browserFallbackState }];
}

function isInTickWindow(second: number, cadence: number): boolean {
  return second >= cadence && second % cadence < DESKTOP_TICK_WINDOW_SECS;
}

export async function getIanState(): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("get_ian_state");
  }

  return browserFallbackState;
}

export async function saveIanPosition(position: Position): Promise<IanAction[]> {
  if (isTauriRuntime()) {
    return invoke<IanAction[]>("save_window_position", { position });
  }

  browserFallbackState.position = position;
  browserFallbackState.home_anchor = position;
  return [{ type: "state.sync", state: browserFallbackState }];
}

export async function resetIanPosition(): Promise<IanAction[]> {
  if (isTauriRuntime()) {
    return invoke<IanAction[]>("reset_window_position");
  }

  browserFallbackState.position = { x: 0, y: 0 };
  browserFallbackState.home_anchor = { x: 0, y: 0 };
  return [{ type: "state.sync", state: browserFallbackState }];
}

export async function getIanSettings(): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("get_settings");
  }

  return browserFallbackState;
}

export async function saveBehaviorMode(mode: BehaviorMode): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_behavior_mode", { mode });
  }

  browserFallbackState.behavior_mode = mode;
  return browserFallbackState;
}

export async function saveActivePet(activePetId: string): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_active_pet", { activePetId });
  }

  browserFallbackState.active_pet_id = activePetId;
  browserFallbackState.active_resource_pack = activePetId;
  return browserFallbackState;
}

export async function saveQuietHours(quietHours: QuietHours): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_quiet_hours", { quietHours });
  }

  browserFallbackState.quiet_hours = quietHours;
  return browserFallbackState;
}

export async function saveCreatureSettings(settings: {
  movement_intensity: string;
  bubble_frequency: string;
  rest_behavior: string;
  playful_energy: IanState["playful_energy"];
  playful_snoozed_until_ms?: number | null;
  surface_scale: number;
  diagnostics_enabled: boolean;
}): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_creature_settings", {
      movementIntensity: settings.movement_intensity,
      bubbleFrequency: settings.bubble_frequency,
      restBehavior: settings.rest_behavior,
      playfulEnergy: settings.playful_energy,
      playfulSnoozedUntilMs: settings.playful_snoozed_until_ms ?? null,
      surfaceScale: settings.surface_scale,
      diagnosticsEnabled: settings.diagnostics_enabled,
    });
  }

  browserFallbackState.movement_intensity = settings.movement_intensity;
  browserFallbackState.bubble_frequency = settings.bubble_frequency;
  browserFallbackState.rest_behavior = settings.rest_behavior;
  browserFallbackState.playful_energy = settings.playful_energy;
  browserFallbackState.playful_snoozed_until_ms =
    settings.playful_snoozed_until_ms ?? null;
  browserFallbackState.surface_scale = settings.surface_scale;
  browserFallbackState.diagnostics_enabled = settings.diagnostics_enabled;
  return browserFallbackState;
}

export async function saveRemindersEnabled(enabled: boolean): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_reminders_enabled", { enabled });
  }

  browserFallbackState.reminders_enabled = enabled;
  return browserFallbackState;
}

export async function saveReminderSettings(
  enabled: boolean,
  intervalMinutes: number,
): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_reminder_settings", {
      enabled,
      intervalMinutes,
    });
  }

  browserFallbackState.reminders_enabled = enabled;
  browserFallbackState.reminder_interval_minutes = intervalMinutes;
  return browserFallbackState;
}

export async function saveDoNotDisturb(enabled: boolean): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_do_not_disturb", { enabled });
  }

  browserFallbackState.do_not_disturb = enabled;
  return browserFallbackState;
}

export async function savePrivacyOnboardingSeen(seen: boolean): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_privacy_onboarding_seen", { seen });
  }

  browserFallbackState.privacy_onboarding_seen = seen;
  return browserFallbackState;
}

export async function saveFindIanShortcutEnabled(enabled: boolean): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_find_ian_shortcut_enabled", { enabled });
  }

  browserFallbackState.find_ian_shortcut_enabled = enabled;
  return browserFallbackState;
}

export async function saveCapabilityEnabled(
  capability: string,
  enabled: boolean,
): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_capability_enabled", { capability, enabled });
  }

  const field = `${capability}_enabled` as keyof IanState;
  if (field in browserFallbackState) {
    (browserFallbackState[field] as boolean) = enabled;
  }
  return browserFallbackState;
}

export async function saveDeveloperWorkspace(
  workspace: DeveloperWorkspace,
): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_developer_workspace", { workspace });
  }

  browserFallbackState.developer_workspace = workspace;
  return browserFallbackState;
}

export async function saveDeveloperSnooze(snooze: DeveloperSnooze): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_developer_snooze", { snooze });
  }

  browserFallbackState.developer_snooze = snooze;
  return browserFallbackState;
}

export async function ingestBuildTestSummary(summary: {
  workspace_id?: string | null;
  tool: string;
  status: BuildTestStatus;
  duration_ms: number;
  tests_total: number;
  tests_failed: number;
  error_kind?: string | null;
}): Promise<IanAction[]> {
  if (isTauriRuntime()) {
    return invoke<IanAction[]>("ingest_build_test_summary", {
      workspaceId: summary.workspace_id,
      tool: summary.tool,
      status: summary.status,
      durationMs: summary.duration_ms,
      testsTotal: summary.tests_total,
      testsFailed: summary.tests_failed,
      errorKind: summary.error_kind ?? null,
    });
  }

  return sendIanEvent({
    type: "developer.build_test_summary",
    workspace_id: summary.workspace_id,
    tool: summary.tool,
    status: summary.status,
    duration_ms: summary.duration_ms,
    tests_total: summary.tests_total,
    tests_failed: summary.tests_failed,
    error_kind: summary.error_kind,
  });
}

export async function listMemoryCandidates(): Promise<MemoryCandidateView[]> {
  if (isTauriRuntime()) {
    return invoke<MemoryCandidateView[]>("list_memory_candidates");
  }

  return [...browserFallbackMemoryCandidates];
}

export async function confirmMemoryCandidate(id: number): Promise<MemoryCandidateView[]> {
  if (isTauriRuntime()) {
    return invoke<MemoryCandidateView[]>("confirm_memory_candidate", { id });
  }

  const candidate = browserFallbackMemoryCandidates.find((item) => item.id === id);
  if (candidate) {
    candidate.status = "confirmed";
    candidate.confirmed_at_ms = Date.now();
  }
  return browserFallbackMemoryCandidates.filter((item) => item.status === "candidate");
}

export async function deleteMemoryCandidate(id: number): Promise<MemoryCandidateView[]> {
  if (isTauriRuntime()) {
    return invoke<MemoryCandidateView[]>("delete_memory_candidate", { id });
  }

  const index = browserFallbackMemoryCandidates.findIndex((item) => item.id === id);
  if (index >= 0) {
    browserFallbackMemoryCandidates.splice(index, 1);
  }
  return [...browserFallbackMemoryCandidates];
}

export async function clearMemoryCandidates(): Promise<MemoryCandidateView[]> {
  if (isTauriRuntime()) {
    return invoke<MemoryCandidateView[]>("clear_memory_candidates");
  }

  browserFallbackMemoryCandidates.splice(0, browserFallbackMemoryCandidates.length);
  return [];
}

export async function exportMemorySummary(): Promise<MemoryExportRecord[]> {
  if (isTauriRuntime()) {
    return invoke<MemoryExportRecord[]>("export_memory_summary");
  }

  return [...browserFallbackMemoryCandidates];
}

export async function clearInteractionJournal(): Promise<void> {
  if (isTauriRuntime()) {
    await invoke<void>("clear_interaction_journal");
  }
}

export async function resetLocalSettings(): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("reset_local_settings");
  }

  Object.assign(browserFallbackState, {
    behavior_mode: "normal",
    reminders_enabled: true,
    reminder_interval_minutes: 90,
    do_not_disturb: false,
    find_ian_shortcut_enabled: false,
    quiet_hours: { enabled: false, start_minute: 22 * 60, end_minute: 7 * 60 },
    movement_intensity: "normal",
    bubble_frequency: "normal",
    rest_behavior: "normal",
    playful_energy: "normal",
    surface_scale: 1,
    diagnostics_enabled: true,
  });
  return browserFallbackState;
}
