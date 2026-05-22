import { PhysicalPosition } from "@tauri-apps/api/dpi";
import {
  currentMonitor,
  cursorPosition,
  getCurrentWindow,
} from "@tauri-apps/api/window";
import type { Position, ScreenBounds } from "../protocol/generated";

export function isRestorablePosition(position: Position): boolean {
  return (
    Number.isFinite(position.x) &&
    Number.isFinite(position.y) &&
    (position.x !== 0 || position.y !== 0)
  );
}

export async function moveDesktopWindow(position: Position): Promise<void> {
  if (!("__TAURI_INTERNALS__" in window) || !isRestorablePosition(position)) {
    return;
  }

  await getCurrentWindow().setPosition(
    new PhysicalPosition(Math.round(position.x), Math.round(position.y)),
  );
}

export async function startDesktopWindowDrag(): Promise<void> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return;
  }

  await getCurrentWindow().startDragging();
}

export async function getDesktopWindowPosition(): Promise<Position | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  const position = await getCurrentWindow().outerPosition();
  return { x: position.x, y: position.y };
}

export async function getDesktopCursorPosition(): Promise<Position | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  const position = await cursorPosition();
  return { x: position.x, y: position.y };
}

export async function getDesktopScreenBounds(): Promise<ScreenBounds | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  const monitor = await currentMonitor();
  if (!monitor) {
    return null;
  }

  return {
    x: monitor.position.x,
    y: monitor.position.y,
    width: monitor.size.width,
    height: monitor.size.height,
  };
}

export function resolveSavedDragPosition(
  pointerPosition: Position,
  desktopWindowPosition: Position | null,
): Position {
  return desktopWindowPosition ?? pointerPosition;
}

type SurfaceSize = {
  width: number;
  height: number;
};

export function clampPositionToBounds(
  position: Position,
  bounds: ScreenBounds,
  size: SurfaceSize = { width: 220, height: 260 },
  margin = 12,
): Position {
  return {
    x: clamp(position.x, bounds.x + margin, bounds.x + bounds.width - size.width - margin),
    y: clamp(position.y, bounds.y + margin, bounds.y + bounds.height - size.height - margin),
  };
}

export function selectContainingMonitor(
  position: Position,
  monitors: ScreenBounds[],
): ScreenBounds | null {
  if (monitors.length === 0) {
    return null;
  }

  return (
    monitors.find(
      (monitor) =>
        position.x >= monitor.x &&
        position.x <= monitor.x + monitor.width &&
        position.y >= monitor.y &&
        position.y <= monitor.y + monitor.height,
    ) ?? monitors[0]
  );
}

export function clampBubbleAnchorToBounds(
  anchor: Position,
  bounds: ScreenBounds,
  size: SurfaceSize = { width: 176, height: 72 },
  margin = 12,
): Position {
  return clampPositionToBounds(anchor, bounds, size, margin);
}

function clamp(value: number, min: number, max: number): number {
  if (max < min) {
    return min;
  }
  return Math.min(Math.max(value, min), max);
}
