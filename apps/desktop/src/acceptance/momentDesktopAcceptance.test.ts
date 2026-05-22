import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

describe("moment desktop acceptance suite", () => {
  it("documents objective desktop checks for every moment slice", () => {
    const doc = readFileSync(
      new URL("../../../../docs/sdd/moment-desktop-acceptance.md", import.meta.url),
      "utf8",
    );

    for (const section of [
      "Find Ian Entrance",
      "Pointer Curiosity",
      "Drag Carry",
      "Drop Settle",
      "Rare Idle Surprise",
      "Memory Echo",
      "Cooldown And Budget",
      "Reduced Motion And DND",
    ]) {
      expect(doc).toContain(`## ${section}`);
    }

    expect(doc.match(/Pass condition:/g)).toHaveLength(8);
    expect(doc.match(/Fail condition:/g)).toHaveLength(8);
    expect(doc).toContain("真实 Tauri 桌面壳");
  });
});
