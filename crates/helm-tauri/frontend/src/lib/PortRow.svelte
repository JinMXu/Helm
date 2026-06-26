<script lang="ts">
  import type { PortEntry, GitInfo } from './types';
  import { i18n } from './i18n.svelte';

  let {
    entry,
    expanded = false,
    gitInfo = null,
    opening = false,
    killing = false,
    ontoggle = () => {},
    onopen = () => {},
    onkill = () => {},
    onforcekill = () => {},
  }: {
    entry: PortEntry;
    expanded?: boolean;
    gitInfo?: GitInfo | null;
    opening?: boolean;
    killing?: boolean;
    ontoggle?: () => void;
    onopen?: () => void;
    onkill?: () => void;
    onforcekill?: () => void;
  } = $props();

  function statePillClass(state: string): string {
    switch (state) {
      case 'listening': return 'pill-ok';
      case 'established': return 'pill-info';
      case 'timeWait':
      case 'closeWait':
      case 'finWait1':
      case 'finWait2': return 'pill-warn';
      default: return '';
    }
  }

  function devKindLabel(entry: PortEntry): string {
    if (!entry.process?.devServer) return '';
    if (typeof entry.process.devServer === 'string') return entry.process.devServer;
    return entry.process.devServer.other;
  }

  function formatRunning(startTime: number): string {
    const elapsed = Math.floor(Date.now() / 1000) - startTime;
    if (elapsed < 60) return `${elapsed}s`;
    if (elapsed < 3600) return `${Math.floor(elapsed / 60)}m ${elapsed % 60}s`;
    const h = Math.floor(elapsed / 3600);
    const m = Math.floor((elapsed % 3600) / 60);
    return `${h}h ${m}m`;
  }
</script>

<tr
  class="port-row"
  class:expanded
  onclick={ontoggle}
>
  <td class="w-[110px]">
    <div class="flex items-baseline gap-2">
      <span class="port-num">{entry.port}</span>
      <span class="port-proto">{entry.protocol}</span>
    </div>
  </td>
  <td>
    <div class="flex flex-col gap-[3px]">
      <div class="flex items-center gap-2">
        <span class="proc-name">{entry.process?.displayName ?? entry.process?.name ?? '—'}</span>
        {#if devKindLabel(entry)}
          <span class="pill pill-accent text-[9.5px] !py-px !px-1.5 tracking-[0.04em]">{devKindLabel(entry)}</span>
        {/if}
      </div>
      <span class="proc-meta">
        PID {entry.pid}
        {#if entry.process?.startTime}
          · {i18n.t('startedAt')}: {new Date(entry.process.startTime * 1000).toLocaleTimeString(i18n.lang === 'zh' ? 'zh-CN' : 'en-US', { hour12: false })}
        {/if}
      </span>
    </div>
  </td>
  <td class="w-[220px]">
    {#if entry.process?.project}
      <div class="flex items-center gap-2">
        <span class="proj-name">{entry.process.project.name}</span>
        {#if gitInfo?.branch}
          <span class="proj-branch">{gitInfo.branch}</span>
        {/if}
      </div>
    {:else}
      <span class="muted-text text-[11px]">—</span>
    {/if}
  </td>
  <td class="w-[100px]">
    <span class="pill {statePillClass(entry.state)}">
      <span class="pill-dot"></span>
      {i18n.t(entry.state)}
    </span>
  </td>
  <td class="w-[80px]">
    {#if entry.process?.startTime}
      <span class="mono muted-text text-xs">{formatRunning(entry.process.startTime)}</span>
    {:else}
      <span class="mono muted-text text-xs">—</span>
    {/if}
  </td>
  <td class="w-[140px]">
    <div class="row gap-6">
      <button class="btn btn-xs" onclick={(e: MouseEvent) => { e.stopPropagation(); onopen(); }} disabled={opening}>
        {opening ? '...' : i18n.t('open')}
      </button>
      <button class="btn btn-xs btn-signal" onclick={(e: MouseEvent) => { e.stopPropagation(); onkill(); }} disabled={killing}>
        {killing ? '...' : i18n.t('kill')}
      </button>
    </div>
  </td>
</tr>

<style>
  .port-row {
    cursor: pointer;
    transition: background var(--dur-fast) var(--ease);
  }
  .port-row:hover {
    background: var(--surface-2);
  }
  .port-row.expanded {
    background: var(--accent-soft);
  }
  .port-row.expanded td {
    border-bottom: 0;
  }
  .port-row.expanded .port-num {
    color: var(--accent);
  }

  .port-num {
    font-weight: 600;
    font-size: 14px;
    line-height: 1;
    font-family: var(--font-mono);
    color: var(--fg);
    font-variant-numeric: tabular-nums;
  }
  .port-proto {
    font-weight: 500;
    font-size: 10px;
    line-height: 1;
    font-family: var(--font-mono);
    color: var(--muted);
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .proc-name {
    font-weight: 500;
    font-size: 13px;
    line-height: 1.2;
    font-family: var(--font-body);
    color: var(--fg);
  }
  .proc-meta {
    font-weight: 400;
    font-size: 11px;
    line-height: 1;
    font-family: var(--font-mono);
    color: var(--muted);
    letter-spacing: 0.02em;
  }

  .proj-name {
    font-weight: 500;
    font-size: 12.5px;
    line-height: 1;
    font-family: var(--font-body);
    color: var(--fg);
  }
  .proj-branch {
    font-weight: 500;
    font-size: 11px;
    line-height: 1;
    font-family: var(--font-mono);
    color: var(--muted);
  }
  .proj-branch::before {
    content: '';
    display: inline-block;
    width: 4px;
    height: 4px;
    background: var(--border-strong);
    border-radius: 50%;
    margin: 0 6px 2px 0;
    vertical-align: middle;
  }
</style>
