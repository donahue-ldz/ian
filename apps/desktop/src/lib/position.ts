import { PhysicalPosition } from "@tauri-apps/api/dpi";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Position } from "../protocol/generated";

export async function moveDesktopWindow(position: Position): Promise<void> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return;
  }

  await getCurrentWindow().setPosition(
    new PhysicalPosition(Math.round(position.x), Math.round(position.y)),
  );
}
