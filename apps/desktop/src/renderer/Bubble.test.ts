import { describe, expect, it } from "vitest";
import { normalizeBubbleMessage } from "./bubbleModel";

describe("Bubble", () => {
  it("normalizes a single short message before submit", () => {
    expect(normalizeBubbleMessage("  你在干嘛  ")).toBe("你在干嘛");
  });

  it("does not submit empty bubble messages", () => {
    expect(normalizeBubbleMessage("   ")).toBeNull();
  });
});
