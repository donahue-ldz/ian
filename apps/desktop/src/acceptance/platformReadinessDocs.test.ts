import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

describe("future platform readiness docs", () => {
  it("keeps resource import productization gated", () => {
    const doc = readDoc("../../../../docs/codex/resource-import-gate.md");

    expect(doc).toContain("Current decision: not product-ready");
    expect(doc).toContain("manifest");
    expect(doc).toContain("rollback");
    expect(doc).toContain("remote references");
  });

  it("keeps optional sound packs default-off and bounded", () => {
    const doc = readDoc("../../../../docs/codex/sound-pack-gate.md");

    expect(doc).toContain("default-off");
    expect(doc).toContain("do-not-disturb");
    expect(doc).toContain("volume cap");
    expect(doc).toContain("no sound assets are shipped");
  });

  it("keeps social and plugin skeletons non-user-visible", () => {
    expect(readDoc("../../../../docs/codex/social-skeleton-readiness.md")).toContain(
      "not product-ready",
    );
    expect(readDoc("../../../../docs/codex/plugin-skeleton-readiness.md")).toContain(
      "no plugin execution entry",
    );
  });

  it("recommends delaying Developer Rhythm behind creature feel", () => {
    const doc = readDoc("../../../../docs/codex/developer-rhythm-reprioritization.md");

    expect(doc).toContain("Recommendation: delay and hide by default");
    expect(doc).toContain("Ian is a desktop creature first");
    expect(doc).toContain("do not read code, diffs, terminal output, or key text");
  });
});

function readDoc(path: string): string {
  return readFileSync(new URL(path, import.meta.url), "utf8");
}
