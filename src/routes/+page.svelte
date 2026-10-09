<script lang="ts">
  import { onMount } from 'svelte';

  import { round } from '$lib/stores/round.svelte';

  onMount(() => {
    void round.refresh();
  });
</script>

<main>
  <h1>抽讲</h1>

  {#if round.state.phase === 'ready'}
    <p class="hint">抽一个术语，限时准备，脱稿讲清楚。</p>
    <button onclick={() => round.draw()}>抽取</button>
    {#if round.state.error}
      <p class="error">上次抽取失败：{round.state.error}</p>
    {/if}
  {:else if round.state.phase === 'generating'}
    <p class="hint">生成中…</p>
    <button disabled>抽取</button>
  {:else if round.state.phase === 'preparing'}
    <p class="word">{round.state.word}</p>
    <button onclick={() => round.speak()}>开始口播</button>
  {:else if round.state.phase === 'speaking'}
    <p class="word">{round.state.word}</p>
    <button onclick={() => round.end()}>结束口播</button>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, sans-serif;
    background: #f7f7f8;
    color: #1f2328;
  }

  main {
    max-width: 32rem;
    margin: 0 auto;
    padding: 3rem 1.5rem;
    text-align: center;
  }

  h1 {
    font-size: 1.5rem;
    font-weight: 600;
  }

  .word {
    margin: 2rem 0;
    font-size: 1.75rem;
    font-weight: 600;
    line-height: 1.4;
  }

  .hint {
    color: #57606a;
  }

  .error {
    color: #b42318;
  }

  button {
    padding: 0.6rem 1.6rem;
    font-size: 1rem;
    border: none;
    border-radius: 0.5rem;
    background: #1f2328;
    color: #fff;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: default;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: #16181d;
      color: #e6e6e6;
    }

    .hint {
      color: #9aa4b2;
    }

    .error {
      color: #ff8a80;
    }

    button {
      background: #e6e6e6;
      color: #16181d;
    }
  }
</style>
