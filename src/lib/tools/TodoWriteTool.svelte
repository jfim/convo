<script lang="ts">
  import type { ToolProps } from "./registry";
  import { asRecord } from "./registry";

  let { input }: ToolProps = $props();

  type Todo = { content?: string; status?: string; activeForm?: string };
  let todos = $derived((asRecord(input).todos as Todo[] | undefined) ?? []);

  const icon = (status?: string) =>
    status === "completed" ? "✓" : status === "in_progress" ? "●" : "○";
  // Show the present-continuous activeForm for the in-progress item, the plain
  // content otherwise.
  const label = (t: Todo) =>
    (t.status === "in_progress" ? t.activeForm : t.content) ??
    t.content ??
    "";
</script>

<ul class="todos">
  {#each todos as t, i (i)}
    <li class={t.status ?? "pending"}>
      <span class="icon">{icon(t.status)}</span>
      <span class="text">{label(t)}</span>
    </li>
  {/each}
</ul>

<style>
  .todos {
    list-style: none;
    margin: 0.3rem 0;
    padding: 0;
    font-size: 0.85rem;
  }
  .todos li {
    display: flex;
    gap: 0.5rem;
    padding: 0.1rem 0;
    align-items: baseline;
  }
  .icon {
    width: 1em;
    flex: none;
    text-align: center;
  }
  li.completed {
    color: #586e75;
  }
  li.completed .text {
    text-decoration: line-through;
  }
  li.completed .icon {
    color: #859900;
  }
  li.in_progress {
    color: #eee8d5;
  }
  li.in_progress .icon {
    color: #b58900;
  }
  li.pending {
    color: #839496;
  }
  li.pending .icon {
    color: #586e75;
  }
</style>
