export type AnimationDefinition = {
  frames: number[];
  fps: number;
  loop: boolean;
};

export type AnimationMap = Record<string, AnimationDefinition>;
