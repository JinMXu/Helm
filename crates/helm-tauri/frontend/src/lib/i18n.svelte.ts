type Lang = 'zh' | 'en';

const translations = {
  zh: {
    title: 'Helm',
    subtitle: '本地端口',
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
    running: '运行',
    devServer: '开发服务',
    cmdline: '命令行',
    actions: '操作',
    open: '↗ 打开',
    kill: '结束',
    force: '强制结束',
    copy: '复制',
    copied: '已复制到剪贴板',
    loading: '加载中…',
    noMatch: '无匹配端口',
    noMatchHint: '尝试更改筛选条件或切换到"全部"',
    noDevServer: '本地地址无监听端口',
    noDevServerHint: '点击"全部"查看所有端口',
    footer: '{filtered} / {total} 个端口',
    footerRefreshing: '每 2 秒自动刷新',
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
    localAddress: '本地地址',
    startedAt: '启动于',
    cwd: '工作目录',
    gitBranch: 'Git 分支',
    repo: '仓库',
    nonGit: '非 Git 仓库',
    detailTitle: '详细信息',
  },
  en: {
    title: 'Helm',
    subtitle: 'Local Ports',
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
    running: 'Running',
    devServer: 'Dev Server',
    cmdline: 'Cmdline',
    actions: 'Actions',
    open: '↗ Open',
    kill: 'Kill',
    force: 'Force Kill',
    copy: 'Copy',
    copied: 'Copied to clipboard',
    loading: 'Loading…',
    noMatch: 'No ports match.',
    noMatchHint: 'Try adjusting the filter or switch to "All".',
    noDevServer: 'No services on local addresses.',
    noDevServerHint: 'Click "All" to see all ports.',
    footer: '{filtered} of {total} ports',
    footerRefreshing: 'auto-refresh every 2s',
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
    localAddress: 'Local Address',
    startedAt: 'Started',
    cwd: 'Working Directory',
    gitBranch: 'Git Branch',
    repo: 'Repository',
    nonGit: 'Not a Git repository',
    detailTitle: 'Details',
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
