<script lang="ts">
  import { store } from './lib/portsStore.svelte';
  import { i18n } from './lib/i18n.svelte';
  import { getGitInfo } from './lib/api';
  import type { DevServerKind, GitInfo, PortEntry } from './lib/types';

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
    // Fetch git info for the project root
    const root = entry.process?.project?.root;
    if (root && !(root in gitCache)) {
      gitCache[root] = await getGitInfo(root);
    }
  }

  function formatDuration(startTime: number): string {
    const now = Math.floor(Date.now() / 1000);
    const elapsed = now - startTime;
    if (elapsed < 60) return `${elapsed}s`;
    if (elapsed < 3600) return `${Math.floor(elapsed / 60)}m ${elapsed % 60}s`;
    const h = Math.floor(elapsed / 3600);
    const m = Math.floor((elapsed % 3600) / 60);
    return `${h}h ${m}m`;
  }

  function devServerLabel(kind: DevServerKind | null): string {
    if (!kind) return '';
    if (typeof kind === 'string') return kind;
    return kind.other;
  }

  function projectIcon(icon: string | null): string {
    const map: Record<string, string> = {
      node: '📦',
      rust: '🦀',
      python: '🐍',
      go: '🐹',
      java: '☕',
      dotnet: '🔵',
    };
    return icon ? (map[icon] ?? '📁') : '';
  }
</script>

