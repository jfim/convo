<script lang="ts">
  import type { AssistantBlock } from "$lib/bindings";
  import AssistantBlockView from "./AssistantBlockView.svelte";

  let { blocks }: { blocks: AssistantBlock[] } = $props();
</script>

<div class="turn assistant">
  <div class="role">Assistant</div>
  {#each blocks as block}
    <AssistantBlockView {block} />
  {/each}
</div>

<style>
  .turn {
    padding: 0.75rem 1rem;
    margin: 0.6rem 0;
    border-radius: 8px;
  }
  .assistant {
    background: #002b36;
    border: 1px solid #073642;
  }
  .role {
    font-weight: 600;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    opacity: 0.6;
    margin-bottom: 0.4rem;
  }
</style>
