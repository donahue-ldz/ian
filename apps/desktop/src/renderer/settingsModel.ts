import type { BehaviorMode } from "../protocol/generated";

export const behaviorModeOptions: Array<{
  label: string;
  value: BehaviorMode;
}> = [
  { label: "安静", value: "quiet" },
  { label: "正常", value: "normal" },
  { label: "活泼", value: "lively" },
];

export function isBehaviorMode(value: string): value is BehaviorMode {
  return value === "quiet" || value === "normal" || value === "lively";
}
