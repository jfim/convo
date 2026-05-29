<script lang="ts">
  let { text, summary }: { text: string; summary: string | null } = $props();

  let open = $state(false);
  let label = $derived(summary ?? "notice");
</script>

<div class="notice">
  <button class="summary" onclick={() => (open = !open)} aria-expanded={open}>
    <span class="cog">⚙</span>
    <span class="label">{label}</span>
  </button>
  {#if open}
    <pre class="body">{text}</pre>
  {/if}
</div>

<style>
  .notice {
    margin: 0.3rem 0;
    color: #93a1a1;
  }
  .summary {
    display: block;
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    color: #93a1a1;
    cursor: pointer;
    font-size: 0.82rem;
    padding: 0.1rem 0;
  }
  .cog {
    color: #586e75;
    margin-right: 0.35rem;
  }
  .body {
    margin: 0.25rem 0 0;
    padding: 0.4rem 0.6rem;
    border-left: 3px solid #586e75;
    background: #00212b;
    border-radius: 0 4px 4px 0;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.8rem;
  }
</style>
