<script lang="ts">
  import { tick } from "svelte";
  import type { RenderItem } from "$lib/bindings";
  import RenderItemView from "$lib/render/RenderItemView.svelte";

  let {
    items,
    anchor = null,
    showHidden = false,
  }: {
    items: RenderItem[];
    anchor?: string | null;
    showHidden?: boolean;
  } = $props();

  // Hidden items (attachments, stop-hook summaries, queue ops, filler turns)
  // are omitted unless the "Show hidden" toggle is on.
  let visible = $derived(items.filter((item) => showHidden || !item.hidden));

  // Keyed by uuid so a future live-tail can append without re-rendering.
  function keyOf(item: RenderItem, i: number): string {
    return item.uuid ?? `idx-${i}`;
  }

  $effect(() => {
    if (anchor) void scrollToAnchor(anchor);
  });

  async function scrollToAnchor(id: string) {
    await tick();
    const el = document.getElementById(`event-${id}`);
    if (el) {
      el.scrollIntoView({ behavior: "smooth", block: "start" });
      el.classList.add("highlight");
      setTimeout(() => el.classList.remove("highlight"), 1500);
    }
  }
</script>

<div class="conversation">
  {#each visible as item, i (keyOf(item, i))}
    <div
      id={`event-${keyOf(item, i)}`}
      class="event-wrap"
      class:hidden-item={item.hidden}
    >
      <RenderItemView {item} />
    </div>
  {/each}
</div>

<style>
  .conversation {
    max-width: 56rem;
    margin: 0 auto;
    padding: 1rem;
  }
  :global(.event-wrap.highlight) {
    outline: 2px solid #b58900;
    border-radius: 6px;
  }
  /* Revealed hidden items: dimmed and marked with a left rule. */
  .event-wrap.hidden-item {
    opacity: 0.55;
    border-left: 2px dashed #586e75;
    padding-left: 0.6rem;
    margin-left: -0.6rem;
  }
</style>
