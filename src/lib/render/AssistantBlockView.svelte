<script lang="ts">
  import type { AssistantBlock } from "$lib/bindings";
  import Markdown from "$lib/Markdown.svelte";
  import ThinkingBlock from "$lib/events/ThinkingBlock.svelte";
  import UnknownNode from "$lib/events/UnknownNode.svelte";
  import ToolCall from "./ToolCall.svelte";

  let { block }: { block: AssistantBlock } = $props();
</script>

{#if block.kind === "markdown"}
  <Markdown source={block.text} />
{:else if block.kind === "thinking"}
  <ThinkingBlock thinking={block.text} />
{:else if block.kind === "toolCall"}
  <ToolCall name={block.name} input={block.input} result={block.result} />
{:else}
  <UnknownNode label="Unknown block" raw={block.raw} />
{/if}
