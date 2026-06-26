<script lang="ts">
  import { store } from './lib/portsStore.svelte';
  import { i18n } from './lib/i18n.svelte';
  import { getGitInfo } from './lib/api';
  import type { GitInfo, PortEntry } from './lib/types';
  import Header from './lib/Header.svelte';
  import Toolbar from './lib/Toolbar.svelte';
  import PortRow from './lib/PortRow.svelte';
  import PortDetail from './lib/PortDetail.svelte';
  import Toast from './lib/Toast.svelte';

  let expandedKey = $state<string | null>(null);
  let gitCache = $state<Record<string, GitInfo | null>>({});

  function rowKey(e: PortEntry): string {
    return `${e.port}|${e.protocol}|${e.pid}|${e.localAddress}`;
  }

  async function toggleExpand(entry: PortEntry) {
    const key = rowKey(entry);
    if (expandedKey === key) {
      expandedKey = null;
      return;
    }
    expandedKey = key;
    const root = entry.process?.project?.root;
    if (root && !(root in gitCache)) {
      gitCache[root] = await getGitInfo(root);
    }
  }

  function onCopyCmdline(entry: PortEntry) {
    const text = entry.process?.cmdline?.join(' ') ?? '';
    if (text && typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(text);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;
    if (e.key === '/' || e.key === 'f') {
      e.preventDefault();
      document.getElementById('filter')?.focus();
    } else if (e.key === 'r' || e.key === 'R') {
      store.refresh();
    } else if (e.key === 'Escape') {
      expandedKey = null;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="wrap">
  <Header onRefresh={() => store.refresh()} />

  <Toolbar />

  <div class="table-wrap">
    {#if store.loading && store.ports.length === 0}
      <div class="empty">
        <div class="empty-title">{i18n.t('loading')}</div>
      </div>
    {:else if store.filtered.length === 0}
      <div class="empty">
        <div class="empty-title">{store.devOnly ? i18n.t('noDevServer') : i18n.t('noMatch')}</div>
        <div class="empty-sub">{store.devOnly ? i18n.t('noDevServerHint') : i18n.t('noMatchHint')}</div>
      </div>
    {:else}
      <table class="tbl">
        <thead>
          <tr>
            <th style="width:110px">{i18n.t('port')}</th>
            <th>{i18n.t('process')}</th>
            <th style="width:220px">{i18n.t('project')}</th>
            <th style="width:100px">{i18n.t('state')}</th>
            <th style="width:80px">{i18n.t('running')}</th>
            <th style="width:140px">{i18n.t('actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each store.filtered as entry (rowKey(entry))}
            {@const key = rowKey(entry)}
            {@const open = expandedKey === key}
            {@const gitInfo = entry.process?.project?.root ? gitCache[entry.process.project.root] ?? null : null}
            <PortRow
              {entry}
              expanded={open}
              {gitInfo}
              opening={store.opening.has(entry.port)}
              killing={store.killing.has(entry.port)}
              ontoggle={() => toggleExpand(entry)}
              onopen={() => store.open(entry.port)}
              onkill={() => store.kill(entry.port)}
              onforcekill={() => store.kill(entry.port, true)}
            />
            {#if open}
              <PortDetail
                {entry}
                {gitInfo}
                opening={store.opening.has(entry.port)}
                killing={store.killing.has(entry.port)}
                onopen={() => store.open(entry.port)}
                onkill={() => store.kill(entry.port)}
                onforcekill={() => store.kill(entry.port, true)}
                oncopy={() => onCopyCmdline(entry)}
              />
            {/if}
          {/each}
        </tbody>
      </table>
    {/if}
  </div>

  <footer class="foot">
    <span><span class="dot"></span>{i18n.t('footer', { filtered: String(store.filtered.length), total: String(store.ports.length) })}</span>
    <span class="mono muted-text">{i18n.t('footerRefreshing')}</span>
  </footer>
</div>

<Toast toast={store.toast} />

<style>
  .wrap {
    max-width: 1080px;
    margin: 0 auto;
    padding: 28px 32px 48px;
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .table-wrap {
    flex: 1;
    overflow: auto;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--r-md);
  }

  .empty {
    padding: 56px 32px;
    text-align: center;
    color: var(--muted);
  }
  .empty-title {
    font-weight: 500;
    font-size: 14px;
    line-height: 1.4;
    font-family: var(--font-body);
    color: var(--fg);
    margin-bottom: 6px;
  }
  .empty-sub {
    font-weight: 400;
    font-size: 12px;
    line-height: 1.5;
    font-family: var(--font-mono);
    color: var(--muted);
  }

  .foot {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 14px;
    font-weight: 400;
    font-size: 11px;
    line-height: 1.5;
    font-family: var(--font-mono);
    color: var(--muted);
    letter-spacing: 0.04em;
  }
  .foot .dot {
    width: 6px;
    height: 6px;
    background: var(--ok);
    border-radius: 50%;
    margin-right: 6px;
    display: inline-block;
    vertical-align: middle;
  }
</style>
