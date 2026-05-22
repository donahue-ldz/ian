import { describe, expect, it } from "vitest";
import {
  clampBubbleAnchorToBounds,
  clampPositionToBounds,
  selectContainingMonitor,
  resolveSavedDragPosition,
} from "./position";

describe("position helpers", () => {
  it("persists the native desktop window position after a Tauri drag", () => {
    expect(
      resolveSavedDragPosition(
        { x: 18, y: 24 },
        { x: 1420, y: 785 },
      ),
    ).toEqual({ x: 1420, y: 785 });
  });

  it("falls back to pointer position for browser preview drags", () => {
    expect(resolveSavedDragPosition({ x: 18, y: 24 }, null)).toEqual({
      x: 18,
      y: 24,
    });
  });

  it("clamps desktop pet position so the visible surface stays inside safe bounds", () => {
    expect(
      clampPositionToBounds(
        { x: 1950, y: 1100 },
        { x: 0, y: 0, width: 1920, height: 1080 },
        { width: 220, height: 260 },
        12,
      ),
    ).toEqual({ x: 1688, y: 808 });
  });

  it("selects the current monitor and falls back without cross-screen roaming", () => {
    const monitors = [
      { x: 0, y: 0, width: 1440, height: 900 },
      { x: 1440, y: 0, width: 1920, height: 1080 },
    ];

    expect(selectContainingMonitor({ x: 1600, y: 400 }, monitors)).toEqual(
      monitors[1],
    );
    expect(selectContainingMonitor({ x: -100, y: 400 }, monitors)).toEqual(
      monitors[0],
    );
  });

  it("keeps bubble anchors visible near screen edges", () => {
    expect(
      clampBubbleAnchorToBounds(
        { x: -80, y: 1040 },
        { x: 0, y: 0, width: 1920, height: 1080 },
        { width: 176, height: 72 },
        12,
      ),
    ).toEqual({ x: 12, y: 996 });
  });
});
