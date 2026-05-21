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

  return {
    pet,
    animations,
    expressions,
  };
}
