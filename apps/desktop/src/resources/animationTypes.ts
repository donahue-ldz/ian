import type { AnimationName } from "../protocol/generated";

export type AnimationDefinition = {
  frames: number[];
  fps: number;
  loop: boolean;
};

export type AnimationMap = Record<AnimationName, AnimationDefinition>;
