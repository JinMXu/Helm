//! Tauri commands — the RPC surface that the frontend invokes.
//!
//! All commands are async to avoid blocking the Tauri runtime. Each delegates
//! to `helm-core` via the shared `AppState`.

use crate::state::AppState;
use helm_core::models::{KillResult, PortEntry, ProcessInfo};
use helm_core::GitInfo;
use std::path::PathBuf;
use std::time::Duration;
use tauri::State;

#[tauri::command]
pub async fn list_ports(_state: State<'_, AppState>) -> Result<Vec<PortEntry>, String> {
    let services = helm_core::services();
    helm_core::scan_all(&services).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn port_info(_state: State<'_, AppState>, port: u16) -> Result<Option<ProcessInfo>, String> {
    let services = helm_core::services();
    helm_core::find_port_info(&services, port).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_tree(_state: State<'_, AppState>, port: u16) -> Result<Vec<u32>, String> {
    let services = helm_core::services();
    match helm_core::find_port_info(&services, port) {
        Ok(Some(proc_)) => Ok(services.process.process_tree(proc_.pid)),
        Ok(None) => Err(format!("no process on port {port}")),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn kill_port(
    _state: State<'_, AppState>,
    port: u16,
    force: bool,
) -> Result<KillResult, String> {
    let services = helm_core::services();
    let timeout = Duration::from_secs(3);
    helm_core::kill_port(&services, port, force, timeout).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_free_port(prefer: Vec<u16>) -> Result<u16, String> {
    helm_core::find_free(&prefer, 3000..=9999).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_in_browser(port: u16) -> Result<(), String> {
    let url = format!("http://localhost:{port}");
    open::that(&url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_git_info(path: String) -> Result<Option<GitInfo>, String> {
    Ok(helm_core::read_git_info(&PathBuf::from(path)))
}
