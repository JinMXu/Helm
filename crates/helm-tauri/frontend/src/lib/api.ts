import { invoke } from '@tauri-apps/api/core';
import type { GitInfo, KillResult, PortEntry, ProcessInfo } from './types';

export async function listPorts(): Promise<PortEntry[]> {
  return invoke<PortEntry[]>('list_ports');
}

export async function portInfo(port: number): Promise<ProcessInfo | null> {
  return invoke<ProcessInfo | null>('port_info', { port });
}

export async function processTree(port: number): Promise<number[]> {
  return invoke<number[]>('process_tree', { port });
}

export async function killPort(port: number, force = false): Promise<KillResult> {
  return invoke<KillResult>('kill_port', { port, force });
}

export async function findFreePort(prefer: number[] = []): Promise<number> {
  return invoke<number>('find_free_port', { prefer });
}

export async function openInBrowser(port: number): Promise<void> {
  await invoke('open_in_browser', { port });
}

export async function getGitInfo(path: string): Promise<GitInfo | null> {
  return invoke<GitInfo | null>('get_git_info', { path });
}
