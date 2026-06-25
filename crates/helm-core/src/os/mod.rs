//! OS abstraction layer.
//!
//! Traits defined here are implemented per-platform (killers) or via
//! cross-platform crates (scanner via `netstat2`, process info via `sysinfo`).
//! `services()` returns the appropriate bundle for the current OS.

use crate::error::Result;
use crate::models::{KillResult, ProcessInfo, Protocol, RawSocket};
use std::sync::Mutex;
use std::time::Duration;

#[cfg(windows)]
mod windows_killer;
#[cfg(unix)]
mod unix_killer;

pub trait PortScanner: Send + Sync {
    fn scan_tcp(&self) -> Result<Vec<RawSocket>>;
    fn scan_udp(&self) -> Result<Vec<RawSocket>>;
}

pub trait ProcessInfoProvider: Send + Sync {
    fn by_pid(&self, pid: u32) -> Option<ProcessInfo>;
    fn process_tree(&self, pid: u32) -> Vec<u32>;
}

pub trait ProcessKiller: Send + Sync {
    fn kill_graceful(&self, pid: u32, timeout: Duration) -> Result<KillResult>;
    fn kill_force(&self, pid: u32) -> Result<()>;
}

pub struct Services {
    pub scanner: Box<dyn PortScanner>,
    pub process: Box<dyn ProcessInfoProvider>,
    pub killer: Box<dyn ProcessKiller>,
}

pub fn services() -> Services {
    Services {
        scanner: Box::new(NetstatScanner),
        process: Box::new(SysinfoProvider::new()),
        #[cfg(windows)]
        killer: Box::new(windows_killer::WindowsKiller),
        #[cfg(unix)]
        killer: Box::new(unix_killer::UnixKiller),
    }
}

// ============================================================================
// Cross-platform PortScanner (netstat2)
// ============================================================================

pub struct NetstatScanner;

impl PortScanner for NetstatScanner {
    fn scan_tcp(&self) -> Result<Vec<RawSocket>> {
        use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

        let addrs = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let protos = ProtocolFlags::TCP;
        let sockets = netstat2::get_sockets_info(addrs, protos)
            .map_err(|e| crate::error::Error::Scan(format!("netstat2 TCP scan: {e}")))?;

        let mut out = Vec::with_capacity(sockets.len());
        for sock in sockets {
            let ProtocolSocketInfo::Tcp(tcp) = sock.protocol_socket_info else {
                continue;
            };
            let pid = sock.associated_pids.first().copied().unwrap_or(0);
            out.push(RawSocket {
                port: tcp.local_port,
                protocol: if tcp.local_addr.is_ipv6() {
                    Protocol::Tcp6
                } else {
                    Protocol::Tcp
                },
                local_address: tcp.local_addr,
                remote_address: Some(tcp.remote_addr),
                remote_port: Some(tcp.remote_port),
                state: map_tcp_state(tcp.state),
                pid,
            });
        }
        Ok(out)
    }

    fn scan_udp(&self) -> Result<Vec<RawSocket>> {
        use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

        let addrs = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let protos = ProtocolFlags::UDP;
        let sockets = netstat2::get_sockets_info(addrs, protos)
            .map_err(|e| crate::error::Error::Scan(format!("netstat2 UDP scan: {e}")))?;

        let mut out = Vec::with_capacity(sockets.len());
        for sock in sockets {
            let ProtocolSocketInfo::Udp(udp) = sock.protocol_socket_info else {
                continue;
            };
            let pid = sock.associated_pids.first().copied().unwrap_or(0);
            out.push(RawSocket {
                port: udp.local_port,
                protocol: if udp.local_addr.is_ipv6() {
                    Protocol::Udp6
                } else {
                    Protocol::Udp
                },
                local_address: udp.local_addr,
                remote_address: None,
                remote_port: None,
                state: crate::models::PortState::Unknown,
                pid,
            });
        }
        Ok(out)
    }
}

fn map_tcp_state(state: netstat2::TcpState) -> crate::models::PortState {
    use crate::models::PortState;
    use netstat2::TcpState;
    match state {
        TcpState::Listen => PortState::Listening,
        TcpState::SynSent => PortState::Unknown,
        TcpState::SynReceived => PortState::Unknown,
        TcpState::Established => PortState::Established,
        TcpState::FinWait1 => PortState::FinWait1,
        TcpState::FinWait2 => PortState::FinWait2,
        TcpState::CloseWait => PortState::CloseWait,
        TcpState::Closing => PortState::Unknown,
        TcpState::LastAck => PortState::Unknown,
        TcpState::TimeWait => PortState::TimeWait,
        TcpState::DeleteTcb => PortState::Closed,
        TcpState::Closed => PortState::Closed,
        TcpState::Unknown => PortState::Unknown,
    }
}

// ============================================================================
// Cross-platform ProcessInfoProvider (sysinfo)
// ============================================================================

pub struct SysinfoProvider {
    sys: Mutex<sysinfo::System>,
}

impl SysinfoProvider {
    pub fn new() -> Self {
        // new_all() refreshes everything including cwd, cmdline, exe.
        let sys = sysinfo::System::new_all();
        Self { sys: Mutex::new(sys) }
    }
}

impl Default for SysinfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessInfoProvider for SysinfoProvider {
    fn by_pid(&self, pid: u32) -> Option<ProcessInfo> {
        let mut sys = self.sys.lock().ok()?;
        let pid_sys = sysinfo::Pid::from_u32(pid);
        // Re-refresh this specific process with full specifics to get fresh cwd/cmd.
        let kind = sysinfo::ProcessRefreshKind::everything();
        sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::Some(&[pid_sys]),
            true,
            kind,
        );
        let proc_ = sys.process(pid_sys)?;
        Some(process_to_info(proc_))
    }

    fn process_tree(&self, pid: u32) -> Vec<u32> {
        let sys = self.sys.lock().ok();
        let sys = match sys {
            Some(s) => s,
            None => return vec![pid],
        };
        let mut chain = Vec::new();
        let mut current = sysinfo::Pid::from_u32(pid);
        while let Some(proc_) = sys.process(current) {
            chain.push(current.as_u32());
            match proc_.parent() {
                Some(parent) if parent != current => current = parent,
                _ => break,
            }
            if chain.len() > 64 {
                break; // sanity guard against cycles
            }
        }
        chain.reverse();
        chain
    }
}

fn process_to_info(proc_: &sysinfo::Process) -> ProcessInfo {
    let cwd = proc_.cwd().map(|p| p.to_path_buf());
    let cwd_permission_denied = cwd.is_none();
    ProcessInfo {
        pid: proc_.pid().as_u32(),
        name: proc_.name().to_string_lossy().into_owned(),
        exe: proc_.exe().map(|p| p.to_path_buf()),
        cwd,
        cwd_permission_denied,
        cmdline: proc_
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy().into_owned())
            .collect(),
        parent_pid: proc_.parent().map(|p| p.as_u32()),
        start_time: proc_.start_time(),
        project: None,   // filled by project_assoc in Phase E polish
        dev_server: None, // filled by dev_server_detect in Phase E polish
        display_name: None,
    }
}
