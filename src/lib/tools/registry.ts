import type { Component } from "svelte";
import type { ToolResult } from "$lib/bindings";
import BashTool from "./BashTool.svelte";
import DefaultTool from "./DefaultTool.svelte";

/** Props every tool renderer component receives. */
export interface ToolProps {
  input: unknown;
  result: ToolResult | null;
}

/** A per-tool renderer: a collapsed-line summary plus an expanded component. */
export interface ToolRenderer {
  /** Short text shown after the tool name on the collapsed line. */
  summary(input: unknown): string;
  /** Component rendered when the tool call is expanded. */
  component: Component<ToolProps>;
}

function asRecord(input: unknown): Record<string, unknown> {
  return input && typeof input === "object" && !Array.isArray(input)
    ? (input as Record<string, unknown>)
    : {};
}

const bashRenderer: ToolRenderer = {
  summary(input) {
    const o = asRecord(input);
    return (o.description as string) ?? (o.command as string) ?? "";
  },
  component: BashTool,
};

/** Fallback for any tool without a dedicated renderer. */
const defaultRenderer: ToolRenderer = {
  summary(input) {
    const o = asRecord(input);
    // Best-effort: first short string value (e.g. file_path, pattern).
    for (const v of Object.values(o)) {
      if (typeof v === "string" && v.length <= 120) return v;
    }
    return "";
  },
  component: DefaultTool,
};

const registry: Record<string, ToolRenderer> = {
  Bash: bashRenderer,
  // Add new tool renderers here, keyed by the tool name.
};

export function rendererFor(name: string): ToolRenderer {
  return registry[name] ?? defaultRenderer;
}
