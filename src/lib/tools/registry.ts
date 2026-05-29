import type { Component } from "svelte";
import type { ToolResult } from "$lib/bindings";
import BashTool from "./BashTool.svelte";
import DefaultTool from "./DefaultTool.svelte";
import ReadTool from "./ReadTool.svelte";
import WriteTool from "./WriteTool.svelte";
import EditTool from "./EditTool.svelte";
import AgentTool from "./AgentTool.svelte";
import TodoWriteTool from "./TodoWriteTool.svelte";

/** Props every tool renderer component receives. */
export interface ToolProps {
  input: unknown;
  result: ToolResult | null;
  /** The collapsed-line summary, so bodies can avoid repeating it. */
  summary: string;
}

/** A per-tool renderer: a collapsed-line summary plus an expanded component. */
export interface ToolRenderer {
  /** Short text shown after the tool name on the collapsed line. */
  summary(input: unknown): string;
  /** Component rendered when the tool call is expanded. */
  component: Component<ToolProps>;
}

export function asRecord(input: unknown): Record<string, unknown> {
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

/** file_path-summarized tools (Read/Write/Edit). */
const filePathSummary = (input: unknown) =>
  (asRecord(input).file_path as string) ?? "";

const readRenderer: ToolRenderer = {
  summary: filePathSummary,
  component: ReadTool,
};

const writeRenderer: ToolRenderer = {
  summary: filePathSummary,
  component: WriteTool,
};

const editRenderer: ToolRenderer = {
  summary: filePathSummary,
  component: EditTool,
};

const agentRenderer: ToolRenderer = {
  summary(input) {
    const t = asRecord(input).subagent_type as string | undefined;
    return t ? `Start ${t} agent` : "Start agent";
  },
  component: AgentTool,
};

interface Todo {
  content?: string;
  status?: string;
  activeForm?: string;
}

const todoWriteRenderer: ToolRenderer = {
  summary(input) {
    const todos = (asRecord(input).todos as Todo[] | undefined) ?? [];
    const total = todos.length;
    if (total === 0) return "";
    const done = todos.filter((t) => t.status === "completed").length;
    const current = todos.find((t) => t.status === "in_progress");
    const label = current?.activeForm ?? current?.content;
    return label ? `${label} (${done}/${total})` : `${done}/${total} done`;
  },
  component: TodoWriteTool,
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
  Read: readRenderer,
  Write: writeRenderer,
  Edit: editRenderer,
  Agent: agentRenderer,
  TodoWrite: todoWriteRenderer,
  // Add new tool renderers here, keyed by the tool name.
};

export function rendererFor(name: string): ToolRenderer {
  return registry[name] ?? defaultRenderer;
}
