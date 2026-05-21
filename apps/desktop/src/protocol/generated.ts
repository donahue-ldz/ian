// Generated from Rust protocol types by ts-rs.
// P0 keeps this checked in so the frontend can typecheck before a Rust toolchain is available.

export type AnimationName = "idle" | "walk" | "happy" | "run" | "sleep";
export type MovementSpeed = "slow" | "normal" | "fast";
export type CurrentBehavior = "idle" | "walking" | "happy" | "running" | "sleeping";
export type BehaviorMode = "quiet" | "normal" | "lively";

export type Position = {
  x: number;
  y: number;
};

export type IanState = {
  active_pet_id: string;
  current_behavior: CurrentBehavior;
  current_animation: AnimationName;
  position: Position;
  active_resource_pack: string;
  behavior_mode: BehaviorMode;
};

export type IanEvent =
  | { type: "app.started" }
  | { type: "time.tick"; now_ms: number }
  | { type: "mouse.click"; x: number; y: number }
  | { type: "mouse.double_click"; x: number; y: number }
  | { type: "mouse.drag_start"; x: number; y: number }
  | { type: "mouse.drag_end"; x: number; y: number }
  | { type: "dialogue.user_message"; text: string };

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
  | { type: "state.sync"; state: IanState };
