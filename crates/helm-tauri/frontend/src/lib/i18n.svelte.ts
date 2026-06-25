type Lang = 'zh' | 'en';

const translations = {
  zh: {
    title: 'Helm',
    filterPlaceholder: '按端口、进程或命令行筛选…',
    findFree: '找空闲端口',
    free: '空闲',
    refresh: '刷新',
    devOnly: '仅本地',
    allPorts: '全部',
    port: '端口',
    proto: '协议',
    state: '状态',
    process: '进程',
    project: '项目',
    devServer: '开发服务',
    cmdline: '命令行',
    actions: '操作',
    open: '↗',
    kill: '结束',
    force: '强制',
    loading: '加载中…',
    noMatch: '无匹配端口。',
    noDevServer: '本地地址无监听端口。',
    footer: '{filtered} / {total} 个端口 · 每2秒自动刷新',
    killSuccess: '已结束端口 {port}',
    killFail: '结束端口 {port} 失败',
    forceKillSuccess: '已强制结束端口 {port}',
    openFail: '打开端口 {port} 失败',
    freeFound: '找到空闲端口: {port}',
    freeFail: '查找空闲端口失败',
    listening: '监听中',
    established: '已建立',
    timeWait: '等待中',
    closeWait: '关闭等待',
    finWait1: '结束等待1',
    finWait2: '结束等待2',
    closed: '已关闭',
    unknown: '未知',
  },
  en: {
    title: 'Helm',
    filterPlaceholder: 'Filter by port, process, or cmdline…',
    findFree: 'Find Free Port',
    free: 'Free',
    refresh: 'Refresh',
    devOnly: 'Local Only',
    allPorts: 'All',
    port: 'Port',
    proto: 'Proto',
    state: 'State',
    process: 'Process',
    project: 'Project',
    devServer: 'Dev Server',
    cmdline: 'Cmdline',
    actions: 'Actions',
    open: '↗',
    kill: 'Kill',
    force: 'Force',
    loading: 'Loading…',
    noMatch: 'No ports match.',
    noDevServer: 'No services on local addresses.',
    footer: '{filtered} of {total} ports · auto-refresh every 2s',
    killSuccess: 'Killed port {port}',
    killFail: 'Failed to kill port {port}',
    forceKillSuccess: 'Force killed port {port}',
    openFail: 'Failed to open port {port}',
    freeFound: 'Free port found: {port}',
    freeFail: 'Failed to find free port',
    listening: 'Listening',
    established: 'Established',
    timeWait: 'TimeWait',
    closeWait: 'CloseWait',
    finWait1: 'FinWait1',
    finWait2: 'FinWait2',
    closed: 'Closed',
    unknown: 'Unknown',
  },
} satisfies Record<Lang, Record<string, string>>;

function detectLang(): Lang {
  if (typeof localStorage !== 'undefined') {
    const stored = localStorage.getItem('helm-lang');
    if (stored === 'zh' || stored === 'en') return stored;
  }
  if (typeof navigator !== 'undefined' && navigator.language?.startsWith('zh')) {
    return 'zh';
  }
  return 'en';
}

class I18n {
  lang = $state<Lang>(detectLang());

  t(key: string, params?: Record<string, string>): string {
    let text = translations[this.lang][key] ?? key;
    if (params) {
      for (const [k, v] of Object.entries(params)) {
        text = text.replace(`{${k}}`, v);
      }
    }
    return text;
  }

  toggle() {
    this.lang = this.lang === 'zh' ? 'en' : 'zh';
    localStorage.setItem('helm-lang', this.lang);
  }
}

export const i18n = new I18n();
