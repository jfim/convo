import type { ToolResult } from "$lib/bindings";

/** Render arbitrary JSON as pretty text. */
export function pretty(value: unknown): string {
  if (typeof value === "string") return value;
  return JSON.stringify(value, null, 2);
}

/** The textual body of a tool result, or null if there is no result yet. */
export function resultText(result: ToolResult | null): string | null {
  if (!result) return null;
  return pretty(result.content);
}
