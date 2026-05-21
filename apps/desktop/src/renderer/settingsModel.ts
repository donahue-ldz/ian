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

export const developerCapabilityOptions = [
  {
    capability: "git_metadata",
    label: "Git 元数据",
    scope: "分支、dirty 状态、短 hash；不读取代码正文或 diff",
  },
  {
    capability: "build_test_events",
    label: "构建测试摘要",
    scope: "状态、耗时、测试数量、错误类别；不读取终端全文",
  },
  {
    capability: "keyboard_rhythm",
    label: "键盘节奏",
    scope: "时间窗口、强度、计数；不记录按键内容",
  },
  {
    capability: "active_app_presence",
    label: "应用类别",
    scope: "粗粒度类别；不读取窗口标题、URL 或屏幕文字",
  },
] as const;

export const developerSnoozeOptions = [
  { label: "关闭", value: "off", durationMs: 0 },
  { label: "30 分钟", value: "30m", durationMs: 30 * 60 * 1000 },
  { label: "2 小时", value: "2h", durationMs: 2 * 60 * 60 * 1000 },
  { label: "今天", value: "today", durationMs: null },
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
