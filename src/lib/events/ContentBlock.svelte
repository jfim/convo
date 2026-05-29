<script lang="ts">
  import type { ContentBlock, KnownBlock } from "$lib/bindings";
  import ThinkingBlock from "./ThinkingBlock.svelte";
  import ToolUseBlock from "./ToolUseBlock.svelte";
  import ToolResultBlock from "./ToolResultBlock.svelte";
  import UnknownNode from "./UnknownNode.svelte";

  let { block }: { block: ContentBlock } = $props();

  const knownTypes = new Set(["text", "thinking", "tool_use", "tool_result"]);

  function isKnownBlock(b: ContentBlock): b is KnownBlock {
    return (
      b !== null &&
      typeof b === "object" &&
      !Array.isArray(b) &&
      "type" in b &&
      typeof (b as { type: unknown }).type === "string" &&
      knownTypes.has((b as { type: string }).type)
    );
  }

  let known = $derived(isKnownBlock(block) ? (block as KnownBlock) : null);
</script>

{#if known !== null}
  {#if known.type === "text"}
    <p class="text">{known.text}</p>
  {:else if known.type === "thinking"}
    <ThinkingBlock thinking={known.thinking} />
  {:else if known.type === "tool_use"}
    <ToolUseBlock name={known.name} input={known.input} />
  {:else if known.type === "tool_result"}
    <ToolResultBlock content={known.content} isError={known.is_error} />
  {/if}
{:else}
  <UnknownNode label="Unknown block" raw={block} />
{/if}

<style>
  .text {
    margin: 0.25rem 0;
    color: #eee8d5;
    white-space: pre-wrap;
    word-break: break-word;
  }
</style>
