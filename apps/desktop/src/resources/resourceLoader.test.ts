import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import {
  buildAnimationCoverageMatrix,
  getResourcePackVersion,
  REQUIRED_SEMANTIC_ANIMATIONS,
  resolveExpressionWithFallback,
  resolveAnimationWithFallback,
  validatePetManifestFields,
  validatePetResourcePack,
  type PetResourcePack,
} from "./resourceLoader";

const DEFAULT_RESOURCE_PACK_IDS = [
  "ian-alpaca",
  "ian-kitten",
  "ian-puppy",
  "ian-adventurer",
] as const;
const REQUIRED_ANIMATIONS = [
  "idle",
  "walk",
  "run",
  "zoomies",
  "happy",
  "rest",
  "sleep",
];

const pack: PetResourcePack = {
  pet: {
    id: "ian-alpaca",
    name: "Ian",
    version: "0.1.0",
    species: "alpaca",
    defaultPersonality: "calm",
    sprite: "sprite.svg",
    animations: "animations.json",
    expressions: "expressions.json",
    sounds: "sounds",
    capabilities: [],
  },
  animations: {
    meta: { frameWidth: 96, frameHeight: 96, scale: 2 },
    animations: {
      idle: { frames: [0], fps: 1, loop: true },
      rest: { frames: [0], fps: 1, loop: true },
      walk: { frames: [1], fps: 1, loop: true },
      happy: { frames: [2], fps: 1, loop: false },
      run: { frames: [3], fps: 1, loop: true },
      zoomies: { frames: [3, 2], fps: 2, loop: true },
      sleep: { frames: [4], fps: 1, loop: true },
    },
  },
  expressions: {
    expressions: {
      idle: { overlay: null },
      happy: { overlay: "blush" },
    },
  },
};

