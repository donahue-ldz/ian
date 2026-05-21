import { describe, expect, it } from "vitest";
import { formatBubbleText, normalizeBubbleMessage } from "./bubbleModel";

describe("Bubble", () => {
  it("normalizes a single short message before submit", () => {
    expect(normalizeBubbleMessage("  你在干嘛  ")).toBe("你在干嘛");
  });

  it("does not submit empty bubble messages", () => {
    expect(normalizeBubbleMessage("   ")).toBeNull();
  });

  it("keeps default feedback bubble text short", () => {
    expect(
      formatBubbleText("  我在这里陪你，不用急，慢慢来，先把手上的事情做好就可以。  "),
    ).toBe("我在这里陪你，不用急，慢慢来，先把手上的事情做好…");
  });
});
