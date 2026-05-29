<script lang="ts">
  import type { RenderItem } from "$lib/bindings";
  import UserPrompt from "./UserPrompt.svelte";
  import AssistantTurn from "./AssistantTurn.svelte";
  import SystemEvent from "$lib/events/SystemEvent.svelte";
  import AttachmentEvent from "$lib/events/AttachmentEvent.svelte";
  import PrLinkEvent from "$lib/events/PrLinkEvent.svelte";
  import UnknownNode from "$lib/events/UnknownNode.svelte";

  let { item }: { item: RenderItem } = $props();
</script>

{#if item.kind === "userPrompt"}
  <UserPrompt text={item.text} />
{:else if item.kind === "assistantTurn"}
  <AssistantTurn blocks={item.blocks} />
{:else if item.kind === "system"}
  <SystemEvent subtype={item.subtype} content={item.content} />
{:else if item.kind === "attachment"}
  <AttachmentEvent attachment={item.content} />
{:else if item.kind === "prLink"}
  <PrLinkEvent url={item.url} number={item.number} />
{:else if item.kind === "parseError"}
  <UnknownNode label={`Parse error (line ${item.lineNumber})`} raw={item.raw} />
{:else}
  <UnknownNode label={item.label} raw={item.raw} />
{/if}
