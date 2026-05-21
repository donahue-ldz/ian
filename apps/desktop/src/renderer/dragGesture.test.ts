import { describe, expect, it } from "vitest";
import { getDragOffset, shouldStartDrag } from "./dragGesture";

describe("dragGesture", () => {
  it("does not start dragging for a stationary Ian click", () => {
    expect(shouldStartDrag({ x: 20, y: 20 }, { x: 20, y: 20 })).toBe(false);
    expect(shouldStartDrag({ x: 20, y: 20 }, { x: 23, y: 24 })).toBe(false);
  });

  it("starts dragging only after pointer movement passes the threshold", () => {
    expect(shouldStartDrag({ x: 20, y: 20 }, { x: 29, y: 20 })).toBe(true);
    expect(getDragOffset({ x: 20, y: 20 }, { x: 29, y: 20 })).toEqual({
      x: 9,
      y: 0,
    });
  });
});
