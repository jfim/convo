<script lang="ts">
  let { content, isError = false }: { content: unknown; isError?: boolean } = $props();

  let displayText = $derived(
    typeof content === "string" ? content : JSON.stringify(content, null, 2)
  );
</script>

<div class="tool-result" class:error={isError}>
  <span class="badge">{isError ? "tool error" : "tool result"}</span>
  <pre>{displayText}</pre>
</div>

<style>
  .tool-result {
    border-left: 3px solid #2aa198;
    padding: 0.4rem 0.6rem;
    margin: 0.25rem 0;
    background: #073642;
    border-radius: 0 4px 4px 0;
  }
  .tool-result.error {
    border-left-color: #dc322f;
  }
  .badge {
    display: inline-block;
    font-size: 0.7rem;
    background: #2aa198;
    color: #fdf6e3;
    border-radius: 3px;
    padding: 0.1em 0.4em;
    margin-bottom: 0.25rem;
  }
  .error .badge {
    background: #dc322f;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0.25rem 0 0;
    color: #93a1a1;
    font-size: 0.85rem;
  }
</style>
