import { invoke } from "@tauri-apps/api/core";
import type {
  BehaviorMode,
  IanAction,
  IanEvent,
  IanState,
  Position,
} from "../protocol/generated";

const browserFallbackState: IanState = {
  active_pet_id: "ian-kitten",
  current_behavior: "idle",
  current_animation: "idle",
  position: { x: 0, y: 0 },
  active_resource_pack: "ian-kitten",
  behavior_mode: "normal",
};

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

  if (event.type === "mouse.double_click") {
    return [{ type: "behavior.run_around", duration_ms: 1800 }];
  }

  if (event.type === "mouse.click") {
    return [
      { type: "bubble.open" },
      { type: "speech.show", text: "我在这儿。", mood: "calm", duration_ms: 2400 },
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
