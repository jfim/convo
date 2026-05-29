<script lang="ts">
  let { message }: { message: string } = $props();
</script>

<div class="error">
  <h1>Could not open conversation</h1>
  <p>{message}</p>
</div>

<style>
  .error { padding: 2rem; color: #dc322f; }
  h1 { font-size: 1.2rem; }
</style>
