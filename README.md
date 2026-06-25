# Helm

本地开发服务器端口管理工具。自动识别 localhost 上运行的开发服务，支持查看、终止、释放端口。

## 功能

- **智能识别** — 基于 Git 仓库、进程名白名单、Docker 检测自动筛选开发服务器
- **仅本地** — 默认只显示 localhost / 127.0.0.1 / 0.0.0.0 / ::1 上的服务
- **进程树** — 追踪 IDE → npm → vite 完整链条，精确终止
- **终止端口** — 优雅终止（SIGTERM） + 强制终止（SIGKILL）
- **找空闲端口** — 优先推荐常用端口，兜底扫描范围
- **浏览器打开** — 一键 http://localhost:{port}
- **系统托盘** — 常驻后台，点击切换窗口
- **中英文切换** — 自动检测系统语言，手动切换持久化
- **操作反馈** — Toast 提示 + 按钮禁用，防止重复点击

## 截图

展开行可查看：Git 分支、仓库名、运行时长、命令行、项目信息

## 安装

从 [Releases](../../releases) 下载最新安装包：

- Windows: `Helm_0.1.0_x64-setup.exe` (NSIS) 或 `.msi`
- macOS: `Helm_0.1.0_aarch64.dmg` / `_x64.dmg`
- Linux: `Helm_0.1.0_amd64.AppImage` 或 `.deb`

## 开发

前置条件：
- Rust 1.75+
- Node 20+ 和 pnpm 9+
- Windows: MSVC Build Tools + Windows SDK
- Linux: `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`

```bash
# 安装前端依赖
cd crates/helm-tauri/frontend
pnpm install

# 开发模式（热更新）
cd ../..
cargo tauri dev

# 生产构建
cargo tauri build
```

## 架构

Rust workspace，三个 crate 共享核心：

```
crates/
├── helm-core    — 跨平台端口扫描、进程信息、Git 检测、端口释放
├── helm-cli     — clap CLI 二进制
└── helm-tauri   — Tauri v2 + Svelte 5 + Tailwind GUI
```

## 开发服务器识别规则

1. TCP LISTEN 状态 + 端口范围 1024~49151
2. 进程在 Git 仓库内 → 无条件视为开发服务器
3. 进程名在运行时白名单中（node/python/go/java/ruby/bun/deno 等）
4. Docker 进程（com.docker.*、vpnkit）
5. CWD 目录名有意义且匹配白名单

## License

MIT
