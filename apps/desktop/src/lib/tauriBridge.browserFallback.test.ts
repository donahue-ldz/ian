import { describe, expect, it } from "vitest";
import {
  saveCreatureSettings,
  saveDoNotDisturb,
  sendIanEvent,
} from "./tauriBridge";

describe("browser fallback runtime parity", () => {
  it("suppresses autonomous visual motion when movement intensity is reduced", async () => {
    await saveDoNotDisturb(false);
    await saveCreatureSettings({
      movement_intensity: "reduced",
      bubble_frequency: "normal",
      rest_behavior: "normal",
      playful_energy: "high",
      playful_snoozed_until_ms: null,
      surface_scale: 1,
      diagnostics_enabled: true,
    });

    const actions = await sendIanEvent({ type: "time.tick", now_ms: 180_000 });

    expect(actions.some((action) => action.type === "behavior.zoomies")).toBe(false);
    expect(
      actions.some(
        (action) => action.type === "effect.play" && action.name === "tail_wag",
      ),
    ).toBe(false);
    expect(
      actions.some(
        (action) => action.type === "effect.play" && action.name === "speed_lines",
      ),
    ).toBe(false);
  });

  it("keeps browser preview quiet during do-not-disturb ticks", async () => {
    await saveCreatureSettings({
      movement_intensity: "normal",
      bubble_frequency: "normal",
      rest_behavior: "normal",
      playful_energy: "high",
      playful_snoozed_until_ms: null,
      surface_scale: 1,
      diagnostics_enabled: true,
    });
    await saveDoNotDisturb(true);

    const actions = await sendIanEvent({ type: "time.tick", now_ms: 180_000 });

    expect(actions).toEqual([{ type: "animation.play", name: "idle", looped: true }]);
  });

  it("mirrors Moment debug triggers through browser fallback without sensitive data", async () => {
    await saveDoNotDisturb(false);
    await saveCreatureSettings({
      movement_intensity: "normal",
      bubble_frequency: "normal",
      rest_behavior: "normal",
      playful_energy: "normal",
      playful_snoozed_until_ms: null,
      surface_scale: 1,
      diagnostics_enabled: true,
    });

    const actions = await sendIanEvent({
      type: "moment.debug_trigger",
      kind: "rare_idle_surprise",
      now_ms: 42_000,
    });

    expect(actions).toContainEqual({
      type: "playful.diagnostic",
      timestamp_ms: 42_000,
      reason: "rare_idle_surprise",
      result: "diagnostic_triggered",
      cooldown_key: "rare_idle_surprise",
      chosen_reaction_key: "rare_idle_surprise_sequence",
    });
    expect(JSON.stringify(actions)).not.toContain("/Users/");
  });
});
