<script lang="ts">
  import type { ToolProps } from "./registry";
  import { asRecord } from "./registry";
  import { pretty } from "./util";
  import CodeBlock from "./CodeBlock.svelte";
  import Markdown from "$lib/Markdown.svelte";

  let { input, result }: ToolProps = $props();

  let prompt = $derived((asRecord(input).prompt as string) ?? "");

  // The agent's response is the tool result: an array of content blocks
  // (usually text). Render each block on its own — text as markdown, anything
  // else as raw JSON. Contiguous text blocks are intentionally not merged.
  type Block = { type?: string; text?: string };
  let responseBlocks = $derived.by((): Block[] => {
    const c = result?.content;
    if (c == null) return [];
    if (typeof c === "string") return [{ type: "text", text: c }];
    if (Array.isArray(c)) return c as Block[];
    return [{ type: "unknown", text: undefined }];
  });
</script>

<CodeBlock label="prompt" text={prompt} />

{#if responseBlocks.length}
  <div class="label">response</div>
  {#each responseBlocks as block, i (i)}
    {#if block?.type === "text" && typeof block.text === "string"}
      <div class="resp-block"><Markdown source={block.text} /></div>
    {:else}
      <CodeBlock text={pretty(block)} error={result?.isError ?? false} />
    {/if}
  {/each}
{/if}

<style>
  .label {
    font-size: 0.7rem;
    text-transform: uppercase;
    color: #586e75;
    margin: 0.4rem 0 0.15rem;
  }
  .resp-block {
    margin: 0.4rem 0;
    color: #93a1a1;
  }
</style>
