<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrent, onOpenUrl } from "@tauri-apps/plugin-deep-link";
  import { listen } from "@tauri-apps/api/event";
  import { commands, type ConversationEvent, type ConvoError } from "$lib/bindings";
  import Conversation from "$lib/Conversation.svelte";
  import ErrorScreen from "$lib/ErrorScreen.svelte";

  let events = $state<ConversationEvent[] | null>(null);
  let anchor = $state<string | null>(null);
  let errorMessage = $state<string | null>(null);

  async function load(url: string) {
    errorMessage = null;
    const result = await commands.loadConversation(url);
    if (result.status === "ok") {
      events = result.data.events;
      anchor = result.data.anchor;
    } else {
      errorMessage = formatError(result.error);
      events = null;
    }
  }

  function formatError(err: ConvoError): string {
    return "message" in err ? `${err.kind}: ${err.message}` : err.kind;
  }

  onMount(async () => {
    const start = await getCurrent();
    if (start && start.length > 0) {
      await load(start[0]);
    } else {
      errorMessage = "No conversation URL provided.";
    }
    await onOpenUrl((urls) => {
      if (urls.length > 0) void load(urls[0]);
    });
    await listen<string>("deep-link", (e) => void load(e.payload));
  });
</script>

{#if events}
  <Conversation {events} {anchor} />
{:else if errorMessage}
  <ErrorScreen message={errorMessage} />
{:else}
  <p class="loading">Loading…</p>
{/if}

<style>
  :global(body) { margin: 0; font-family: ui-sans-serif, system-ui, sans-serif; background: #002b36; color: #eee8d5; }
  .loading { padding: 2rem; opacity: 0.6; }
</style>
