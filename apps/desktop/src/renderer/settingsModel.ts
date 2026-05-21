import type { BehaviorMode } from "../protocol/generated";

export const behaviorModeOptions: Array<{
  label: string;
  value: BehaviorMode;
}> = [
  { label: "安静", value: "quiet" },
  { label: "正常", value: "normal" },
  { label: "活泼", value: "lively" },
];

export const movementIntensityOptions = [
  { label: "少动", value: "low" },
  { label: "适中", value: "normal" },
  { label: "活跃", value: "high" },
] as const;

export const bubbleFrequencyOptions = [
  { label: "少说", value: "quiet" },
  { label: "正常", value: "normal" },
  { label: "爱说", value: "chatty" },
] as const;

export const restBehaviorOptions = [
  { label: "多休息", value: "restful" },
  { label: "正常", value: "normal" },
  { label: "少休息", value: "active" },
] as const;

export function isBehaviorMode(value: string): value is BehaviorMode {
  return value === "quiet" || value === "normal" || value === "lively";
}

export function toTimeInputValue(totalMinutes: number): string {
  const normalized = Math.max(0, Math.min(24 * 60 - 1, totalMinutes));
  const hours = Math.floor(normalized / 60);
  const minutes = normalized % 60;
  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}`;
}

export function fromTimeInputValue(value: string): number {
  const [hours, minutes] = value.split(":").map((part) => Number(part));
  if (!Number.isFinite(hours) || !Number.isFinite(minutes)) {
    return 0;
  }
  return Math.max(0, Math.min(24 * 60, hours * 60 + minutes));
}
