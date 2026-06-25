//! Data models for port and process information.
//!
//! All structs use `#[serde(rename_all = "camelCase")]` so the JSON shape
//! matches TypeScript interfaces in the frontend without manual renaming.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;

/// A single port entry — one row in the port table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortEntry {
    pub port: u16,
    pub protocol: Protocol,
    pub local_address: IpAddr,
    pub remote_address: Option<IpAddr>,
    pub remote_port: Option<u16>,
    pub state: PortState,
    pub pid: u32,
    /// None if the process has exited or we lack permission to inspect it.
    pub process: Option<ProcessInfo>,
}

/// Process owning a port. Enriched with project and dev-server metadata
/// by `project_assoc` and `dev_server_detect`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe: Option<PathBuf>,
    pub cwd: Option<PathBuf>,
    /// true on macOS/Linux when the process is owned by another user and
    /// we can't read its CWD. UI should show "—" with a tooltip rather
    /// than crash or hide the row.
    pub cwd_permission_denied: bool,
    pub cmdline: Vec<String>,
    pub parent_pid: Option<u32>,
    /// Unix epoch seconds.
    pub start_time: u64,
    pub project: Option<ProjectInfo>,
    pub dev_server: Option<DevServerKind>,
    /// Display name computed by dev-server classification rules.
    /// Present when the process qualifies as a development server.
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub root: PathBuf,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    Tcp,
    Tcp6,
    Udp,
    Udp6,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PortState {
    Listening,
    Established,
    TimeWait,
    CloseWait,
    FinWait1,
    FinWait2,
    Closed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DevServerKind {
    Vite,
    Next,
    Nuxt,
    Rails,
    Puma,
    Uvicorn,
    Gunicorn,
    Flask,
    Express,
    WebpackDev,
    GradleBootRun,
    DotnetWatch,
    CargoRun,
    GoRun,
    Django,
    /// Unrecognized dev server; carries the process name for display.
    Other(String),
}

/// Result of a kill operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillResult {
    pub pid: u32,
    /// true if the process exited after SIGTERM (or sysinfo's graceful kill).
    /// false if we had to escalate to SIGKILL / TerminateProcess.
    pub graceful: bool,
    /// Wall-clock time spent waiting for the process to exit, in milliseconds.
    pub elapsed_ms: u64,
}

/// Raw socket info returned by the OS-specific scanner.
/// Used internally by `scanner.rs` to build `PortEntry`.
#[derive(Debug, Clone)]
pub struct RawSocket {
    pub port: u16,
    pub protocol: Protocol,
    pub local_address: IpAddr,
    pub remote_address: Option<IpAddr>,
    pub remote_port: Option<u16>,
    pub state: PortState,
    pub pid: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_entry_serde_round_trip() {
        let entry = PortEntry {
            port: 3000,
            protocol: Protocol::Tcp,
            local_address: "127.0.0.1".parse().unwrap(),
            remote_address: None,
            remote_port: None,
            state: PortState::Listening,
            pid: 12345,
            process: Some(ProcessInfo {
                pid: 12345,
                name: "node".into(),
                exe: Some("/usr/bin/node".into()),
                cwd: Some("/home/user/proj".into()),
                cwd_permission_denied: false,
                cmdline: vec!["node".into(), "vite".into()],
                parent_pid: Some(1),
                start_time: 1700000000,
                project: Some(ProjectInfo {
                    root: "/home/user/proj".into(),
                    name: "my-app".into(),
                    icon: Some("node".into()),
                }),
                dev_server: Some(DevServerKind::Vite),
                display_name: None,
            }),
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"localAddress\":\"127.0.0.1\""), "json: {json}");
        assert!(json.contains("\"devServer\":\"vite\""), "json: {json}");
        let back: PortEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.port, 3000);
        assert_eq!(back.protocol, Protocol::Tcp);
        assert_eq!(back.process.as_ref().unwrap().dev_server, Some(DevServerKind::Vite));
    }

    #[test]
    fn dev_server_other_preserves_label() {
        let kind = DevServerKind::Other("custom-tool".into());
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, "{\"other\":\"custom-tool\"}");
        let back: DevServerKind = serde_json::from_str(&json).unwrap();
        match back {
            DevServerKind::Other(s) => assert_eq!(s, "custom-tool"),
            _ => panic!("expected Other variant"),
        }
    }

    #[test]
    fn port_state_camel_case() {
        assert_eq!(
            serde_json::to_string(&PortState::FinWait1).unwrap(),
            "\"finWait1\""
        );
        assert_eq!(
            serde_json::to_string(&PortState::TimeWait).unwrap(),
            "\"timeWait\""
        );
    }
}
