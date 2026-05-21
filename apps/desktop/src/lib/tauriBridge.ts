import { invoke } from "@tauri-apps/api/core";
import type {
  BehaviorMode,
  IanAction,
  IanEvent,
  IanState,
  Position,
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
};
let browserInteractionCount = 0;

function isTauriRuntime(): boolean {
  return "__TAURI_INTERNALS__" in window;
}

export async function sendIanEvent(event: IanEvent): Promise<IanAction[]> {
  if (isTauriRuntime()) {
    return invoke<IanAction[]>("handle_ian_event", { event });
  }

  if (event.type === "mouse.near") {
    return [{ type: "animation.play", name: "happy", looped: false }];
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
    const text =
      browserInteractionCount >= 2 ? "再摸摸也可以。" : "我在这儿。";

    return [
      { type: "bubble.open" },
      { type: "speech.show", text, mood: "calm", duration_ms: 2400 },
      { type: "animation.play", name: "happy", looped: false },
    ];
  }

  if (event.type === "dialogue.user_message") {
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
