import { describe, expect, it, vi } from "vitest";
import type { IanEvent, Position } from "./protocol/generated";
import {
  sendArmedChaseCandidate,
  sendIanClickAndArmChase,
  sendIanLeaveWithArmedChase,
} from "./App";

describe("click armed pointer chase", () => {
  it("keeps the normal click event and arms chase for desktop pointer leave", async () => {
    const events: IanEvent[] = [];

    const isArmed = await sendIanClickAndArmChase({
      point: { x: 12, y: 18 },
      isDesktopWindow: true,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(isArmed).toBe(true);
    expect(events).toEqual([{ type: "mouse.click", x: 12, y: 18 }]);
  });

  it("sends a chase candidate when the pointer leaves after a desktop click", async () => {
    const events: IanEvent[] = [];
    const cursor: Position = { x: 620, y: 360 };
    const getCursorPosition = vi.fn(async () => ({ x: 620, y: 360 }));

    const isArmed = await sendIanLeaveWithArmedChase({
      point: { x: 12, y: 18 },
      isDesktopWindow: true,
      isChaseArmed: true,
      now: () => 123_456,
      getCursorPosition,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(getCursorPosition).toHaveBeenCalledOnce();
    expect(events).toEqual([
      { type: "mouse.leave", x: 12, y: 18 },
      { type: "mouse.chase_candidate", ...cursor, now_ms: 123_456 },
    ]);
    expect(isArmed).toBe(false);
  });

  it("can send the armed chase candidate from a delayed desktop fallback", async () => {
    const events: IanEvent[] = [];
    const cursor: Position = { x: 720, y: 420 };
    const getCursorPosition = vi.fn(async () => cursor);

    const isArmed = await sendArmedChaseCandidate({
      isDesktopWindow: true,
      isChaseArmed: true,
      now: () => 456_789,
      getCursorPosition,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(getCursorPosition).toHaveBeenCalledOnce();
    expect(events).toEqual([
      { type: "mouse.chase_candidate", ...cursor, now_ms: 456_789 },
    ]);
    expect(isArmed).toBe(false);
  });

  it("does not read the desktop cursor on leave when chase is not armed", async () => {
    const events: IanEvent[] = [];
    const getCursorPosition = vi.fn(async () => ({ x: 620, y: 360 }));

    const isArmed = await sendIanLeaveWithArmedChase({
      point: { x: 12, y: 18 },
      isDesktopWindow: true,
      isChaseArmed: false,
      now: () => 123_456,
      getCursorPosition,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(getCursorPosition).not.toHaveBeenCalled();
    expect(events).toEqual([{ type: "mouse.leave", x: 12, y: 18 }]);
    expect(isArmed).toBe(false);
  });

  it("does not read the desktop cursor in browser preview", async () => {
    const events: IanEvent[] = [];
    const getCursorPosition = vi.fn(async () => ({ x: 620, y: 360 }));

    const isArmed = await sendIanLeaveWithArmedChase({
      point: { x: 12, y: 18 },
      isDesktopWindow: false,
      isChaseArmed: true,
      now: () => 123_456,
      getCursorPosition,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(getCursorPosition).not.toHaveBeenCalled();
    expect(events).toEqual([{ type: "mouse.leave", x: 12, y: 18 }]);
    expect(isArmed).toBe(false);
  });
});
