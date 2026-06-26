<script lang="ts">
  import { store } from './portsStore.svelte';
  import { i18n } from './i18n.svelte';

  let { onRefresh }: { onRefresh: () => void } = $props();
</script>

<header class="flex items-center gap-3.5 mb-5">
  <div class="flex items-center gap-2.5 mr-auto">
    <svg class="w-[30px] h-[30px] text-accent shrink-0" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <circle cx="32" cy="32" r="6" />
      <circle cx="32" cy="32" r="22" />
      <path d="M32 10 V20 M32 44 V54 M10 32 H20 M44 32 H54" />
    </svg>
    <div>
      <div class="font-semibold text-[17px] leading-none tracking-[-0.01em]">{i18n.t('title')}</div>
      <div class="font-medium text-[10px] leading-none text-muted tracking-[0.1em] uppercase mt-[3px]">{i18n.t('subtitle')}</div>
    </div>
  </div>

  <button class="btn" onclick={() => store.findFree()} disabled={store.freeLoading}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="8" cy="8" r="6"/>
      <path d="M8 5 V8 L10 10"/>
    </svg>
    {store.freeLoading ? '...' : i18n.t('findFree')}
  </button>

  <button class="btn btn-icon" onclick={onRefresh} disabled={store.loading} title={i18n.t('refresh')}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M2 8 A6 6 0 0 1 14 8"/>
      <polyline points="14 4 14 8 10 8"/>
      <path d="M14 8 A6 6 0 0 1 2 8"/>
      <polyline points="2 12 2 8 6 8"/>
    </svg>
  </button>
</header>
