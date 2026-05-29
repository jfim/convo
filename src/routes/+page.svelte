<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrent, onOpenUrl } from "@tauri-apps/plugin-deep-link";
  import { listen } from "@tauri-apps/api/event";
  import {
    commands,
    type RenderItem,
    type ConversationStats,
    type ConvoError,
  } from "$lib/bindings";
  import Conversation from "$lib/Conversation.svelte";
  import ErrorScreen from "$lib/ErrorScreen.svelte";

  let items = $state<RenderItem[] | null>(null);
  let stats = $state<ConversationStats | null>(null);
  let anchor = $state<string | null>(null);
  let errorMessage = $state<string | null>(null);
  let showHidden = $state(false);

  let hiddenCount = $derived(items?.filter((i) => i.hidden).length ?? 0);

  async function load(url: string) {
    errorMessage = null;
    const result = await commands.loadConversation(url);
    if (result.status === "ok") {
      items = result.data.items;
      stats = result.data.stats;
      anchor = result.data.anchor;
    } else {
      errorMessage = formatError(result.error);
      items = null;
      stats = null;
    }
  }

  function formatError(err: ConvoError): string {
    return "message" in err ? `${err.kind}: ${err.message}` : err.kind;
  }

  onMount(async () => {
    // Cold-start URL. Prefer our own argv scan (robust against the extra CLI
    // flags cargo/tauri dev append, which defeat the plugin's getCurrent), and
    // fall back to the plugin for platforms that don't deliver it via argv.
    const coldStart = (await commands.initialUrl()) ?? (await getCurrent())?.[0] ?? null;
    if (coldStart) {
      await load(coldStart);
    } else {
      errorMessage = "No conversation URL provided.";
    }
    // Warm-start: the OS spawns a second instance; single-instance forwards the
    // URL as a "deep-link" event. onOpenUrl covers macOS.
    await onOpenUrl((urls) => {
      if (urls.length > 0) void load(urls[0]);
    });
    await listen<string>("deep-link", (e) => void load(e.payload));
  });
</script>

{#if items}
  <Conversation {items} {anchor} {showHidden} {stats} />
  <footer class="bottombar">
    <label class="show-hidden" class:disabled={hiddenCount === 0}>
      <input type="checkbox" bind:checked={showHidden} disabled={hiddenCount === 0} />
      Show hidden{hiddenCount > 0 ? ` (${hiddenCount})` : ""}
    </label>
  </footer>
{:else if errorMessage}
  <ErrorScreen message={errorMessage} />
{:else}
  <p class="loading">Loading…</p>
{/if}

<style>
  :global(body) { margin: 0; font-family: ui-sans-serif, system-ui, sans-serif; background: #002b36; color: #eee8d5; }
  .loading { padding: 2rem; opacity: 0.6; }
  .bottombar {
    position: sticky;
    bottom: 0;
    z-index: 10;
    display: flex;
    justify-content: flex-end;
    padding: 0.4rem 1rem;
    background: #002b36;
    border-top: 1px solid #073642;
  }
  .show-hidden {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.8rem;
    color: #93a1a1;
    cursor: pointer;
    user-select: none;
  }
  .show-hidden.disabled {
    opacity: 0.4;
    cursor: default;
  }
</style>
