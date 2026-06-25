//! Error types for helm-core.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Process not found: pid {0}")]
    ProcessNotFound(u32),

    #[error("Permission denied killing pid {0}")]
    PermissionDenied(u32),

    #[error("Kill timeout exceeded for pid {0} (waited {1:?})")]
    KillTimeout(u32, std::time::Duration),

    #[error("Scan error: {0}")]
    Scan(String),
}
