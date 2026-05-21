import { describe, expect, it } from "vitest";
import {
  behaviorModeOptions,
  bubbleFrequencyOptions,
  developerCapabilityOptions,
  developerSnoozeOptions,
  isBehaviorMode,
  movementIntensityOptions,
  playfulEnergyOptions,
  restBehaviorOptions,
  settingsGroups,
  toTimeInputValue,
} from "./settingsModel";

describe("settingsModel", () => {
  it("accepts only supported behavior modes", () => {
    expect(isBehaviorMode("quiet")).toBe(true);
    expect(isBehaviorMode("normal")).toBe(true);
    expect(isBehaviorMode("lively")).toBe(true);
    expect(isBehaviorMode("developer")).toBe(false);
  });

  it("keeps behavior modes small and P0-friendly", () => {
    expect(behaviorModeOptions.map((option) => option.value)).toEqual([
      "quiet",
      "normal",
      "lively",
    ]);
  });

  it("exposes compact creature life setting options", () => {
    expect(movementIntensityOptions.map((option) => option.value)).toEqual([
      "low",
      "normal",
      "high",
    ]);
    expect(bubbleFrequencyOptions.map((option) => option.value)).toEqual([
      "quiet",
      "normal",
      "chatty",
    ]);
    expect(restBehaviorOptions.map((option) => option.value)).toEqual([
      "restful",
      "normal",
      "active",
    ]);
    expect(playfulEnergyOptions.map((option) => option.value)).toEqual([
      "off",
      "low",
      "normal",
      "high",
    ]);
  });

  it("formats quiet hour minutes as time inputs", () => {
    expect(toTimeInputValue(22 * 60 + 5)).toBe("22:05");
    expect(toTimeInputValue(7 * 60)).toBe("07:00");
  });

  it("describes developer rhythm capabilities with privacy scope", () => {
    expect(developerCapabilityOptions.map((option) => option.capability)).toEqual([
      "git_metadata",
      "build_test_events",
      "keyboard_rhythm",
      "active_app_presence",
    ]);
    expect(developerCapabilityOptions.find((option) => option.capability === "git_metadata")?.scope)
      .toContain("不读取代码正文");
    expect(
      developerCapabilityOptions.find((option) => option.capability === "keyboard_rhythm")?.scope,
    ).toContain("不记录按键内容");
  });

  it("organizes settings around user mental models", () => {
    expect(settingsGroups.map((group) => group.label)).toEqual([
      "性格",
      "生活",
      "打扰",
      "隐私",
      "高级",
    ]);
  });

  it("keeps technical capability names out of the primary settings labels", () => {
    expect(developerCapabilityOptions.map((option) => option.label)).toEqual([
      "项目状态",
      "构建测试",
      "键盘节奏",
      "当前应用",
    ]);
  });

  it("explains every sensitive capability with read, non-read and disable copy", () => {
    for (const option of developerCapabilityOptions) {
      expect(option.reads).toBeTruthy();
      expect(option.doesNotRead).toBeTruthy();
      expect(option.canDisable).toBe("可随时关闭");
    }
  });

  it("offers compact developer rhythm snooze durations", () => {
    expect(developerSnoozeOptions.map((option) => option.value)).toEqual([
      "off",
      "30m",
      "2h",
      "today",
    ]);
  });
});
