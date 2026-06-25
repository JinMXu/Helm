//! Windows process killer.
//!
//! Windows has no native SIGTERM equivalent. `sysinfo::Process::kill()`
//! calls `TerminateProcess` internally, which is an immediate forced kill.
//! Phase E will swap this for a job-object-based graceful shutdown if
//! we need to support SIGTERM-equivalent behavior (e.g. for services that
//! trap SIGTERM to clean up). For MVP, immediate termination is acceptable
//! since dev servers are short-lived and don't typically need graceful shutdown.

use crate::error::{Error, Result};
use crate::models::KillResult;
use crate::os::ProcessKiller;
use std::time::{Duration, Instant};

pub struct WindowsKiller;

impl ProcessKiller for WindowsKiller {
    fn kill_graceful(&self, pid: u32, timeout: Duration) -> Result<KillResult> {
        // Windows has no SIGTERM. We try to open the process and TerminateProcess it.
        // sysinfo::Process::kill() wraps this. The `timeout` is effectively ignored
        // on Windows for MVP — TerminateProcess is synchronous.
        let start = Instant::now();
        let mut sys = sysinfo::System::new();
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, false);
        let proc_ = sys
            .process(sysinfo::Pid::from_u32(pid))
            .ok_or(Error::ProcessNotFound(pid))?;

        if !proc_.kill() {
            return Err(Error::PermissionDenied(pid));
        }
        // Poll for exit (TerminateProcess is async at the OS level; the process
        // handle becomes signaled when it actually exits).
        let deadline = start + timeout;
        loop {
            sys.refresh_processes(sysinfo::ProcessesToUpdate::All, false);
            if sys.process(sysinfo::Pid::from_u32(pid)).is_none() {
                let elapsed_ms = start.elapsed().as_millis() as u64;
                return Ok(KillResult {
                    pid,
                    graceful: true, // best-effort label; Windows can't distinguish
                    elapsed_ms,
                });
            }
            if Instant::now() > deadline {
                return Ok(KillResult {
                    pid,
                    graceful: false,
                    elapsed_ms: start.elapsed().as_millis() as u64,
                });
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn kill_force(&self, pid: u32) -> Result<()> {
        let mut sys = sysinfo::System::new();
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, false);
        let proc_ = sys
            .process(sysinfo::Pid::from_u32(pid))
            .ok_or(Error::ProcessNotFound(pid))?;
        if proc_.kill() {
            Ok(())
        } else {
            Err(Error::PermissionDenied(pid))
        }
    }
}
