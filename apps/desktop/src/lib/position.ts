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
