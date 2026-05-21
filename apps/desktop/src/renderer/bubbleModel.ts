export function normalizeBubbleMessage(text: string): string | null {
  const normalized = text.trim();
  return normalized.length > 0 ? normalized : null;
}
