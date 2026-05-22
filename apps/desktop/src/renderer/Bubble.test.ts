import { describe, expect, it } from "vitest";
import { formatBubbleText, normalizeBubbleMessage } from "./bubbleModel";

describe("Bubble", () => {
  it("normalizes a single short message before submit", () => {
    expect(normalizeBubbleMessage("  你在干嘛  ")).toBe("你在干嘛");
  });

  it("folds whitespace and caps submitted bubble messages", () => {
    const message = normalizeBubbleMessage(
      `  ${"Ian ".repeat(30)}  今天    慢慢来  `,
    );

    expect(message).not.toContain("  ");
    expect(Array.from(message ?? "").length).toBeLessThanOrEqual(80);
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
