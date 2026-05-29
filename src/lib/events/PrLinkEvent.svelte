<script lang="ts">
  let { url, number }: { url: string | null | undefined; number: number | null | undefined } = $props();
</script>

<div class="pr-link">
  {#if url}
    <a href={url} target="_blank" rel="noreferrer">
      PR{number != null ? ` #${number}` : ""}
    </a>
  {:else}
    <span class="no-url">PR link (no url)</span>
  {/if}
</div>

<style>
  .pr-link {
    margin: 0.25rem 0;
    font-size: 0.85rem;
  }
  a {
    color: #268bd2;
    text-decoration: underline;
  }
  a:hover {
    color: #2aa198;
  }
  .no-url {
    color: #586e75;
  }
</style>
