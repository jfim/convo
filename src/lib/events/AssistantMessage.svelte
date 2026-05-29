<script lang="ts">
  import type { Message } from "$lib/bindings";
  import ContentBlock from "./ContentBlock.svelte";

  let { message }: { message: Message } = $props();
</script>

<div class="assistant-message">
  <div class="header">Assistant</div>
  <div class="body">
    {#if typeof message.content === "string"}
      <p class="text">{message.content}</p>
    {:else}
      {#each message.content as block}
        <ContentBlock {block} />
      {/each}
    {/if}
  </div>
</div>

<style>
  .assistant-message {
    margin: 0.5rem 0;
    background: #002b36;
    border-radius: 6px;
    overflow: hidden;
    border: 1px solid #073642;
  }
  .header {
    font-size: 0.75rem;
    font-weight: 600;
    color: #268bd2;
    background: #073642;
    padding: 0.2rem 0.6rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .body {
    padding: 0.5rem 0.75rem;
  }
  .text {
    margin: 0.25rem 0;
    color: #eee8d5;
    white-space: pre-wrap;
    word-break: break-word;
  }
</style>
