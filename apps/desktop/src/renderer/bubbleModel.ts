const DEFAULT_MAX_BUBBLE_CHARS = 25;

export function normalizeBubbleMessage(text: string): string | null {
  const normalized = text.trim();
  return normalized.length > 0 ? normalized : null;
}

export function formatBubbleText(
  text: string,
  maxChars = DEFAULT_MAX_BUBBLE_CHARS,
): string {
  const normalized = text.trim().replace(/\s+/g, " ");

  if (normalized.length <= maxChars) {
    return normalized;
  }

  return `${normalized.slice(0, Math.max(maxChars - 1, 1))}…`;
}
