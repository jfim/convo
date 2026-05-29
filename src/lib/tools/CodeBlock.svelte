<script lang="ts">
  let {
    label = null,
    text,
    error = false,
  }: { label?: string | null; text: string; error?: boolean } = $props();
</script>

{#if label}<div class="label">{label}</div>{/if}
<pre class="code" class:error>{text}</pre>

<style>
  .label {
    font-size: 0.7rem;
    text-transform: uppercase;
    color: #586e75;
    margin: 0.4rem 0 0.15rem;
  }
  .code {
    background: #00212b;
    border: 1px solid #073642;
    border-radius: 6px;
    padding: 0.5rem 0.7rem;
    margin: 0;
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.82rem;
  }
  .code.error {
    border-color: #dc322f;
  }
</style>
