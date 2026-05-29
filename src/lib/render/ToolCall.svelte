<script lang="ts">
  import type { ToolResult } from "$lib/bindings";
  import { rendererFor } from "$lib/tools/registry";

  let {
    name,
    input,
    result,
  }: { name: string; input: unknown; result: ToolResult | null } = $props();

  let renderer = $derived(rendererFor(name));
  let summary = $derived(renderer.summary(input));
  let Body = $derived(renderer.component);
</script>

<!-- Native <details> so expand/collapse works with no JS — including in the
     static HTML export, which is a DOM snapshot with no scripts. -->
<details class="tool" class:error={result?.isError}>
  <summary>
    <span class="chev"></span>
    <span class="name">{name}</span>{#if summary}<span class="desc">: {summary}</span
      >{/if}
  </summary>
  <div class="body">
    <Body {input} {result} {summary} />
  </div>
</details>

<style>
  .tool {
    margin: 0.35rem 0;
    border-left: 3px solid #268bd2;
    padding-left: 0.5rem;
  }
  .tool.error {
    border-left-color: #dc322f;
  }
  summary {
    display: block;
    width: 100%;
    text-align: left;
    color: #93a1a1;
    cursor: pointer;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.85rem;
    padding: 0.1rem 0;
    list-style: none;
  }
  /* Hide the default disclosure marker; we draw our own chevron. */
  summary::-webkit-details-marker {
    display: none;
  }
  .chev::before {
    content: "▸";
    color: #586e75;
    margin-right: 0.3rem;
  }
  details[open] > summary .chev::before {
    content: "▾";
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
