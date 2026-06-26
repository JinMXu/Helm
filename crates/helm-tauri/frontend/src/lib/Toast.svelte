<script lang="ts">
  import type { Toast } from './portsStore.svelte';
  import { i18n } from './i18n.svelte';

  let { toast }: { toast: Toast | null } = $props();
</script>

{#if toast}
  <div class="fixed bottom-6 right-6 z-50 flex flex-col gap-2">
    <div
      class:toast-ok={toast.type === 'success'}
      class:toast-err={toast.type === 'error'}
      class="toast-item"
      role="alert"
      key={toast.id}
    >
      {toast.message}
    </div>
  </div>
{/if}

<style>
  .toast-item {
    background: var(--surface);
    border: 1px solid var(--border);
    border-left: 3px solid var(--accent);
    border-radius: var(--r-sm);
    padding: 10px 14px;
    font-weight: 500;
    font-size: 12.5px;
    line-height: 1.4;
    color: var(--fg);
    box-shadow: 0 8px 24px -8px rgba(20, 30, 60, 0.2);
    animation: toast-in 200ms var(--ease);
    min-width: 200px;
  }
  .toast-ok {
    border-left-color: var(--ok);
  }
  .toast-err {
    border-left-color: var(--signal);
  }
  @keyframes toast-in {
    from { opacity: 0; transform: translateX(8px); }
    to { opacity: 1; transform: none; }
  }
</style>
