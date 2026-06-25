//! Top-level kill wrapper. Delegates to the platform `ProcessKiller`.

use crate::error::Result;
use crate::models::KillResult;
use crate::os::Services;
use std::time::Duration;

pub fn kill_graceful(services: &Services, pid: u32, timeout: Duration) -> Result<KillResult> {
    services.killer.kill_graceful(pid, timeout)
}

pub fn kill_force(services: &Services, pid: u32) -> Result<()> {
    services.killer.kill_force(pid)
}

/// Convenience: look up the PID owning a port, then kill it.
pub fn kill_port(
    services: &Services,
    port: u16,
    force: bool,
    timeout: Duration,
) -> Result<KillResult> {
    let Some(proc_) = crate::scanner::find_port_info(services, port)? else {
        return Err(crate::error::Error::ProcessNotFound(port as u32));
    };
    if force {
        kill_force(services, proc_.pid)?;
    } else {
        kill_graceful(services, proc_.pid, timeout)?;
    }
    Ok(KillResult {
        pid: proc_.pid,
        graceful: !force,
        elapsed_ms: 0,
    })
}
