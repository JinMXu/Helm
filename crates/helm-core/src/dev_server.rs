//! Dev server classification rules.
//!
//! Implements the deterministic rules for identifying development servers from
//! port scan results: TCP LISTEN + userland port range + (git repo or known runtime or Docker).

use crate::models::{PortEntry, PortState, ProcessInfo};
use std::path::Path;

/// Process names that indicate a development runtime.
const DEV_RUNTIME_NAMES: &[&str] = &[
    "node", "python", "python3", "go", "java", "ruby", "bun", "deno",
    "elixir", "erl", "php", "gunicorn", "uvicorn", "puma", "mix", "air",
    "beam.smp", "reflex",
];

/// Directory names that are "meaningless" for CWD-based naming.
const IGNORED_DIR_NAMES: &[&str] = &[
    "", "/", "_build", "build", "tmp", "dist", "deps",
];

/// Check if an entry qualifies as a development server.
///
/// Rules (in order):
/// 1. Must be TCP LISTEN, port in 1024..49152
/// 2. In a git repo → dev server (set display name to repo root dir name)
/// 3. Process name in known runtime whitelist → dev server
/// 4. Is a Docker process → dev server
/// 5. CWD directory name is in a meaningful whitelist → dev server
pub fn classify(entry: &PortEntry, git_root: Option<&Path>) -> Classification {
    // Rule 1: TCP LISTEN + userland port range
    if entry.state != PortState::Listening || entry.port < 1024 || entry.port >= 49152 {
        return Classification::NotDev;
    }

    let info = match &entry.process {
        Some(p) => p,
        None => return Classification::NotDev,
    };

    let is_docker = is_docker_process(&info.name);

    // Rule 2: Git repo — strongest signal
    if let Some(root) = git_root {
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        return Classification::Dev { display_name: name };
    }

    // Rule 3: Known runtime process name
    if is_known_runtime(&info.name) {
        let display_name = display_name_for_runtime(info, is_docker);
        return Classification::Dev { display_name };
    }

    // Rule 4: Docker process
    if is_docker {
        return Classification::Dev { display_name: "Docker".into() };
    }

    // Rule 5: CWD directory name is meaningful
    if let Some(cwd) = &info.cwd {
        if let Some(dir_name) = cwd.file_name().and_then(|n| n.to_str()) {
            if !dir_name.starts_with('.') && !IGNORED_DIR_NAMES.contains(&dir_name) {
                if is_known_runtime(dir_name) {
                    return Classification::Dev { display_name: dir_name.to_string() };
                }
            }
        }
    }

    Classification::NotDev
}

pub enum Classification {
    Dev { display_name: String },
    NotDev,
}

fn is_docker_process(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("docker") || lower.starts_with("com.dock") || lower.starts_with("vpnkit")
}

fn is_known_runtime(name: &str) -> bool {
    let lower = name.to_lowercase();
    if DEV_RUNTIME_NAMES.iter().any(|r| lower == *r) {
        return true;
    }
    // Python version variants: python3.12, python3.11, etc.
    if lower.starts_with("python") {
        let rest = &lower[6..];
        if rest.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return true;
        }
    }
    false
}

fn display_name_for_runtime(info: &ProcessInfo, is_docker: bool) -> String {
    // Priority: docker label > CWD dir name > process name
    if is_docker {
        return "Docker".into();
    }
    if let Some(cwd) = &info.cwd {
        if let Some(dir_name) = cwd.file_name().and_then(|n| n.to_str()) {
            if !dir_name.starts_with('.') && !IGNORED_DIR_NAMES.contains(&dir_name) {
                return dir_name.to_string();
            }
        }
    }
    info.name.clone()
}
