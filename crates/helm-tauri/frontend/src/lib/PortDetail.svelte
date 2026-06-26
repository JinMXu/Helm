<script lang="ts">
  import type { PortEntry, GitInfo } from './types';
  import { i18n } from './i18n.svelte';

  let {
    entry,
    gitInfo = null,
    opening = false,
    killing = false,
    onopen = () => {},
    onkill = () => {},
    onforcekill = () => {},
    oncopy = () => {},
  }: {
    entry: PortEntry;
    gitInfo?: GitInfo | null;
    opening?: boolean;
    killing?: boolean;
    onopen?: () => void;
    onkill?: () => void;
    onforcekill?: () => void;
    oncopy?: () => void;
  } = $props();
</script>

<tr class="detail-row">
  <td colspan="6">
    <div class="detail">
      <div class="detail-grid">
        <div>
          <div class="detail-block-label">{i18n.t('cmdline')}</div>
          <div class="detail-cmdline">
            <button class="btn btn-xs btn-ghost detail-cmdline-copy" onclick={(e: MouseEvent) => { e.stopPropagation(); oncopy(); }}>
              {i18n.t('copy')}
            </button>
            <span class="mono">{entry.process?.cmdline?.join(' ') ?? '—'}</span>
          </div>
        </div>
        <div>
          <div class="detail-block-label">{i18n.t('detailTitle')}</div>
          <dl class="detail-kv">
            <dt>{i18n.t('localAddress')}</dt>
            <dd>{entry.localAddress}:{entry.port}</dd>
            <dt>{i18n.t('process')}</dt>
            <dd>{entry.process?.name ?? '—'} · PID {entry.pid}</dd>
            {#if entry.process?.startTime}
              <dt>{i18n.t('startedAt')}</dt>
              <dd>{new Date(entry.process.startTime * 1000).toLocaleString(i18n.lang === 'zh' ? 'zh-CN' : 'en-US')}</dd>
            {/if}
            {#if entry.process?.cwd}
              <dt>{i18n.t('cwd')}</dt>
              <dd>{entry.process.cwd}</dd>
            {/if}
            {#if entry.process?.project}
              <dt>{i18n.t('project')}</dt>
              <dd>{entry.process.project.name}</dd>
              {#if gitInfo?.branch}
                <dt>{i18n.t('gitBranch')}</dt>
                <dd>{gitInfo.branch}</dd>
              {/if}
              {#if gitInfo?.repo}
                <dt>{i18n.t('repo')}</dt>
                <dd>{gitInfo.repo}</dd>
              {/if}
            {:else}
              <dt>{i18n.t('project')}</dt>
              <dd class="muted-text">{i18n.t('nonGit')}</dd>
            {/if}
          </dl>
        </div>
      </div>
      <div class="detail-actions">
        <button class="btn btn-xs" onclick={(e: MouseEvent) => { e.stopPropagation(); onopen(); }} disabled={opening}>
          {opening ? '...' : i18n.t('open')}
        </button>
        <button class="btn btn-xs" onclick={(e: MouseEvent) => { e.stopPropagation(); onkill(); }} disabled={killing}>
          {killing ? '...' : i18n.t('kill')}
        </button>
        <button class="btn btn-xs btn-signal" onclick={(e: MouseEvent) => { e.stopPropagation(); onforcekill(); }} disabled={killing}>
          {killing ? '...' : i18n.t('force')}
        </button>
      </div>
    </div>
  </td>
</tr>

<style>
  .detail-row td {
    padding: 0 !important;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
  }
  .detail {
    padding: 14px 18px 16px;
    border-top: 1px solid var(--accent-soft);
  }
  .detail-grid {
    display: grid;
    grid-template-columns: 1.4fr 1fr;
    gap: 14px 32px;
  }
  .detail-block-label {
    font-weight: 500;
    font-size: 10px;
    line-height: 1;
    font-family: var(--font-mono);
    color: var(--muted);
    letter-spacing: 0.1em;
    text-transform: uppercase;
    margin-bottom: 6px;
  }
  .detail-cmdline {
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: var(--r-sm);
    padding: 10px 12px;
    font-weight: 400;
    font-size: 12px;
    line-height: 1.5;
    font-family: var(--font-mono);
    color: var(--fg);
    word-break: break-all;
    position: relative;
  }
  .detail-cmdline-copy {
    position: absolute;
    top: 6px;
    right: 6px;
  }
  .detail-kv {
    display: grid;
    grid-template-columns: 84px 1fr;
    gap: 5px 12px;
    font-size: 12px;
  }
  .detail-kv dt {
    font-weight: 500;
    font-size: 10px;
    line-height: 1.4;
    font-family: var(--font-mono);
    color: var(--muted);
    letter-spacing: 0.06em;
    text-transform: uppercase;
    padding-top: 1px;
  }
  .detail-kv dd {
    font-family: var(--font-mono);
    font-size: 12px;
    word-break: break-all;
  }
  .detail-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px dashed var(--border);
  }
</style>
