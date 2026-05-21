import { invoke } from "@tauri-apps/api/core";
import type {
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
  active_pet_id: "ian-alpaca",
  current_behavior: "idle",
  current_animation: "idle",
  position: { x: 0, y: 0 },
  active_resource_pack: "ian-alpaca",
  behavior_mode: "normal",
  reminders_enabled: true,
  byom_enabled: false,
  git_metadata_enabled: false,
  build_test_events_enabled: false,
  keyboard_rhythm_enabled: false,
  active_app_presence_enabled: false,
  home_anchor: { x: 0, y: 0 },
  quiet_hours: { enabled: false, start_minute: 22 * 60, end_minute: 7 * 60 },
  movement_intensity: "normal",
  bubble_frequency: "normal",
  rest_behavior: "normal",
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
let browserInteractionCount = 0;
let browserAttentionAvailableAfterMs = 0;

function isTauriRuntime(): boolean {
  return "__TAURI_INTERNALS__" in window;
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
      },
      {
        type: "movement.move_to",
        x: browserFallbackState.home_anchor.x,
        y: browserFallbackState.home_anchor.y,
        speed: "fast",
      },
      { type: "animation.play", name: "idle", looped: true },
    ];
  }

  if (event.type === "mouse.click") {
    browserInteractionCount += 1;
    if (browserInteractionCount % 4 === 3) {
      return [];
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
    ];

    if (browserInteractionCount >= 4 && browserInteractionCount % 4 === 0) {
      actions.push({
        type: "movement.move_to",
        x: browserFallbackState.position.x + 18,
        y: browserFallbackState.position.y,
        speed: "slow",
      });
    }

    return actions;
  }

  if (event.type === "mouse.drag_start") {
    browserFallbackState.is_dragging = true;
    return [{ type: "state.sync", state: browserFallbackState }];
  }

  if (event.type === "mouse.drag_end") {
    browserFallbackState.is_dragging = false;
    browserFallbackState.position = { x: event.x, y: event.y };
    return [{ type: "state.sync", state: browserFallbackState }];
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
    ];
  }

  return [{ type: "state.sync", state: browserFallbackState }];
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
  surface_scale: number;
  diagnostics_enabled: boolean;
}): Promise<IanState> {
  if (isTauriRuntime()) {
    return invoke<IanState>("save_creature_settings", {
      movementIntensity: settings.movement_intensity,
      bubbleFrequency: settings.bubble_frequency,
      restBehavior: settings.rest_behavior,
      surfaceScale: settings.surface_scale,
      diagnosticsEnabled: settings.diagnostics_enabled,
    });
  }

  browserFallbackState.movement_intensity = settings.movement_intensity;
  browserFallbackState.bubble_frequency = settings.bubble_frequency;
  browserFallbackState.rest_behavior = settings.rest_behavior;
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
