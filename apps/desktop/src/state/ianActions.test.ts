import { describe, expect, it } from "vitest";
import {
  createInitialIanViewState,
  expireBubbleIfNeeded,
  expireRunAroundIfNeeded,
  expireVisualEffectIfNeeded,
  reduceIanActions,
  viewStateForWindowContent,
} from "./ianActions";
import {
  createMovementSequenceGuard,
  durationForDesktopMovement,
  durationForSpeed,
  planDesktopMovementFrames,
} from "./useIanActions";
import type { IanAction, IanState } from "../protocol/generated";

describe("reduceIanActions", () => {
  it("opens a speech bubble from Rust Core actions", () => {
    const actions: IanAction[] = [
      { type: "bubble.open" },
      {
        type: "speech.show",
        text: "我在这儿。",
        mood: "calm",
        duration_ms: 2400,
      },
    ];

    const next = reduceIanActions(createInitialIanViewState(), actions, 1000);

    expect(next.bubble.isOpen).toBe(true);
    expect(next.bubble.text).toBe("我在这儿。");
    expect(next.bubble.mood).toBe("calm");
    expect(next.bubble.visibleUntil).toBe(3400);
  });

  it("keeps Moment bubble copy short and softly timed", () => {
    const next = reduceIanActions(
      createInitialIanViewState(),
      [
        {
          type: "speech.show",
          text: "我在这儿，我会慢慢靠近一下然后就安静待着，不会一直打扰你。",
          mood: "calm",
          duration_ms: 6_000,
        },
      ],
      1_000,
    );

    expect(next.bubble.text).toHaveLength(25);
    expect(next.bubble.text?.endsWith("…")).toBe(true);
    expect(next.bubble.visibleUntil).toBeLessThanOrEqual(4_600);
  });

  it("does not replace visible text while bubble input is active", () => {
    const inputActive = {
      ...createInitialIanViewState(),
      isBubbleInputActive: true,
      bubble: {
        isOpen: true,
        text: "我听着。",
        mood: "calm",
        visibleUntil: null,
      },
    };

    const next = reduceIanActions(
      inputActive,
      [
        {
          type: "speech.show",
          text: "新的主动气泡",
          mood: "happy",
          duration_ms: 2400,
        },
        {
          type: "state.sync",
          state: testIanState({ is_bubble_input_active: true }),
        },
      ],
      1000,
    );

    expect(next.bubble.text).toBe("我听着。");
    expect(next.bubble.mood).toBe("calm");
  });

  it("allows the user submitted reply to replace input text after core ends input", () => {
    const inputActive = {
      ...createInitialIanViewState(),
      isBubbleInputActive: true,
      bubble: {
        isOpen: true,
        text: "我听着。",
        mood: "calm",
        visibleUntil: null,
      },
    };

    const next = reduceIanActions(
      inputActive,
      [
        {
          type: "speech.show",
          text: "喝水水。",
          mood: "calm",
          duration_ms: 2400,
        },
        {
          type: "state.sync",
          state: testIanState({ is_bubble_input_active: false }),
        },
      ],
      1000,
    );

    expect(next.bubble.text).toBe("喝水水。");
    expect(next.isBubbleInputActive).toBe(false);
  });

  function testIanState(overrides: Partial<IanState> = {}): IanState {
    const state: IanState = {
      active_pet_id: "ian-alpaca",
      current_behavior: "idle" as const,
      current_animation: "idle" as const,
      position: { x: 0, y: 0 },
      active_resource_pack: "ian-alpaca",
      behavior_mode: "normal" as const,
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
      quiet_hours: { enabled: false, start_minute: 1320, end_minute: 420 },
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
        enabled: false,
        workspace_id: null,
        display_name: null,
        root_path: null,
      },
      developer_snooze: { enabled: false, until_ms: null, reason: null },
      active_app_category: null,
    };

    return { ...state, ...overrides } as IanState;
  }


  it("closes feedback bubbles after their visible duration", () => {
    const visible = reduceIanActions(
      createInitialIanViewState(),
      [
        {
          type: "speech.show",
          text: "我在这儿。",
          mood: "calm",
          duration_ms: 1200,
        },
      ],
      1000,
    );

    const next = expireBubbleIfNeeded(visible, 2200);

    expect(next.bubble.isOpen).toBe(false);
    expect(next.bubble.text).toBeNull();
  });

  it("plays run animation when Rust Core emits run-around behavior", () => {
    const actions: IanAction[] = [
      { type: "behavior.run_around", duration_ms: 1800 },
    ];

    const next = reduceIanActions(createInitialIanViewState(), actions, 1000);

    expect(next.animation.name).toBe("run");
    expect(next.behavior).toBe("running");
    expect(next.runAroundUntil).toBe(2800);
  });

  it("plays zoomies and visual effects from Rust Core actions", () => {
    const actions: IanAction[] = [
      { type: "behavior.zoomies", duration_ms: 3200, reason: "idle_surprise" },
      {
        type: "effect.play",
        name: "speed_lines",
        intensity: "high",
        duration_ms: 900,
      },
    ];

    const next = reduceIanActions(createInitialIanViewState(), actions, 1000);

    expect(next.animation.name).toBe("zoomies");
    expect(next.behavior).toBe("zooming");
    expect(next.visualEffect?.name).toBe("speed_lines");
  });

  it("expires transient visual effects after their duration", () => {
    const active = reduceIanActions(
      createInitialIanViewState(),
      [
        {
          type: "effect.play",
          name: "heart_pop",
          intensity: "low",
          duration_ms: 900,
        },
      ],
      1_000,
    );

    expect(expireVisualEffectIfNeeded(active, 1_899).visualEffect?.name).toBe(
      "heart_pop",
    );
    expect(expireVisualEffectIfNeeded(active, 1_900).visualEffect).toBeNull();
  });

  it("maps semantic reaction animations into stable view behavior", () => {
    const reactionBehaviors = new Map([
      ["find", "happy"],
      ["wake", "idle"],
      ["wave", "happy"],
      ["affection", "happy"],
      ["tantrum", "running"],
    ]);

    for (const [animation, behavior] of reactionBehaviors) {
      const next = reduceIanActions(
        createInitialIanViewState(),
        [
          {
            type: "animation.play",
            name: animation,
            looped: false,
          } as IanAction,
        ],
      );

      expect(next.behavior).toBe(behavior);
    }
  });

  it("records the latest movement target from Rust Core movement actions", () => {
    const actions: IanAction[] = [
      { type: "movement.move_to", x: 24, y: 36, speed: "normal" },
    ];

    const next = reduceIanActions(createInitialIanViewState(), actions, 1000);

    expect(next.position).toEqual({ x: 24, y: 36 });
    expect(next.movementTarget).toEqual({ x: 24, y: 36, speed: "normal" });
    expect(next.lastMovementAt).toBe(1000);
  });

  it("applies transient appearance scale without changing persisted surface scale", () => {
    const scaled = reduceIanActions(
      createInitialIanViewState(),
      [{ type: "appearance.scale_to", scale: 0.45, duration_ms: 0 }],
      1000,
    );
    const synced = reduceIanActions(
      scaled,
      [{ type: "state.sync", state: testIanState({ surface_scale: 1.2 }) }],
      1200,
    );

    expect(scaled.appearanceScale).toBe(0.45);
    expect(synced.appearanceScale).toBe(0.45);
  });

  it("keeps desktop window content anchored inside the transparent window", () => {
    const state = {
      ...createInitialIanViewState(),
      position: { x: 2840, y: 1570 },
      movementTarget: { x: 2840, y: 1570, speed: "normal" as const },
    };

    const next = viewStateForWindowContent(state, true);

    expect(next.position).toEqual({ x: 0, y: 0 });
    expect(next.movementTarget).toEqual({ x: 0, y: 0, speed: "normal" });
  });

  it("keeps browser preview positions unchanged", () => {
    const state = {
      ...createInitialIanViewState(),
      position: { x: 24, y: 36 },
    };

    expect(viewStateForWindowContent(state, false).position).toEqual({
      x: 24,
      y: 36,
    });
  });

  it("returns to idle after run-around duration expires", () => {
    const running = reduceIanActions(
      createInitialIanViewState(),
      [{ type: "behavior.run_around", duration_ms: 1800 }],
      1000,
    );

    const next = expireRunAroundIfNeeded(running, 2800);

    expect(next.animation.name).toBe("idle");
    expect(next.animation.loop).toBe(true);
    expect(next.behavior).toBe("idle");
    expect(next.runAroundUntil).toBe(0);
  });
});

