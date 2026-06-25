//! Shared application state.
//!
//! Currently minimal — each command creates its own `Services` bundle on demand.
//! The background scan loop uses `scan_all()` here. If we later want to share
//! a single `sysinfo::System` instance across calls (to avoid re-scanning the
//! process table), this is where the mutex would live.

use helm_core::models::PortEntry;

pub struct AppState;

impl AppState {
    pub fn new() -> Self {
        Self
    }

    /// Scan all ports — used by the background loop in main.rs.
    pub fn scan_all(&self) -> Vec<PortEntry> {
        let services = helm_core::services();
        helm_core::scan_all(&services).unwrap_or_default()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
