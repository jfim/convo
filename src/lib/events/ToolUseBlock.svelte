<script lang="ts">
  let { name, input }: { name: string; input: unknown } = $props();
</script>

<div class="tool-use">
  <span class="badge">tool: {name}</span>
  <pre>{JSON.stringify(input, null, 2)}</pre>
</div>

<style>
  .tool-use {
    border-left: 3px solid #268bd2;
    padding: 0.4rem 0.6rem;
    margin: 0.25rem 0;
    background: #073642;
    border-radius: 0 4px 4px 0;
  }
  .badge {
    display: inline-block;
    font-size: 0.7rem;
    background: #268bd2;
    color: #fdf6e3;
    border-radius: 3px;
    padding: 0.1em 0.4em;
    margin-bottom: 0.25rem;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0.25rem 0 0;
    color: #93a1a1;
    font-size: 0.85rem;
  }
</style>
