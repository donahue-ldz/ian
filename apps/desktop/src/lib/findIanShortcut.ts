import {
  isRegistered,
  register,
  unregister,
  type ShortcutEvent,
} from "@tauri-apps/plugin-global-shortcut";

export type FindIanShortcutStatus = "idle" | "registered" | "conflict" | "unsupported";

export async function registerFindIanShortcut(
  shortcut: string,
  onPressed: () => void,
): Promise<FindIanShortcutStatus> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return "unsupported";
  }

  try {
    if (await isRegistered(shortcut)) {
      return "registered";
    }
    await register(shortcut, (event: ShortcutEvent) => {
      if (event.state === "Pressed") {
        onPressed();
      }
    });
    return "registered";
  } catch {
    return "conflict";
  }
}

export async function unregisterFindIanShortcut(shortcut: string): Promise<void> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return;
  }

  try {
    await unregister(shortcut);
  } catch {
    // Unregister can fail when another app owns the shortcut or this app never registered it.
  }
}

export function formatFindIanShortcut(shortcut: string): string {
  return shortcut.replace("CommandOrControl", "Cmd");
}
