import { describe, expect, it } from "vitest";
import { behaviorModeOptions, isBehaviorMode } from "./settingsModel";

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
});
