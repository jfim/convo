<script lang="ts">
  import type { ToolProps } from "./registry";
  import { asRecord } from "./registry";
  import { resultText } from "./util";
  import CodeBlock from "./CodeBlock.svelte";

  let { input, result, summary }: ToolProps = $props();

  // The summary already shows file_path, so the invocation is redundant —
  // only surface the path if it somehow differs from what's on the line.
  let filePath = $derived(asRecord(input).file_path as string | undefined);
  let output = $derived(resultText(result));
</script>

{#if filePath && filePath !== summary}
  <CodeBlock label="file" text={filePath} />
{/if}
{#if output !== null}
  <CodeBlock label="result" text={output} error={result?.isError ?? false} />
{/if}
