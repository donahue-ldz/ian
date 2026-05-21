import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const checklistPath =
  "../../docs/sdd/checklists/playful-expressiveness-acceptance.md";
const smokeRecordPath =
  "../../docs/sdd/specs/0070-playful-expressiveness-acceptance-suite/screenshots/0070-browser-smoke-playful.json";

describe("playful expressiveness acceptance packet", () => {
  it("documents objective pass and fail conditions for the playful experience", () => {
    const checklist = readFileSync(checklistPath, "utf8");

    for (const section of [
      "Zoomies",
      "Cute Reactions",
      "Controlled Randomness",
      "Safety Gates",
      "Visual Effects",
      "Evidence",
    ]) {
      expect(checklist).toContain(`## ${section}`);
    }

    expect(checklist.match(/Pass condition:/g)).toHaveLength(6);
    expect(checklist.match(/Fail condition:/g)).toHaveLength(6);
  });

  it("records browser smoke evidence in a machine-readable artifact", () => {
    const smoke = JSON.parse(readFileSync(smokeRecordPath, "utf8")) as {
      settingsSmoke: { hasHighEnergy: boolean; optionLabels: string[] };
      clickSmoke: { hasEffect: boolean; effect: string };
      zoomiesSmoke: { actionTypes: string[]; diagnosticResult: string };
      safetySmoke: { offBlocksZoomies: boolean; quietBlocksZoomies: boolean };
    };

    expect(smoke.settingsSmoke).toMatchObject({
      hasHighEnergy: true,
      optionLabels: ["关闭", "低", "正常", "高"],
    });
    expect(smoke.clickSmoke).toMatchObject({
      hasEffect: true,
      effect: "heart_pop",
    });
    expect(smoke.zoomiesSmoke.actionTypes).toContain("behavior.zoomies");
    expect(smoke.zoomiesSmoke.diagnosticResult).toBe("triggered");
    expect(smoke.safetySmoke).toMatchObject({
      offBlocksZoomies: true,
      quietBlocksZoomies: true,
    });
  });
});
