import { describe, expect, it } from "vitest";
import { resolveSavedDragPosition } from "./position";

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
});
