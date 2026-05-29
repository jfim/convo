<script lang="ts">
  import { marked } from "marked";
  import DOMPurify from "dompurify";

  let { source }: { source: string } = $props();

  // marked.parse is synchronous when async is disabled; sanitize before {@html}.
  let html = $derived(
    DOMPurify.sanitize(marked.parse(source ?? "", { async: false }) as string),
  );
</script>

<div class="md">{@html html}</div>

<style>
  .md :global(p) {
    margin: 0.4rem 0;
    white-space: normal;
    word-break: break-word;
  }
  .md :global(:first-child) {
    margin-top: 0;
  }
  .md :global(:last-child) {
    margin-bottom: 0;
  }
  .md :global(pre) {
    background: #00212b;
    border: 1px solid #073642;
    border-radius: 6px;
    padding: 0.6rem 0.8rem;
    overflow-x: auto;
  }
  .md :global(code) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.85em;
  }
  .md :global(:not(pre) > code) {
    background: #00212b;
    padding: 0.1rem 0.3rem;
    border-radius: 4px;
  }
  .md :global(a) {
    color: #268bd2;
  }
  .md :global(h1),
  .md :global(h2),
  .md :global(h3) {
    margin: 0.8rem 0 0.4rem;
    line-height: 1.25;
  }
  .md :global(ul),
  .md :global(ol) {
    margin: 0.4rem 0;
    padding-left: 1.4rem;
  }
  .md :global(blockquote) {
    margin: 0.4rem 0;
    padding-left: 0.8rem;
    border-left: 3px solid #073642;
    color: #93a1a1;
  }
  .md :global(table) {
    border-collapse: collapse;
  }
  .md :global(th),
  .md :global(td) {
    border: 1px solid #073642;
    padding: 0.3rem 0.5rem;
  }
</style>
