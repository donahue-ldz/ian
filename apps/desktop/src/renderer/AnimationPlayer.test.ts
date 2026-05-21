import { describe, expect, it } from "vitest";
import {
  getSpriteSheetFrameCount,
  resolveAnimation,
  resolveAnimationFrame,
} from "./AnimationPlayer";
import type { AnimationManifest } from "../resources/resourceLoader";

const manifest: AnimationManifest = {
  meta: {
    frameWidth: 96,
    frameHeight: 96,
    scale: 2,
  },
  animations: {
    idle: {
      frames: [0, 1],
      fps: 2,
      loop: true,
    },
    rest: {
      frames: [1],
      fps: 1,
      loop: true,
    },
    walk: {
      frames: [2, 3, 4, 5],
      fps: 8,
      loop: true,
    },
    sleep: {
      frames: [6, 7],
      fps: 1,
      loop: true,
    },
    run: {
      frames: [8, 9, 10, 11],
      fps: 12,
      loop: true,
    },
    zoomies: {
      frames: [8, 12, 9, 13],
      fps: 14,
      loop: true,
    },
    happy: {
      frames: [12, 13],
      fps: 6,
      loop: false,
    },
  },
};

describe("AnimationPlayer", () => {
  it("falls back to idle for unknown animations", () => {
    expect(resolveAnimation(manifest, "spin")).toBe("idle");
  });

  it("selects looped frames from elapsed time and fps", () => {
    expect(resolveAnimationFrame(manifest, "run", 0)).toBe(8);
    expect(resolveAnimationFrame(manifest, "run", 84)).toBe(9);
    expect(resolveAnimationFrame(manifest, "run", 334)).toBe(8);
  });

  it("holds the last frame for non-looping animations", () => {
    expect(resolveAnimationFrame(manifest, "happy", 0)).toBe(12);
    expect(resolveAnimationFrame(manifest, "happy", 1000)).toBe(13);
  });

  it("counts sprite sheet frames from animation manifests", () => {
    expect(getSpriteSheetFrameCount(manifest)).toBe(14);
  });
});
