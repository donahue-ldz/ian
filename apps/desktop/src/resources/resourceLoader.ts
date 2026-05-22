import type { AnimationMap } from "./animationTypes";

export type PetManifest = {
  id: string;
  name: string;
  version: string;
  species: string;
  defaultPersonality: string;
  sprite: string;
  animations: string;
  expressions: string;
  sounds: string;
  capabilities: string[];
};

export type AnimationManifest = {
  meta: {
    frameWidth: number;
    frameHeight: number;
    scale: number;
  };
  animations: AnimationMap;
};

export type ExpressionManifest = {
  expressions: Record<string, { overlay: string | null }>;
};

export type PetResourcePack = {
  pet: PetManifest;
  animations: AnimationManifest;
  expressions: ExpressionManifest;
};

export type PetResourcePackOption = {
  id: string;
  label: string;
  previewAnimation: string;
  previewTone: string;
};

export const BUILT_IN_PET_RESOURCE_PACKS: PetResourcePackOption[] = [
  { id: "ian-adventurer", label: "小冒险家", previewAnimation: "happy", previewTone: "像素" },
  { id: "ian-puppy", label: "小狗", previewAnimation: "happy", previewTone: "圆润" },
  { id: "ian-kitten", label: "小猫", previewAnimation: "idle", previewTone: "轻巧" },
  { id: "ian-alpaca", label: "羊驼", previewAnimation: "idle", previewTone: "安静" },
];

export class ResourcePackError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ResourcePackError";
  }
}

async function loadJson<T>(path: string): Promise<T> {
  const response = await fetch(path);
  if (!response.ok) {
    throw new Error(`Failed to load resource pack file: ${path}`);
  }
  return (await response.json()) as T;
}

export async function loadPetResourcePack(id: string): Promise<PetResourcePack> {
  const base = `/resources/pets/${id}`;
  const pet = await loadJson<PetManifest>(`${base}/pet.json`);
  const [animations, expressions] = await Promise.all([
    loadJson<AnimationManifest>(`${base}/${pet.animations}`),
    loadJson<ExpressionManifest>(`${base}/${pet.expressions}`),
  ]);

  const pack = {
    pet,
    animations,
    expressions,
  };

  validatePetResourcePack(pack);
  return pack;
}

export function validatePetResourcePack(pack: PetResourcePack): void {
  validatePetManifestFields(pack.pet);

  const { frameHeight, frameWidth, scale } = pack.animations.meta;
  if (frameWidth <= 0 || frameHeight <= 0 || scale <= 0) {
    throw new ResourcePackError("Resource pack animation meta must be positive");
  }

  for (const animationName of [
    "idle",
    "walk",
    "run",
    "zoomies",
    "happy",
    "rest",
    "sleep",
  ] as const) {
    const animation = pack.animations.animations[animationName];
    if (!animation || animation.frames.length === 0) {
      throw new ResourcePackError(
        `Resource pack must define ${animationName} animation`,
      );
    }
  }
}

export function validatePetManifestFields(pet: PetManifest): void {
  const requiredPetFields: Array<keyof PetManifest> = [
    "id",
    "name",
    "version",
    "sprite",
    "animations",
    "expressions",
  ];

  for (const field of requiredPetFields) {
    if (!pet[field]) {
      throw new ResourcePackError(`Resource pack is missing pet.${field}`);
    }
  }
}

export function getResourcePackVersion(pack: PetResourcePack): string {
  return pack.pet.version;
}

export function resolveAnimationWithFallback(
  pack: PetResourcePack,
  requested: string,
): keyof AnimationMap {
  if (requested in pack.animations.animations) {
    return requested as keyof AnimationMap;
  }

  return "idle";
}

export function resolveExpressionWithFallback(
  pack: PetResourcePack,
  requested: string,
): { overlay: string | null } {
  return (
    pack.expressions.expressions[requested] ??
    pack.expressions.expressions.idle ?? { overlay: null }
  );
}
