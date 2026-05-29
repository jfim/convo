<script lang="ts">
  import Markdown from "$lib/Markdown.svelte";

  let { text }: { text: string } = $props();
</script>

<div class="turn user">
  <div class="role">User</div>
  <Markdown source={text} />
</div>

<style>
  .turn {
    padding: 0.75rem 1rem;
    margin: 0.6rem 0;
    border-radius: 8px;
  }
  .user {
    background: #073642;
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
