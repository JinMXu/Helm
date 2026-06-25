//! Top-level scan orchestrator. Combines raw socket info with process metadata,
//! then enriches each process with dev-server and project association.

use crate::error::Result;
use crate::models::{PortEntry, PortState, ProcessInfo};
use crate::os::Services;

/// Scan all listening and established TCP ports, plus UDP ports.
/// Each `PortEntry` is enriched with `ProcessInfo` (and its `project` /
/// `dev_server` fields) if the PID is still alive.
pub fn scan_all(services: &Services) -> Result<Vec<PortEntry>> {
    let mut entries = Vec::new();

    for raw in services.scanner.scan_tcp()? {
        let process = enrich_process(services, raw.pid);
        let mut entry = PortEntry {
            port: raw.port,
            protocol: raw.protocol,
            local_address: raw.local_address,
            remote_address: raw.remote_address,
            remote_port: raw.remote_port,
            state: raw.state,
            pid: raw.pid,
            process,
        };
        classify_entry(&mut entry);
        entries.push(entry);
    }

    for raw in services.scanner.scan_udp()? {
        let process = enrich_process(services, raw.pid);
        let mut entry = PortEntry {
            port: raw.port,
            protocol: raw.protocol,
            local_address: raw.local_address,
            remote_address: raw.remote_address,
            remote_port: raw.remote_port,
            state: raw.state,
            pid: raw.pid,
            process,
        };
        classify_entry(&mut entry);
        entries.push(entry);
    }

    // Sort: listening first, then by port asc.
    entries.sort_by(|a, b| {
        let rank = |s: &PortState| match s {
            PortState::Listening => 0,
            PortState::Established => 1,
            _ => 2,
        };
        rank(&a.state)
            .cmp(&rank(&b.state))
            .then(a.port.cmp(&b.port))
    });

    Ok(entries)
}

/// Build `ProcessInfo` for a PID, then enrich with dev-server + project.
fn enrich_process(services: &Services, pid: u32) -> Option<ProcessInfo> {
    let mut info = services.process.by_pid(pid)?;
    if let Some(cwd) = info.cwd.as_ref() {
        info.project = crate::project_assoc::find_project_root(cwd)
            .map(|root| crate::project_assoc::project_info(&root));
    }
    info.dev_server = crate::dev_server_detect::detect(&info.name, &info.cmdline);
    Some(info)
}

/// Finalize dev-server classification on a full PortEntry.
pub fn classify_entry(entry: &mut PortEntry) {
    use crate::dev_server::Classification;

    let git_root = entry
        .process
        .as_ref()
        .and_then(|p| p.cwd.as_ref())
        .and_then(|cwd| crate::git_info::find_git_root(cwd));

    let result = crate::dev_server::classify(entry, git_root.as_deref());
    if let Classification::Dev { display_name } = result {
        if let Some(ref mut info) = entry.process {
            info.display_name = Some(display_name);
        }
    }
}

/// Find a port's process info. Returns None if the port is not in use
/// or the process can't be inspected.
pub fn find_port_info(services: &Services, port: u16) -> Result<Option<ProcessInfo>> {
    let entries = scan_all(services)?;
    Ok(entries
        .into_iter()
        .find(|e| e.port == port)
        .and_then(|e| e.process))
}
