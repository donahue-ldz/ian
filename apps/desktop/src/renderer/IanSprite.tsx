import { useEffect, useMemo, useState } from "react";
import type { PetResourcePack } from "../resources/resourceLoader";
import {
  getSpriteSheetFrameCount,
  resolveAnimation,
  resolveAnimationDefinition,
  resolveAnimationFrame,
} from "./AnimationPlayer";

type IanSpriteProps = {
  animation: string;
  resourcePack: PetResourcePack | null;
};

const FALLBACK_FRAME_MS = 500;

export function IanSprite({ animation, resourcePack }: IanSpriteProps) {
  const [elapsedMs, setElapsedMs] = useState(0);
  const spritePath = resourcePack?.pet.sprite
    ? `/resources/pets/${resourcePack.pet.id}/${resourcePack.pet.sprite}`
    : null;

  const resolvedAnimation = resourcePack
    ? resolveAnimation(resourcePack.animations, animation)
    : "idle";
  const animationDefinition = resourcePack
    ? resolveAnimationDefinition(resourcePack.animations, resolvedAnimation)
    : null;
  const frameCount = resourcePack
    ? getSpriteSheetFrameCount(resourcePack.animations)
    : 0;
  const spriteFrame = resourcePack
    ? resolveAnimationFrame(resourcePack.animations, resolvedAnimation, elapsedMs)
    : 0;

  useEffect(() => {
    setElapsedMs(0);
  }, [resolvedAnimation]);

  useEffect(() => {
    if (!animationDefinition) {
      return;
    }

    const frameDurationMs =
      1000 / Math.max(animationDefinition.fps, 1) || FALLBACK_FRAME_MS;
    const interval = window.setInterval(() => {
      setElapsedMs((current) => current + frameDurationMs);
    }, frameDurationMs);

    return () => window.clearInterval(interval);
  }, [animationDefinition]);

  const frameStyle = useMemo(() => {
    if (!resourcePack || !spritePath || frameCount === 0) {
      return undefined;
    }

    const { frameHeight, frameWidth, scale } = resourcePack.animations.meta;
    const renderedWidth = frameWidth * scale;
    const renderedHeight = frameHeight * scale;

    return {
      width: `${renderedWidth}px`,
      height: `${renderedHeight}px`,
      backgroundImage: `url("${spritePath}")`,
      backgroundPosition: `-${spriteFrame * renderedWidth}px 0px`,
      backgroundSize: `${frameCount * renderedWidth}px ${renderedHeight}px`,
    };
  }, [frameCount, resourcePack, spriteFrame, spritePath]);

  if (frameStyle) {
    return (
      <span
        className="ian-sprite ian-sprite--asset"
        data-animation={resolvedAnimation}
      >
        <span
          className="ian-sprite-frame"
          role="img"
          aria-label="Ian"
          style={frameStyle}
        />
      </span>
    );
  }

  return (
    <span
      className="ian-sprite ian-sprite--fallback"
      data-animation={resolvedAnimation}
    >
      <span className="ian-shadow" />
      <span className="ian-body" />
      <span className="ian-head" />
      <span className="ian-ear ian-ear-left" />
      <span className="ian-ear ian-ear-right" />
      <span className="ian-face">
        <span className="ian-eye ian-eye-left" />
        <span className="ian-eye ian-eye-right" />
        <span className="ian-mouth" />
      </span>
      <span className="ian-leg ian-leg-left" />
      <span className="ian-leg ian-leg-right" />
    </span>
  );
}
