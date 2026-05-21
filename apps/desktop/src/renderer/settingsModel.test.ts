import { describe, expect, it } from "vitest";
import {
  behaviorModeOptions,
  bubbleFrequencyOptions,
  isBehaviorMode,
  movementIntensityOptions,
  restBehaviorOptions,
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
  });

  it("formats quiet hour minutes as time inputs", () => {
    expect(toTimeInputValue(22 * 60 + 5)).toBe("22:05");
    expect(toTimeInputValue(7 * 60)).toBe("07:00");
  });
});
