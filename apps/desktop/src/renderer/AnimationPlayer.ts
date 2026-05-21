import type { AnimationName } from "../protocol/generated";
import type { AnimationDefinition } from "../resources/animationTypes";
import type { AnimationManifest } from "../resources/resourceLoader";

export function resolveAnimation(
  manifest: AnimationManifest,
  requested: string,
): AnimationName {
  if (requested in manifest.animations) {
    return requested as AnimationName;
  }

  return "idle";
}

export function resolveAnimationDefinition(
  manifest: AnimationManifest,
  requested: string,
): AnimationDefinition {
  return manifest.animations[resolveAnimation(manifest, requested)];
}

export function resolveAnimationFrame(
  manifest: AnimationManifest,
  requested: string,
  elapsedMs: number,
): number {
  const definition = resolveAnimationDefinition(manifest, requested);

  if (definition.frames.length === 0) {
    return 0;
  }

  const frameDurationMs = 1000 / Math.max(definition.fps, 1);
  const frameIndex = Math.floor(Math.max(elapsedMs, 0) / frameDurationMs);
  const boundedIndex = definition.loop
    ? frameIndex % definition.frames.length
    : Math.min(frameIndex, definition.frames.length - 1);

  return definition.frames[boundedIndex] ?? 0;
}

export function getSpriteSheetFrameCount(manifest: AnimationManifest): number {
  const frameIndexes = Object.values(manifest.animations).flatMap(
    (animation) => animation.frames,
  );

  if (frameIndexes.length === 0) {
    return 0;
  }

  return Math.max(...frameIndexes) + 1;
}
