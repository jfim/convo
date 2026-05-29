<script lang="ts">
  import { tick } from "svelte";
  import type { ConversationEvent } from "$lib/bindings";
  import EventNode from "$lib/events/EventNode.svelte";

  let { events, anchor = null }: { events: ConversationEvent[]; anchor?: string | null } = $props();

  function keyOf(e: ConversationEvent, i: number): string {
    return e.uuid ?? `idx-${i}`;
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
  {#each events as event, i (keyOf(event, i))}
    <div id={`event-${keyOf(event, i)}`} class="event-wrap">
      <EventNode {event} />
    </div>
  {/each}
</div>

<style>
  .conversation { max-width: 56rem; margin: 0 auto; padding: 1rem; }
  :global(.event-wrap.highlight) { outline: 2px solid #b58900; border-radius: 6px; }
</style>
