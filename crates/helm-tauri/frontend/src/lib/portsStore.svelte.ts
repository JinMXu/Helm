import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import type { PortEntry } from './types';
import { killPort as apiKill, openInBrowser as apiOpen, findFreePort as apiFindFree } from './api';
import { i18n } from './i18n.svelte';

function dedupe(entries: PortEntry[]): PortEntry[] {
  const seen = new Set<string>();
  return entries.filter((e) => {
    const key = `${e.port}|${e.protocol}|${e.pid}|${e.localAddress}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function isLocalAddress(addr: string): boolean {
  return addr === '127.0.0.1' || addr === '::1' || addr === '0.0.0.0'
    || addr.startsWith('127.') || addr.startsWith('::ffff:127.');
}

export interface Toast {
  id: number;
  message: string;
  type: 'success' | 'error';
}

let toastId = 0;

class PortsStore {
  ports = $state<PortEntry[]>([]);
  filter = $state('');
  devOnly = $state(true);
  loading = $state(false);
  killing = $state<Set<number>>(new Set());
  opening = $state<Set<number>>(new Set());
  freeLoading = $state(false);
  freeResult = $state<number | null>(null);
  toast = $state<Toast | null>(null);

  constructor() {
    this.refresh();
    listen<PortEntry[]>('helm://update', (event) => {
      this.ports = dedupe(event.payload);
      this.loading = false;
    });
  }

  get filtered(): PortEntry[] {
    let list = this.ports;
    if (this.devOnly) {
      list = list.filter((e) =>
        e.process?.displayName != null && isLocalAddress(e.localAddress)
      );
    }
    if (!this.filter) return list;
    const needle = this.filter.toLowerCase();
    return list.filter((e) => {
      if (e.port.toString().includes(needle)) return true;
      if (e.process) {
        if (e.process.name.toLowerCase().includes(needle)) return true;
        if (e.process.cmdline.some((c) => c.toLowerCase().includes(needle))) return true;
      }
      return false;
    });
  }

  private showToast(message: string, type: 'success' | 'error') {
    this.toast = { id: ++toastId, message, type };
    setTimeout(() => {
      if (this.toast?.id === toastId) {
        this.toast = null;
      }
    }, 2500);
  }

  async refresh() {
    this.loading = true;
    try {
      this.ports = dedupe(await invoke<PortEntry[]>('list_ports'));
    } finally {
      this.loading = false;
    }
  }

  async kill(port: number, force = false) {
    this.killing = new Set([...this.killing, port]);
    try {
      await apiKill(port, force);
      this.showToast(
        force ? i18n.t('forceKillSuccess', { port: String(port) }) : i18n.t('killSuccess', { port: String(port) }),
        'success',
      );
      await this.refresh();
    } catch {
      this.showToast(i18n.t('killFail', { port: String(port) }), 'error');
    } finally {
      const next = new Set(this.killing);
      next.delete(port);
      this.killing = next;
    }
  }

  async open(port: number) {
    this.opening = new Set([...this.opening, port]);
    try {
      await apiOpen(port);
    } catch {
      this.showToast(i18n.t('openFail', { port: String(port) }), 'error');
    } finally {
      const next = new Set(this.opening);
      next.delete(port);
      this.opening = next;
    }
  }

  async findFree(prefer: number[] = [3000, 5173, 8000, 8080]) {
    this.freeLoading = true;
    this.freeResult = null;
    try {
      this.freeResult = await apiFindFree(prefer);
      this.showToast(i18n.t('freeFound', { port: String(this.freeResult) }), 'success');
    } catch {
      this.showToast(i18n.t('freeFail'), 'error');
    } finally {
      this.freeLoading = false;
    }
  }
}

export const store = new PortsStore();
