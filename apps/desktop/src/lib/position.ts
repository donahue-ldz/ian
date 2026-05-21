import { PhysicalPosition } from "@tauri-apps/api/dpi";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Position } from "../protocol/generated";

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
