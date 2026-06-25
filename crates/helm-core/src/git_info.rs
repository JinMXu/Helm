use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git metadata for a project directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitInfo {
    /// Current branch name, e.g. "main", "feat/foo".
    pub branch: Option<String>,
    /// Repository name extracted from the origin remote URL.
    /// e.g. "helm" from "https://github.com/user/helm.git".
    pub repo: Option<String>,
}

/// Walk up from `start` to find the nearest `.git` directory root.
/// Returns the parent directory of `.git` (i.e., the repo root).
pub fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_dir() {
        start.to_path_buf()
    } else {
        start.parent()?.to_path_buf()
    };
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Some(current);
        }
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            return None;
        }
    }
}

/// Read git info from a directory. Returns `None` if git is not installed,
/// the directory is not a git repo, or any git command fails.
pub fn read_git_info(dir: &Path) -> Option<GitInfo> {
    let branch = git_cmd(dir, &["rev-parse", "--abbrev-ref", "HEAD"]);
    let repo = git_cmd(dir, &["remote", "get-url", "origin"])
        .and_then(extract_repo_name);
    if branch.is_some() || repo.is_some() {
        Some(GitInfo { branch, repo })
    } else {
        None
    }
}

fn git_cmd(dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(["-C"])
        .arg(dir)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_repo_name(url: String) -> Option<String> {
    // Handle: https://github.com/user/repo.git, git@github.com:user/repo.git
    let path = url
        .trim_end_matches('/')
        .trim_end_matches(".git");
    let name = path
        .rsplit('/')
        .next()
        .or_else(|| path.rsplit(':').next())?;
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}
