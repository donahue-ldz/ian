import { describe, expect, it } from "vitest";
import {
  getResourcePackVersion,
  resolveExpressionWithFallback,
  resolveAnimationWithFallback,
  validatePetManifestFields,
  validatePetResourcePack,
  type PetResourcePack,
} from "./resourceLoader";

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
      walk: { frames: [1], fps: 1, loop: true },
      happy: { frames: [2], fps: 1, loop: false },
      run: { frames: [3], fps: 1, loop: true },
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
});
