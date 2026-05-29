<script lang="ts">
  let { label, raw }: { label: string; raw: unknown } = $props();
</script>

<div class="unknown-node">
  <span class="badge">{label}</span>
  <pre>{JSON.stringify(raw, null, 2)}</pre>
</div>

<style>
  .unknown-node {
    border: 1px dashed #586e75;
    border-radius: 4px;
    padding: 0.5rem;
    margin: 0.25rem 0;
    background: #073642;
  }
  .badge {
    display: inline-block;
    font-size: 0.7rem;
    background: #586e75;
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
