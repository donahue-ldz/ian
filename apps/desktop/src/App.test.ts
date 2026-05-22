import { describe, expect, it, vi } from "vitest";
import type { IanEvent, Position } from "./protocol/generated";
import {
  findIan,
  sendArmedChaseCandidate,
  sendIanClickAndArmChase,
  sendIanLeaveWithArmedChase,
  switchPetResourcePack,
} from "./App";
import type { PetResourcePack } from "./resources/resourceLoader";

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

  it("does not read the desktop cursor when settings are open", async () => {
    const events: IanEvent[] = [];
    const getCursorPosition = vi.fn(async () => ({ x: 620, y: 360 }));

    const isArmed = await sendArmedChaseCandidate({
      isDesktopWindow: true,
      isChaseArmed: true,
      isSettingsOpen: true,
      now: () => 456_789,
      getCursorPosition,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(getCursorPosition).not.toHaveBeenCalled();
    expect(events).toEqual([]);
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

describe("find Ian", () => {
  it("sends only a low-sensitive find_ian shortcut event", async () => {
    const events: IanEvent[] = [];

    await findIan({
      now: () => 123_456,
      sendEvent: async (event) => {
        events.push(event);
      },
    });

    expect(events).toEqual([
      {
        type: "system.shortcut_triggered",
        action: "find_ian",
        now_ms: 123_456,
      },
    ]);
  });
});

describe("pet resource pack switching", () => {
  it("loads the requested pack before persisting the active pet", async () => {
    const pack = testPack("ian-kitten");
    const loadPetResourcePack = vi.fn(async () => pack);
    const saveActivePet = vi.fn(async () => ({
      active_pet_id: "ian-kitten",
      active_resource_pack: "ian-kitten",
    }));
    const setResourcePack = vi.fn();
    const setActivePetId = vi.fn();

    await switchPetResourcePack({
      nextPetId: "ian-kitten",
      loadPetResourcePack,
      saveActivePet,
      setResourcePack,
      setActivePetId,
    });

    expect(loadPetResourcePack).toHaveBeenCalledWith("ian-kitten");
    expect(saveActivePet).toHaveBeenCalledWith("ian-kitten");
    expect(setResourcePack).toHaveBeenCalledWith(pack);
    expect(setActivePetId).toHaveBeenCalledWith("ian-kitten");
  });

  it("does not persist the active pet when resource loading fails", async () => {
    const loadPetResourcePack = vi.fn(async () => {
      throw new Error("missing resource pack");
    });
    const saveActivePet = vi.fn();

    await expect(
      switchPetResourcePack({
        nextPetId: "ian-missing",
        loadPetResourcePack,
        saveActivePet,
        setResourcePack: vi.fn(),
        setActivePetId: vi.fn(),
      }),
    ).rejects.toThrow("missing resource pack");

    expect(saveActivePet).not.toHaveBeenCalled();
  });
});

function testPack(id: string): PetResourcePack {
  return {
    pet: {
      id,
      name: id,
      version: "0.1.0",
      species: "test",
      defaultPersonality: "calm",
      sprite: "sprite.svg",
      animations: "animations.json",
      expressions: "expressions.json",
      sounds: "sounds",
      capabilities: [],
    },
    animations: {
      meta: {
        frameWidth: 220,
        frameHeight: 220,
        scale: 1,
      },
      animations: {
        idle: testAnimation(),
        rest: testAnimation(),
        walk: testAnimation(),
        happy: testAnimation(),
        run: testAnimation(),
        zoomies: testAnimation(),
        sleep: testAnimation(),
      },
    },
    expressions: {
      expressions: {},
    },
  };
}

function testAnimation() {
  return {
    fps: 1,
    frames: [0],
    loop: true,
  };
}
