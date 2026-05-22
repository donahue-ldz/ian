import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const docs = [
  ["settings-option-accessibility-audit.md", "reduced motion"],
  ["resource-pack-semantic-coverage.md", "semantic coverage"],
  ["p0-definition-of-done-reconciliation.md", "P0 Definition of Done"],
  ["v01-baseline-readiness-map-v2.md", "default-off"],
  ["privacy-source-denylist-scan-v2.md", "clipboard"],
  ["byom-default-off-readiness-v2.md", "BYOM remains default-off"],
  ["reminder-disturbance-boundary-v2.md", "do-not-disturb"],
  ["memory-export-low-sensitive-audit.md", "low-sensitive tags"],
  ["social-plugin-nonproduct-gate-audit.md", "not product-ready"],
  ["adapter-permission-registry-audit.md", "unknown sources are denied"],
  ["storage-migration-readiness-v2.md", "schema_migrations"],
  ["tauri-command-privacy-inventory.md", "handle_ian_event"],
  ["post-0231-release-readiness-summary.md", "0232-0251"],
] as const;

describe("post-0231 readiness docs", () => {
  it("records every post-0231 readiness document with a default-off boundary", () => {
    for (const [file, requiredText] of docs) {
      const doc = readCodexDoc(file);

      expect(doc).toContain(requiredText);
      expect(doc).toMatch(/default-off|默认关闭|not product-ready|local-first/);
    }
  });

  it("extends the desktop smoke checklist with reduced motion and future gates", () => {
    const checklist = readFileSync(
      new URL("../../../../docs/sdd/desktop-smoke-checklist.md", import.meta.url),
      "utf8",
    );

    expect(checklist).toContain("Post-0231");
    expect(checklist).toContain("减少动画");
    expect(checklist).toContain("未来能力默认关闭");
  });

  it("refreshes the SDD ledger beyond 0231", () => {
    const ledger = readFileSync(
      new URL("../../../../docs/sdd/ledger.md", import.meta.url),
      "utf8",
    );

    expect(ledger).toContain("0232-0251");
    expect(ledger).toContain("post-0231 readiness");
    expect(ledger).toContain("0252-0261");
    expect(ledger).toContain("0262-0269");
    expect(ledger).toContain("当前没有待执行 SDD");
    expect(ledger).not.toContain("| 0202 | memory-debug-and-export-minimal | 待执行 |");
    expect(ledger).not.toContain("| 0232-0251 | post-0231 readiness / hardening | 执行中 |");
  });
});

function readCodexDoc(file: string): string {
  return readFileSync(
    new URL(`../../../../docs/codex/${file}`, import.meta.url),
    "utf8",
  );
}