<div class="flex flex-col h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
  <!-- Toast -->
  {#if store.toast}
    <div
      class="fixed top-3 right-3 z-50 px-4 py-2 rounded-md shadow-lg text-sm font-medium transition-opacity
        {store.toast.type === 'success' ? 'bg-green-600 text-white' : 'bg-red-600 text-white'}"
      role="alert"
      key={store.toast.id}
    >
      {store.toast.message}
    </div>
  {/if}

  <header class="flex items-center gap-3 px-4 py-3 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
    <h1 class="text-lg font-semibold whitespace-nowrap">{i18n.t('title')}</h1>
    <input
      type="text"
      placeholder={i18n.t('filterPlaceholder')}
      bind:value={store.filter}
      class="flex-1 min-w-0 px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-gray-50 dark:bg-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
    <button
      onclick={() => store.findFree()}
      disabled={store.freeLoading}
      class="px-3 py-1.5 text-sm bg-green-600 hover:bg-green-700 text-white rounded-md disabled:opacity-50 whitespace-nowrap"
    >
      {store.freeLoading ? '...' : i18n.t('findFree')}
    </button>
    {#if store.freeResult !== null && !store.freeLoading}
      <span class="text-sm text-green-600 dark:text-green-400 whitespace-nowrap">
        {i18n.t('free')}: <strong>{store.freeResult}</strong>
      </span>
    {/if}
    <button
      onclick={() => store.refresh()}
      disabled={store.loading}
      class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-50 whitespace-nowrap"
    >
      {store.loading ? '⟳' : i18n.t('refresh')}
    </button>
    <button
      onclick={() => { store.devOnly = !store.devOnly; }}
      class="px-2 py-1.5 text-sm rounded-md whitespace-nowrap
        {store.devOnly ? 'bg-blue-600 text-white hover:bg-blue-700' : 'border border-gray-300 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700'}"
    >
      {store.devOnly ? i18n.t('devOnly') : i18n.t('allPorts')}
    </button>
    <button
      onclick={() => i18n.toggle()}
      class="px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 whitespace-nowrap"
      title="Switch language"
    >
      {i18n.lang === 'zh' ? 'EN' : '中文'}
    </button>
  </header>

  <main class="flex-1 overflow-auto">
    {#if store.loading && store.ports.length === 0}
      <div class="flex items-center justify-center h-full text-gray-400">
        {i18n.t('loading')}
      </div>
    {:else if store.filtered.length === 0}
      <div class="flex flex-col items-center justify-center h-full text-gray-400 text-center px-8 gap-2">
        <span>{store.devOnly ? i18n.t('noDevServer') : i18n.t('noMatch')}</span>
        {#if store.devOnly}
          <button
            onclick={() => { store.devOnly = false; }}
            class="px-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700"
          >
            {i18n.t('allPorts')}
          </button>
        {/if}
      </div>
    {:else}
      <table class="w-full text-sm">
        <thead class="sticky top-0 z-10 bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-300 uppercase text-xs">
          <tr>
            <th class="px-3 py-2 text-left w-20">{i18n.t('port')}</th>
            <th class="px-3 py-2 text-left">{i18n.t('process')}</th>
            <th class="px-3 py-2 text-left">{i18n.t('devServer')}</th>
            <th class="px-3 py-2 text-left w-24">{i18n.t('state')}</th>
            <th class="px-3 py-2 text-right">{i18n.t('actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each store.filtered as entry (rowKey(entry))}
            {@const key = rowKey(entry)}
            {@const open = expandedKey === key}
            <!-- Compact row -->
            <tr
              class="border-b border-gray-200 dark:border-gray-700 hover:bg-blue-50 dark:hover:bg-gray-800 cursor-pointer"
              onclick={() => toggleExpand(entry)}
            >
              <td class="px-3 py-2 font-mono font-semibold align-top">
                <span class="inline-block w-4 text-gray-400">{open ? '▾' : '▸'}</span>
                {entry.port}
              </td>
              <td class="px-3 py-2 align-top">
                <span class="font-mono font-medium">
                  {entry.process?.displayName ?? entry.process?.name ?? '—'}
                </span>
                {#if entry.process?.displayName && entry.process?.displayName !== entry.process?.name}
                  <div class="text-xs text-gray-400 font-mono">{entry.process.name}</div>
                {/if}
                <div class="text-xs text-gray-400">pid {entry.pid}</div>
              </td>
              <td class="px-3 py-2 align-top">
                {#if entry.process?.devServer}
                  <span class="px-2 py-0.5 rounded text-xs bg-purple-100 text-purple-700 dark:bg-purple-900 dark:text-purple-300 font-medium">
                    {devServerLabel(entry.process.devServer)}
                  </span>
                {:else}
                  <span class="text-gray-400">—</span>
                {/if}
              </td>
              <td class="px-3 py-2 align-top">
                <span
                  class="px-2 py-0.5 rounded text-xs font-medium
                  {entry.state === 'listening' ? 'bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300' : ''}
                  {entry.state === 'established' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300' : ''}
                  {entry.state === 'timeWait' || entry.state === 'closeWait' ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900 dark:text-yellow-300' : ''}
                  {entry.state === 'unknown' ? 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300' : ''}"
                >
                  {i18n.t(entry.state)}
                </span>
              </td>
              <td class="px-3 py-2 text-right align-top">
                <div class="flex items-center justify-end gap-0.5 whitespace-nowrap">
                  <button
                    onclick={(e) => { e.stopPropagation(); store.open(entry.port); }}
                    disabled={store.opening.has(entry.port)}
                    class="px-1.5 py-0.5 text-xs border border-gray-300 rounded hover:bg-gray-100 dark:border-gray-600 dark:hover:bg-gray-700 disabled:opacity-40"
                    title="Open in browser"
                  >
                    {store.opening.has(entry.port) ? '...' : i18n.t('open')}
                  </button>
                  <button
                    onclick={(e) => { e.stopPropagation(); store.kill(entry.port); }}
                    disabled={store.killing.has(entry.port)}
                    class="px-1.5 py-0.5 text-xs bg-red-600 hover:bg-red-700 text-white rounded disabled:opacity-40"
                    title="Kill (graceful)"
                  >
                    {store.killing.has(entry.port) ? '...' : i18n.t('kill')}
                  </button>
                  <button
                    onclick={(e) => { e.stopPropagation(); store.kill(entry.port, true); }}
                    disabled={store.killing.has(entry.port)}
                    class="px-1.5 py-0.5 text-xs bg-red-800 hover:bg-red-900 text-white rounded disabled:opacity-40"
                    title="Force kill"
                  >
                    {store.killing.has(entry.port) ? '...' : i18n.t('force')}
                  </button>
                </div>
              </td>
            </tr>
            <!-- Expanded detail -->
            {#if open}
              {@const git = entry.process?.project?.root ? gitCache[entry.process.project.root] : null}
              <tr class="border-b border-gray-200 dark:border-gray-700 bg-blue-50/50 dark:bg-gray-800/50">
                <td colspan="5" class="px-6 py-3">
                  <div class="grid grid-cols-3 gap-x-6 gap-y-1.5 text-xs">
                    <div>
                      <span class="text-gray-400">{i18n.t('proto')}:</span>
                      <span class="ml-1 uppercase font-mono">{entry.protocol}</span>
                    </div>
                    <div>
                      <span class="text-gray-400">Local:</span>
                      <span class="ml-1 font-mono">{entry.localAddress}:{entry.port}</span>
                    </div>
                    <div>
                      <span class="text-gray-400">PID:</span>
                      <span class="ml-1 font-mono">{entry.pid}</span>
                    </div>
                    {#if entry.process}
                      <div class="col-span-3">
                        <span class="text-gray-400">{i18n.t('port')}:</span>
                        <span class="ml-1 font-mono font-semibold">{entry.port}</span>
                        <span class="text-gray-400 ml-3">Started:</span>
                        <span class="ml-1 font-mono">{formatDuration(entry.process.startTime)} ago</span>
                      </div>
                      {#if entry.process.cwd}
                        <div class="col-span-3">
                          <span class="text-gray-400">CWD:</span>
                          <span class="ml-1 font-mono break-all">{entry.process.cwd}</span>
                        </div>
                      {/if}
                      {#if entry.process.cmdline.length}
                        <div class="col-span-3">
                          <span class="text-gray-400">Cmd:</span>
                          <span class="ml-1 font-mono break-all">{entry.process.cmdline.join(' ')}</span>
                        </div>
                      {/if}
                      {#if entry.process.project}
                        <div class="col-span-3 flex items-center gap-4">
                          <span>
                            <span class="text-gray-400">{i18n.t('project')}:</span>
                            <span class="ml-1 font-mono">
                              {projectIcon(entry.process.project.icon)}
                              {entry.process.project.name}
                            </span>
                          </span>
                          {#if git?.branch}
                            <span>
                              <span class="text-gray-400">Branch:</span>
                              <span class="ml-1 font-mono text-blue-600 dark:text-blue-400">{git.branch}</span>
                            </span>
                          {/if}
                          {#if git?.repo}
                            <span>
                              <span class="text-gray-400">Repo:</span>
                              <span class="ml-1 font-mono">{git.repo}</span>
                            </span>
                          {/if}
                        </div>
                      {/if}
                    {/if}
                  </div>
                </td>
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>
    {/if}
  </main>

  <footer class="px-4 py-1.5 text-xs text-gray-400 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
    {i18n.t('footer', { filtered: String(store.filtered.length), total: String(store.ports.length) })}
  </footer>
</div>
