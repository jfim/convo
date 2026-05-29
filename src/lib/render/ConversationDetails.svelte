<script lang="ts">
  import type { ConversationStats } from "$lib/bindings";

  let { stats }: { stats: ConversationStats } = $props();

  const fmt = (n: number) => n.toLocaleString();
  const pct = (r: number | null) =>
    r === null ? "—" : `${(r * 100).toFixed(1)}%`;

  let totalTokens = $derived(
    stats.tokens.input +
      stats.tokens.output +
      stats.tokens.cacheCreation +
      stats.tokens.cacheRead,
  );
</script>

<details class="details">
  <summary>
    Conversation details
    <span class="muted">· {fmt(stats.turns)} turns</span>
  </summary>

  <div class="grid">
    <div class="row"><span class="k">Turns</span><span class="v"
        >{fmt(stats.turns)}
        <span class="muted"
          >({fmt(stats.userTurns)} user, {fmt(stats.assistantTurns)} assistant)</span
        ></span
      ></div>

    {#if stats.models.length}
      <div class="row"><span class="k">Model</span><span class="v"
          >{stats.models.join(", ")}</span
        ></div>
    {/if}
    {#if stats.cwd}
      <div class="row"><span class="k">Working dir</span><span class="v mono"
          >{stats.cwd}</span
        ></div>
    {/if}
    {#if stats.gitBranch}
      <div class="row"><span class="k">Git branch</span><span class="v mono"
          >{stats.gitBranch}</span
        ></div>
    {/if}
    {#if stats.version}
      <div class="row"><span class="k">CLI version</span><span class="v mono"
          >{stats.version}</span
        ></div>
    {/if}

    <div class="row"><span class="k">Tokens</span><span class="v"
        >{fmt(totalTokens)} total</span
      ></div>
    <div class="row sub"><span class="k">· input / output</span><span class="v"
        >{fmt(stats.tokens.input)} / {fmt(stats.tokens.output)}</span
      ></div>
    <div class="row sub"><span class="k">· cache write / read</span><span
        class="v">{fmt(stats.tokens.cacheCreation)} / {fmt(stats.tokens.cacheRead)}</span
      ></div>
    <div class="row sub"><span class="k">· cache hit rate</span><span class="v"
        >{pct(stats.cacheHitRate)}</span
      ></div>

    <div class="row"><span class="k">Tool calls</span><span class="v"
        >{fmt(stats.totalToolCalls)}
        {#if stats.failedToolCalls > 0}<span class="err"
            >({fmt(stats.failedToolCalls)} failed)</span
          >{/if}</span
      ></div>
    {#each stats.toolCalls as t (t.name)}
      <div class="row sub"><span class="k mono">· {t.name}</span><span class="v"
          >{fmt(t.count)}</span
        ></div>
    {/each}
  </div>
</details>

<style>
  .details {
    margin: 1rem 0 2rem;
    background: #073642;
    border: 1px solid #586e75;
    border-radius: 6px;
    padding: 0.5rem 0.8rem;
    color: #93a1a1;
    font-size: 0.85rem;
  }
  summary {
    cursor: pointer;
    color: #839496;
    font-weight: 600;
    user-select: none;
  }
  .muted {
    color: #586e75;
    font-weight: 400;
  }
  .err {
    color: #cb4b16;
  }
  .grid {
    margin-top: 0.6rem;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }
  .row {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.15rem 0;
    border-bottom: 1px solid #08303b;
  }
  .row.sub {
    border-bottom: none;
    padding: 0.05rem 0 0.05rem 0.6rem;
    color: #839496;
  }
  .k {
    color: #586e75;
  }
  .row.sub .k {
    color: #657b83;
  }
  .v {
    text-align: right;
  }
  .mono {
    font-family: ui-monospace, monospace;
    word-break: break-all;
  }
</style>
