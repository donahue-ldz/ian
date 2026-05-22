// Generated from Rust protocol types by ts-rs.
// P0 keeps this checked in so the frontend can typecheck before a Rust toolchain is available.

export type AnimationName =
  | "idle"
  | "rest"
  | "walk"
  | "happy"
  | "run"
  | "zoomies"
  | "sleep"
  | "find"
  | "wave"
  | "affection";
export type MovementSpeed = "slow" | "normal" | "fast";
export type CurrentBehavior = "idle" | "resting" | "walking" | "happy" | "running" | "zooming" | "sleeping";
export type BehaviorMode = "quiet" | "normal" | "lively";
export type BuildTestStatus = "success" | "failure";
export type PlayfulEnergy = "off" | "low" | "normal" | "high";
export type PlayfulState = "idle" | "warming_up" | "zooming" | "settling" | "cooling_down";

export type Position = {
  x: number;
  y: number;
};

export type ScreenBounds = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type QuietHours = {
  enabled: boolean;
  start_minute: number;
  end_minute: number;
};

export type DeveloperWorkspace = {
  bound: boolean;
  workspace_id?: string | null;
  display_name?: string | null;
  root_path?: string | null;
  enabled: boolean;
};

export type DeveloperSnooze = {
  enabled: boolean;
  until_ms?: number | null;
  reason?: string | null;
};

export type PlayfulDiagnostic = {
  timestamp_ms: number;
  reason: string;
  result: string;
  cooldown_key?: string | null;
  chosen_reaction_key?: string | null;
};

export type IanState = {
  active_pet_id: string;
  current_behavior: CurrentBehavior;
  current_animation: AnimationName;
  position: Position;
  active_resource_pack: string;
  behavior_mode: BehaviorMode;
  reminders_enabled: boolean;
  reminder_interval_minutes: number;
  do_not_disturb: boolean;
  byom_enabled: boolean;
  byom_key_configured: boolean;
  git_metadata_enabled: boolean;
  build_test_events_enabled: boolean;
  keyboard_rhythm_enabled: boolean;
  active_app_presence_enabled: boolean;
  privacy_onboarding_seen: boolean;
  find_ian_shortcut_enabled: boolean;
  find_ian_shortcut: string;
  home_anchor: Position;
  screen_bounds?: ScreenBounds | null;
  last_user_interaction_ms: number;
  quiet_hours: QuietHours;
  movement_intensity: string;
  bubble_frequency: string;
  rest_behavior: string;
  playful_energy: PlayfulEnergy;
  playful_state: PlayfulState;
  playful_state_until_ms?: number | null;
  playful_snoozed_until_ms?: number | null;
  last_playful_diagnostic?: PlayfulDiagnostic | null;
  surface_scale: number;
  diagnostics_enabled: boolean;
  day_phase: string;
  is_dragging: boolean;
  is_bubble_input_active: boolean;
  developer_workspace: DeveloperWorkspace;
  developer_snooze: DeveloperSnooze;
  active_app_category?: string | null;
};

export type IanEvent =
  | { type: "app.started" }
  | { type: "time.tick"; now_ms: number }
  | { type: "mouse.click"; x: number; y: number }
  | { type: "mouse.double_click"; x: number; y: number }
  | { type: "mouse.near"; x: number; y: number; now_ms: number }
  | { type: "mouse.leave"; x: number; y: number }
  | { type: "mouse.drag_start"; x: number; y: number }
  | { type: "mouse.drag_end"; x: number; y: number }
  | { type: "mouse.chase_candidate"; x: number; y: number; now_ms: number }
  | { type: "screen.bounds"; x: number; y: number; width: number; height: number }
  | { type: "dialogue.user_message"; text: string }
  | { type: "bubble.input_started" }
  | { type: "bubble.input_ended" }
  | {
      type: "developer.git_status_changed";
      workspace_id?: string | null;
      branch: string;
      dirty: boolean;
      short_commit: string;
    }
  | {
      type: "developer.build_test_summary";
      workspace_id?: string | null;
      tool: string;
      status: BuildTestStatus;
      duration_ms: number;
      tests_total: number;
      tests_failed: number;
      error_kind?: string | null;
    }
  | { type: "keyboard.rhythm"; window_ms: number; intensity: string; count: number }
  | {
      type: "active_app.presence";
      category: string;
      confidence: number;
      app_id?: string | null;
    }
  | { type: "system.shortcut_triggered"; action: string; now_ms: number }
  | { type: "moment.debug_trigger"; kind: string; now_ms: number };

export type IanAction =
  | { type: "animation.play"; name: AnimationName; looped: boolean }
  | { type: "movement.move_to"; x: number; y: number; speed: MovementSpeed }
  | {
      type: "speech.show";
      text: string;
      mood?: string | null;
      duration_ms?: number | null;
    }
  | { type: "bubble.open" }
  | { type: "bubble.close" }
  | { type: "behavior.run_around"; duration_ms: number }
  | { type: "behavior.zoomies"; duration_ms: number; reason: string }
  | { type: "effect.play"; name: string; intensity: string; duration_ms: number }
  | { type: "appearance.scale_to"; scale: number; duration_ms: number }
  | { type: "playful.state"; state: PlayfulState; until_ms?: number | null }
  | {
      type: "playful.diagnostic";
      timestamp_ms: number;
      reason: string;
      result: string;
      cooldown_key?: string | null;
      chosen_reaction_key?: string | null;
    }
  | { type: "state.sync"; state: IanState };
