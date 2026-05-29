<script lang="ts">
  let { thinking }: { thinking: string } = $props();
  let open = $state(false);
</script>

<details class="thinking" bind:open>
  <summary>Thinking</summary>
  <pre>{thinking}</pre>
</details>

<style>
  .thinking {
    margin: 0.25rem 0;
    color: #93a1a1;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0.25rem 0 0;
  }
</style>
