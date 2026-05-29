<script lang="ts">
  let { text }: { text: string } = $props();
</script>

<div class="notice">
  <span class="badge">notice</span>
  <pre class="body">{text}</pre>
</div>

<style>
  .notice {
    margin: 0.35rem 0;
    padding: 0.4rem 0.6rem;
    border-left: 3px solid #586e75;
    background: #00212b;
    border-radius: 0 4px 4px 0;
    color: #93a1a1;
    font-size: 0.82rem;
  }
  .badge {
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #586e75;
  }
  .body {
    margin: 0.2rem 0 0;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  }
</style>
