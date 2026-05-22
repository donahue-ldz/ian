const DEFAULT_MAX_BUBBLE_CHARS = 25;
const DEFAULT_MAX_INPUT_CHARS = 80;

export function normalizeBubbleMessage(text: string): string | null {
  const normalized = text.trim().replace(/\s+/g, " ");
  if (normalized.length === 0) {
    return null;
  }

  return Array.from(normalized).slice(0, DEFAULT_MAX_INPUT_CHARS).join("");
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
