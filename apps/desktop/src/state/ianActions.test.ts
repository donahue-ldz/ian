import { describe, expect, it } from "vitest";
import {
  createInitialIanViewState,
  expireRunAroundIfNeeded,
  reduceIanActions,
} from "./ianActions";
import type { IanAction } from "../protocol/generated";

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

    const next = reduceIanActions(createInitialIanViewState(), actions);

    expect(next.bubble.isOpen).toBe(true);
    expect(next.bubble.text).toBe("我在这儿。");
    expect(next.bubble.mood).toBe("calm");
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

  it("records the latest movement target from Rust Core movement actions", () => {
    const actions: IanAction[] = [
      { type: "movement.move_to", x: 24, y: 36, speed: "normal" },
    ];

    const next = reduceIanActions(createInitialIanViewState(), actions, 1000);

    expect(next.position).toEqual({ x: 24, y: 36 });
    expect(next.movementTarget).toEqual({ x: 24, y: 36, speed: "normal" });
    expect(next.lastMovementAt).toBe(1000);
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
