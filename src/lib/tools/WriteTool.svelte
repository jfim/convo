<script lang="ts">
  import type { ToolProps } from "./registry";
  import { asRecord } from "./registry";
  import { resultText } from "./util";
  import CodeBlock from "./CodeBlock.svelte";

  let { input, result, summary }: ToolProps = $props();

  let filePath = $derived(asRecord(input).file_path as string | undefined);
  // The tool field is `content`; accept `contents` too for resilience.
  let contents = $derived(
    (asRecord(input).content as string) ??
      (asRecord(input).contents as string) ??
      "",
  );
  let output = $derived(resultText(result));
</script>

{#if filePath && filePath !== summary}
  <CodeBlock label="file" text={filePath} />
{/if}
<CodeBlock label="contents" text={contents} />
{#if output !== null}
  <CodeBlock label="result" text={output} error={result?.isError ?? false} />
{/if}
