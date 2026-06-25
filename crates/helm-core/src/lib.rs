//! Helm core library — cross-platform port and process management.
//!
//! Shared logic for the `helm-cli` and `helm-tauri` frontends.

pub mod dev_server;
pub mod dev_server_detect;
pub mod error;
pub mod free_port;
pub mod git_info;
pub mod killer;
pub mod models;
pub mod os;
pub mod project_assoc;
pub mod scanner;

pub use dev_server::{classify, Classification};
pub use error::{Error, Result};
pub use free_port::find_free;
pub use git_info::{find_git_root, read_git_info, GitInfo};
pub use killer::{kill_force, kill_graceful, kill_port};
pub use models::*;
pub use os::{services, ProcessInfoProvider, ProcessKiller, PortScanner, Services};
pub use scanner::{find_port_info, scan_all};
