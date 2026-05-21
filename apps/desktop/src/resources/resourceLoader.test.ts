import { describe, expect, it } from "vitest";
import kittenAnimations from "../../public/resources/pets/ian-kitten/animations.json";
import kittenExpressions from "../../public/resources/pets/ian-kitten/expressions.json";
import kittenPet from "../../public/resources/pets/ian-kitten/pet.json";
import {
  getResourcePackVersion,
  type AnimationManifest,
  resolveAnimationWithFallback,
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
  expressions: { expressions: {} },
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

  it("validates the checked-in kitten resource pack contract", () => {
    const kittenPack: PetResourcePack = {
      pet: kittenPet,
      animations: kittenAnimations,
      expressions: kittenExpressions,
    };

    validatePetResourcePack(kittenPack);

    expect(kittenPack.pet.id).toBe("ian-kitten");
    expect(kittenPack.pet.species).toBe("cat");
    expect(kittenPack.pet.sprite).toBe("sprite.svg");
    expect(getSpriteFrameCount(kittenPack.animations)).toBe(14);
    expect(Object.keys(kittenPack.animations.animations).sort()).toEqual([
      "happy",
      "idle",
      "run",
      "sleep",
      "walk",
    ]);
  });
});

function getSpriteFrameCount(manifest: AnimationManifest): number {
  const maxFrame = Math.max(
    ...Object.values(manifest.animations).flatMap(
      (animation) => animation.frames,
    ),
  );

  return maxFrame + 1;
}
