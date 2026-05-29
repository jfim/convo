<script lang="ts">
  import type { ToolResult } from "$lib/bindings";
  import { rendererFor } from "$lib/tools/registry";

  let {
    name,
    input,
    result,
  }: { name: string; input: unknown; result: ToolResult | null } = $props();

  let open = $state(false);
  let renderer = $derived(rendererFor(name));
  let summary = $derived(renderer.summary(input));
</script>

<div class="tool" class:error={result?.isError}>
  <button class="summary" onclick={() => (open = !open)} aria-expanded={open}>
    <span class="chev">{open ? "▾" : "▸"}</span>
    <span class="name">{name}</span>{#if summary}<span class="desc">: {summary}</span
      >{/if}
  </button>
  {#if open}
    {@const Body = renderer.component}
    <div class="body">
      <Body {input} {result} />
    </div>
  {/if}
</div>

<style>
  .tool {
    margin: 0.35rem 0;
    border-left: 3px solid #268bd2;
    padding-left: 0.5rem;
  }
  .tool.error {
    border-left-color: #dc322f;
  }
  .summary {
    display: block;
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    color: #93a1a1;
    cursor: pointer;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.85rem;
    padding: 0.1rem 0;
  }
  .chev {
    color: #586e75;
    margin-right: 0.3rem;
  }
  .name {
    color: #268bd2;
    font-weight: 600;
  }
  .desc {
    color: #839496;
  }
  .body {
    padding: 0.1rem 0 0.2rem;
  }
</style>
