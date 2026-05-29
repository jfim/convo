<script lang="ts">
  import type { ToolProps } from "./registry";
  import { asRecord } from "./registry";
  import { resultText } from "./util";
  import CodeBlock from "./CodeBlock.svelte";

  let { input, result, summary }: ToolProps = $props();

  let o = $derived(asRecord(input));
  let filePath = $derived(o.file_path as string | undefined);
  let oldString = $derived((o.old_string as string) ?? "");
  let newString = $derived((o.new_string as string) ?? "");
  let replaceAll = $derived(o.replace_all === true);
  let output = $derived(resultText(result));
</script>

{#if filePath && filePath !== summary}
  <CodeBlock label="file" text={filePath} />
{/if}
<CodeBlock
  label={replaceAll ? "Replace all instances of" : "Replace one instance of"}
  text={oldString}
/>
<CodeBlock label="with replacement" text={newString} />
{#if output !== null}
  <CodeBlock label="result" text={output} error={result?.isError ?? false} />
{/if}
