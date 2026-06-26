<script lang="ts">
  import { store } from './portsStore.svelte';
  import { i18n } from './i18n.svelte';
</script>

<div class="flex items-center gap-2 mb-3.5">
  <div class="input flex-1 max-w-[380px]">
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
      <circle cx="7" cy="7" r="4.5"/>
      <path d="M10.5 10.5 L14 14"/>
    </svg>
    <input
      id="filter"
      type="text"
      placeholder={i18n.t('filterPlaceholder')}
      bind:value={store.filter}
    />
  </div>

  <div class="flex-1"></div>

  <!-- Scope toggle -->
  <div class="seg">
    <button class:on={store.devOnly} onclick={() => { store.devOnly = true; }}>{i18n.t('devOnly')}</button>
    <button class:on={!store.devOnly} onclick={() => { store.devOnly = false; }}>{i18n.t('allPorts')}</button>
  </div>

  <!-- Language toggle -->
  <div class="seg">
    <button class:on={i18n.lang === 'zh'} onclick={() => { if (i18n.lang !== 'zh') i18n.toggle(); }}>中</button>
    <button class:on={i18n.lang === 'en'} onclick={() => { if (i18n.lang !== 'en') i18n.toggle(); }}>En</button>
  </div>
</div>

<style>
  .seg {
    display: inline-flex;
    border: 1px solid var(--border);
    border-radius: var(--r-sm);
    overflow: hidden;
    background: var(--surface);
  }
  .seg button {
    padding: 6px 12px;
    font-weight: 500;
    font-size: 12.5px;
    line-height: 1;
    font-family: var(--font-body);
    color: var(--muted);
    border-right: 1px solid var(--border);
    background: transparent;
    cursor: pointer;
    transition: background var(--dur-fast) var(--ease), color var(--dur-fast) var(--ease);
  }
  .seg button:last-child {
    border-right: 0;
  }
  .seg button:hover {
    background: var(--surface-2);
    color: var(--fg);
  }
  .seg button.on {
    background: var(--accent-soft);
    color: var(--accent);
  }
</style>
