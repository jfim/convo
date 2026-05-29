<script lang="ts">
  import { tick } from "svelte";
  import type { RenderItem } from "$lib/bindings";
  import RenderItemView from "$lib/render/RenderItemView.svelte";

  let { items, anchor = null }: { items: RenderItem[]; anchor?: string | null } =
    $props();

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
  {#each items as item, i (keyOf(item, i))}
    <div id={`event-${keyOf(item, i)}`} class="event-wrap">
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
</style>
