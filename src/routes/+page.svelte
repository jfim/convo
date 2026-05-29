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
  let exporting = $state(false);

  // Build a self-contained HTML document from the rendered transcript: the
  // `.conversation` subtree plus every stylesheet's rules inlined, so the file
  // renders identically with no external assets. Reflects the current
  // "Show hidden" state (only visible items are in the DOM).
  async function exportHtml() {
    const conv = document.querySelector(".conversation");
    if (!conv) return;
    const css = Array.from(document.styleSheets)
      .map((sheet) => {
        try {
          return Array.from(sheet.cssRules)
            .map((r) => r.cssText)
            .join("\n");
        } catch {
          return ""; // cross-origin sheet — skip
        }
      })
      .join("\n");
    const doc = `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>convo export</title>
<style>
${css}
.convo-export-footer { max-width: 56rem; margin: 2rem auto; padding: 1rem; text-align: center; font-size: 0.8rem; color: #586e75; }
.convo-export-footer a { color: #839496; }
</style>
</head>
<body>
${conv.outerHTML}
<footer class="convo-export-footer">Exported by <a href="https://github.com/jfim/convo">convo</a></footer>
</body>
</html>`;
    exporting = true;
    try {
      const result = await commands.exportHtml("conversation.html", doc);
      if (result.status === "error") errorMessage = formatError(result.error);
    } finally {
      exporting = false;
    }
  }

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
    <button class="export" onclick={exportHtml} disabled={exporting}>
      {exporting ? "Exporting…" : "Export HTML"}
    </button>
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
    align-items: center;
    gap: 1rem;
    padding: 0.4rem 1rem;
    background: #002b36;
    border-top: 1px solid #073642;
  }
  .export {
    font-size: 0.8rem;
    color: #93a1a1;
    background: #073642;
    border: 1px solid #586e75;
    border-radius: 4px;
    padding: 0.2rem 0.6rem;
    cursor: pointer;
  }
  .export:hover:not(:disabled) {
    background: #0a4554;
    color: #eee8d5;
  }
  .export:disabled {
    opacity: 0.5;
    cursor: default;
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
