//! Unix process killer (macOS + Linux).
//!
//! Graceful: send SIGTERM, wait up to `timeout`, then SIGKILL.
//! Force: send SIGKILL immediately.

use crate::error::{Error, Result};
use crate::models::KillResult;
use crate::os::ProcessKiller;
use std::time::{Duration, Instant};

pub struct UnixKiller;

impl ProcessKiller for UnixKiller {
    fn kill_graceful(&self, pid: u32, timeout: Duration) -> Result<KillResult> {
        let start = Instant::now();
        let pid = nix::sys::signal::Pid::from_raw(pid as i32);

        // SIGTERM
        nix::sys::signal::kill(pid, Some(nix::sys::signal::Signal::SIGTERM))
            .map_err(|_| Error::PermissionDenied(pid.as_raw() as u32))?;

        // Poll for exit
        let deadline = start + timeout;
        loop {
            // kill(pid, 0) returns ESRCH if the process is gone.
            match nix::sys::signal::kill(pid, None) {
                Ok(_) => {}
                Err(nix::errno::Errno::ESRCH) => {
                    return Ok(KillResult {
                        pid: pid.as_raw() as u32,
                        graceful: true,
                        elapsed_ms: start.elapsed().as_millis() as u64,
                    });
                }
                Err(_) => return Err(Error::PermissionDenied(pid.as_raw() as u32)),
            }
            if Instant::now() > deadline {
                // Escalate to SIGKILL
                nix::sys::signal::kill(pid, Some(nix::sys::signal::Signal::SIGKILL))
                    .map_err(|_| Error::PermissionDenied(pid.as_raw() as u32))?;
                return Ok(KillResult {
                    pid: pid.as_raw() as u32,
                    graceful: false,
                    elapsed_ms: start.elapsed().as_millis() as u64,
                });
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn kill_force(&self, pid: u32) -> Result<()> {
        let pid = nix::sys::signal::Pid::from_raw(pid as i32);
        nix::sys::signal::kill(pid, Some(nix::sys::signal::Signal::SIGKILL))
            .map_err(|_| Error::PermissionDenied(pid.as_raw() as u32))
    }
}
