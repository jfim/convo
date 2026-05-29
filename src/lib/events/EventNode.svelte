<script lang="ts">
  import type { ConversationEvent } from "$lib/bindings";
  import UnknownNode from "./UnknownNode.svelte";
  import UserMessage from "./UserMessage.svelte";
  import AssistantMessage from "./AssistantMessage.svelte";
  import AttachmentEvent from "./AttachmentEvent.svelte";
  import SystemEvent from "./SystemEvent.svelte";
  import PrLinkEvent from "./PrLinkEvent.svelte";

  let { event }: { event: ConversationEvent } = $props();
</script>

{#if event.parseError}
  <UnknownNode label={`Parse error (line ${event.parseError.line_number})`} raw={event.parseError.raw} />
{:else if event.type === "user" && event.message}
  <UserMessage message={event.message} />
{:else if event.type === "assistant" && event.message}
  <AssistantMessage message={event.message} />
{:else if event.type === "attachment"}
  <AttachmentEvent attachment={event.attachment} />
{:else if event.type === "system"}
  <SystemEvent subtype={event.subtype} content={event.content} />
{:else if event.type === "pr-link"}
  <PrLinkEvent url={event.prUrl} number={event.prNumber} />
{:else}
  <UnknownNode label={`Event: ${event.type ?? "(no type)"}`} raw={event} />
{/if}
