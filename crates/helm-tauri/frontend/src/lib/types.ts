// Mirrors helm-core models. serde rename_all = "camelCase" keeps these aligned.

export type Protocol = 'tcp' | 'tcp6' | 'udp' | 'udp6';

export type PortState =
  | 'listening'
  | 'established'
  | 'timeWait'
  | 'closeWait'
  | 'finWait1'
  | 'finWait2'
  | 'closed'
  | 'unknown';

export type DevServerKind =
  | 'vite' | 'next' | 'nuxt' | 'rails' | 'puma' | 'uvicorn'
  | 'gunicorn' | 'flask' | 'express' | 'webpackDev'
  | 'gradleBootRun' | 'dotnetWatch' | 'cargoRun' | 'goRun'
  | 'django' | { other: string };

export interface ProjectInfo {
  root: string;
  name: string;
  icon: string | null;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  exe: string | null;
  cwd: string | null;
  cwdPermissionDenied: boolean;
  cmdline: string[];
  parentPid: number | null;
  startTime: number;
  project: ProjectInfo | null;
  devServer: DevServerKind | null;
  displayName: string | null;
}

export interface PortEntry {
  port: number;
  protocol: Protocol;
  localAddress: string;
  remoteAddress: string | null;
  remotePort: number | null;
  state: PortState;
  pid: number;
  process: ProcessInfo | null;
}

export interface KillResult {
  pid: number;
  graceful: boolean;
  elapsedMs: number;
}

export interface GitInfo {
  branch: string | null;
  repo: string | null;
}
