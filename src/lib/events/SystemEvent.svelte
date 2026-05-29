<script lang="ts">
  let { subtype, content }: { subtype: string | null | undefined; content: unknown } = $props();
</script>

<div class="system-event">
  <span class="badge">{subtype ? `system: ${subtype}` : "system"}</span>
  {#if content}
    <pre>{JSON.stringify(content, null, 2)}</pre>
  {/if}
</div>

<style>
  .system-event {
    margin: 0.25rem 0;
    color: #586e75;
    font-size: 0.85rem;
  }
  .badge {
    display: inline-block;
    font-size: 0.7rem;
    background: #073642;
    color: #586e75;
    border-radius: 3px;
    padding: 0.1em 0.4em;
    border: 1px solid #586e75;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0.25rem 0 0;
    color: #586e75;
    font-size: 0.8rem;
  }
</style>