describe("resourceLoader contract", () => {
  it("validates the minimum resource pack contract", () => {
    expect(() => validatePetResourcePack(pack)).not.toThrow();
    expect(getResourcePackVersion(pack)).toBe("0.1.0");
  });

  it("throws a handled error when idle animation is missing", () => {
    const invalid = {
      ...pack,
      animations: {
        ...pack.animations,
        animations: { ...pack.animations.animations, idle: undefined },
      },
    } as unknown as PetResourcePack;

    expect(() => validatePetResourcePack(invalid)).toThrow(
      "Resource pack must define idle animation",
    );
  });

  it("throws a handled error when any required semantic animation is missing", () => {
    const invalid = {
      ...pack,
      animations: {
        ...pack.animations,
        animations: { ...pack.animations.animations, run: undefined },
      },
    } as unknown as PetResourcePack;

    expect(() => validatePetResourcePack(invalid)).toThrow(
      "Resource pack must define run animation",
    );
  });

  it("exposes preview metadata for every built-in resource pack", async () => {
    const { BUILT_IN_PET_RESOURCE_PACKS } = await import("./resourceLoader");

    for (const option of BUILT_IN_PET_RESOURCE_PACKS) {
      expect(option.previewAnimation).toBeTruthy();
      expect(option.previewTone).toBeTruthy();
    }
  });

  it("falls back unknown animations to idle", () => {
    expect(resolveAnimationWithFallback(pack, "jump")).toBe("idle");
    expect(resolveAnimationWithFallback(pack, "run")).toBe("run");
  });

  it("resolves expressions with an idle fallback", () => {
    expect(resolveExpressionWithFallback(pack, "happy")).toEqual({
      overlay: "blush",
    });
    expect(resolveExpressionWithFallback(pack, "curious")).toEqual({
      overlay: null,
    });
  });

  it("reports concrete manifest field errors for resource authors", () => {
    expect(() =>
      validatePetManifestFields({ ...pack.pet, animations: "" }),
    ).toThrow("pet.animations");
  });

  it("keeps default resource pack animation states distinct and inside the sprite sheet", () => {
    for (const packId of DEFAULT_RESOURCE_PACK_IDS) {
      const pet = readPublicJson<PetResourcePack["pet"]>(
        `../../public/resources/pets/${packId}/pet.json`,
      );
      const animations = readPublicJson<PetResourcePack["animations"]>(
        `../../public/resources/pets/${packId}/${pet.animations}`,
      );
      const sprite = readPublicText(
        `../../public/resources/pets/${packId}/${pet.sprite}`,
      );
      const frameCapacity = readSvgWidth(sprite) / animations.meta.frameWidth;

      for (const animationName of REQUIRED_ANIMATIONS) {
        expect(pet.capabilities, `${packId} capabilities`).toContain(
          animationName,
        );
        expect(
          animations.animations[animationName as keyof typeof animations.animations],
          `${packId} ${animationName}`,
        ).toBeDefined();
      }

      const idleFrames = animations.animations.idle.frames.join(",");
      expect(animations.animations.rest.frames.join(",")).not.toBe(idleFrames);
      expect(animations.animations.sleep.frames.join(",")).not.toBe(idleFrames);

      if (packId === "ian-puppy") {
        expect(animations.animations.idle.frames.length).toBeGreaterThanOrEqual(2);
        expect(animations.animations.walk.fps).toBeLessThanOrEqual(5);
        expect(animations.animations.run.fps).toBeLessThanOrEqual(8);
        expect(animations.animations.zoomies.fps).toBeLessThanOrEqual(10);
        expect(animations.animations.zoomies.frames.join(",")).not.toBe(
          animations.animations.run.frames.join(","),
        );
        expect(sprite).toContain('data-style="rounded-puppy-v2"');
      }

      if (packId === "ian-adventurer") {
        expect(pet.species).toBe("little_adventurer");
        expect(frameCapacity).toBeGreaterThanOrEqual(32);
        expect(animations.animations.idle.frames.length).toBeGreaterThanOrEqual(5);
        expect(animations.animations.run.frames.length).toBeGreaterThanOrEqual(4);
        expect(animations.animations.zoomies.frames.join(",")).not.toBe(
          animations.animations.run.frames.join(","),
        );
        expect(sprite).toContain('data-style="pixel-adventurer-v1"');
        expect(sprite).toContain('data-style-kind="pixel-sprite"');
        expect(sprite).toContain('data-line-tone="soft-arcade"');
        expect(sprite).toContain('data-pixel-grid="4"');
        expect(sprite).toContain('data-prop="glow-staff"');
        expect(sprite).toContain('data-prop="soft-scarf"');
        expect(sprite).toContain('data-prop="small-backpack"');
        expect(pet.capabilities).toEqual(
          expect.arrayContaining(["find", "wake", "wave", "affection", "tantrum"]),
        );
        for (const semantic of ["find", "wake", "wave", "affection", "tantrum"]) {
          expect(
            animations.animations[semantic as keyof typeof animations.animations],
            `${packId} ${semantic}`,
          ).toBeDefined();
        }
      }

      for (const [animationName, animation] of Object.entries(
        animations.animations,
      )) {
        expect(animation.frames.length, `${packId} ${animationName}`).toBeGreaterThan(
          0,
        );
        for (const frame of animation.frames) {
          expect(frame, `${packId} ${animationName}`).toBeGreaterThanOrEqual(0);
          expect(frame, `${packId} ${animationName}`).toBeLessThan(frameCapacity);
        }
      }
    }
  });

  it("builds an animation coverage matrix with explicit fallbacks for every built-in pack", () => {
    for (const packId of DEFAULT_RESOURCE_PACK_IDS) {
      const pet = readPublicJson<PetResourcePack["pet"]>(
        `../../public/resources/pets/${packId}/pet.json`,
      );
      const animations = readPublicJson<PetResourcePack["animations"]>(
        `../../public/resources/pets/${packId}/${pet.animations}`,
      );
      const matrix = buildAnimationCoverageMatrix({ ...pack, pet, animations });

      expect(matrix.map((row) => row.semantic)).toEqual(REQUIRED_SEMANTIC_ANIMATIONS);
      for (const row of matrix) {
        expect(row.animation).toBeTruthy();
        expect(row.status === "direct" || row.fallback).toBeTruthy();
      }
    }
  });
});

function readPublicJson<T>(path: string): T {
  return JSON.parse(readPublicText(path)) as T;
}

function readPublicText(path: string): string {
  return readFileSync(new URL(path, import.meta.url), "utf8");
}

function readSvgWidth(svg: string): number {
  const width = svg.match(/<svg[^>]+width="(?<width>\d+)"/)?.groups?.width;
  if (!width) {
    throw new Error("Sprite sheet SVG must declare a numeric width");
  }

  return Number(width);
}