describe("movement pacing", () => {
  it("keeps desktop moves readable instead of snapping between points", () => {
    expect(durationForSpeed("fast")).toBeGreaterThanOrEqual(340);
    expect(durationForSpeed("normal")).toBeGreaterThanOrEqual(460);
    expect(durationForSpeed("slow")).toBeGreaterThanOrEqual(680);
  });

  it("adds small deterministic variation across chained moves", () => {
    expect(durationForSpeed("fast", 0)).not.toBe(durationForSpeed("fast", 1));
    expect(durationForSpeed("normal", 1)).toBeGreaterThan(
      durationForSpeed("fast", 1),
    );
    expect(durationForSpeed("slow", 2)).toBeGreaterThan(
      durationForSpeed("normal", 2),
    );
  });

  it("plans smooth desktop movement with intermediate frames", () => {
    const frames = planDesktopMovementFrames(
      { x: 10, y: 20 },
      { x: 110, y: 70 },
      "normal",
    );

    expect(frames.length).toBeGreaterThan(8);
    expect(frames[0]).not.toEqual({ x: 110, y: 70 });
    expect(frames.at(-1)).toEqual({ x: 110, y: 70 });
    expect(frames.some((frame) => frame.x > 10 && frame.x < 110)).toBe(true);
    expect(frames.some((frame) => frame.y > 20 && frame.y < 70)).toBe(true);
  });

  it("scales desktop movement duration by travel distance", () => {
    const shortMoveMs = durationForDesktopMovement(
      { x: 10, y: 20 },
      { x: 110, y: 20 },
      "slow",
    );
    const longMoveMs = durationForDesktopMovement(
      { x: 10, y: 20 },
      { x: 10, y: 820 },
      "slow",
    );

    expect(longMoveMs).toBeGreaterThan(shortMoveMs * 4);
    expect(longMoveMs).toBeGreaterThanOrEqual(12_000);
  });

  it("cancels older desktop movement batches only when a newer movement batch starts", () => {
    const guard = createMovementSequenceGuard();
    const patrolBatch: IanAction[] = [
      { type: "movement.move_to", x: 10, y: 10, speed: "slow" },
      { type: "movement.move_to", x: 100, y: 10, speed: "slow" },
    ];
    const nonMovementBatch: IanAction[] = [
      { type: "animation.play", name: "idle", looped: true },
    ];
    const chaseBatch: IanAction[] = [
      { type: "movement.move_to", x: 40, y: 10, speed: "fast" },
    ];

    const patrolSequence = guard.startBatch(patrolBatch);
    const nonMovementSequence = guard.startBatch(nonMovementBatch);

    expect(guard.isCurrent(patrolSequence)).toBe(true);
    expect(nonMovementSequence).toBe(patrolSequence);

    const chaseSequence = guard.startBatch(chaseBatch);

    expect(chaseSequence).not.toBe(patrolSequence);
    expect(guard.isCurrent(patrolSequence)).toBe(false);
    expect(guard.isCurrent(chaseSequence)).toBe(true);
  });
});
