<script lang="ts">
  let { attachment }: { attachment: unknown } = $props();
</script>

<details class="attachment">
  <summary>Attachment</summary>
  <pre>{JSON.stringify(attachment, null, 2)}</pre>
</details>

<style>
  .attachment {
    margin: 0.25rem 0;
    background: #073642;
    border-radius: 4px;
    padding: 0.4rem 0.6rem;
    color: #93a1a1;
  }
  summary {
    font-size: 0.8rem;
    cursor: pointer;
    color: #839496;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0.25rem 0 0;
    font-size: 0.85rem;
  }
</style>
