<script lang="ts">
  import type { ToolProps } from "./registry";
  import { resultText } from "./util";

  let { input, result }: ToolProps = $props();

  let command = $derived(
    (input as Record<string, unknown> | null)?.command as string | undefined,
  );
  let output = $derived(resultText(result));
</script>

<div class="bash">
  <div class="label">command</div>
  <pre class="code">{command ?? ""}</pre>
  {#if output !== null}
    <div class="label">output</div>
    <pre class="code" class:error={result?.isError}>{output}</pre>
  {/if}
</div>

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
