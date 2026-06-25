//! Project association — map a process's CWD to a project root.
//!
//! Walks up from the CWD looking for known project manifest files
//! (package.json, Cargo.toml, pyproject.toml, go.mod, pom.xml, build.gradle,
//! *.csproj). The directory containing the manifest is the project root.

use crate::models::ProjectInfo;
use std::path::{Path, PathBuf};

const MANIFEST_FILES: &[&str] = &[
    "package.json",
    "Cargo.toml",
    "pyproject.toml",
    "go.mod",
    "pom.xml",
    "build.gradle",
    "build.gradle.kts",
];

/// Find the project root by walking up from `cwd` looking for a manifest.
/// Returns `None` if no manifest is found before hitting the filesystem root.
pub fn find_project_root(cwd: &Path) -> Option<PathBuf> {
    let mut current = Some(cwd);
    while let Some(dir) = current {
        for manifest in MANIFEST_FILES {
            let candidate = dir.join(manifest);
            if candidate.is_file() {
                return Some(dir.to_path_buf());
            }
        }
        // Also check for *.csproj (any name)
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.ends_with(".csproj") {
                    return Some(dir.to_path_buf());
                }
            }
        }
        current = dir.parent();
    }
    None
}

/// Build a `ProjectInfo` from a project root path.
/// `name` comes from package.json's `name` field if available, otherwise the directory name.
/// `icon` is a per-language emoji.
pub fn project_info(root: &Path) -> ProjectInfo {
    let name = read_project_name(root).unwrap_or_else(|| {
        root.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into())
    });
    let icon = detect_icon(root);
    ProjectInfo {
        root: root.to_path_buf(),
        name,
        icon,
    }
}

fn read_project_name(root: &Path) -> Option<String> {
    let pkg = root.join("package.json");
    if pkg.is_file() {
        if let Ok(content) = std::fs::read_to_string(&pkg) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(name) = json.get("name").and_then(|v| v.as_str()) {
                    return Some(name.to_string());
                }
            }
        }
    }
    let cargo = root.join("Cargo.toml");
    if cargo.is_file() {
        if let Ok(content) = std::fs::read_to_string(&cargo) {
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("name") {
                    let rest = rest.trim_start_matches(['=', ' ', '"']);
                    if let Some(end) = rest.find('"') {
                        return Some(rest[..end].to_string());
                    }
                }
            }
        }
    }
    None
}

fn detect_icon(root: &Path) -> Option<String> {
    if root.join("package.json").is_file() {
        return Some("node".into());
    }
    if root.join("Cargo.toml").is_file() {
        return Some("rust".into());
    }
    if root.join("pyproject.toml").is_file() || root.join("requirements.txt").is_file() {
        return Some("python".into());
    }
    if root.join("go.mod").is_file() {
        return Some("go".into());
    }
    if root.join("pom.xml").is_file() || root.join("build.gradle").is_file() {
        return Some("java".into());
    }
    if std::fs::read_dir(root)
        .ok()
        .and_then(|entries| entries.flatten().find(|e| e.file_name().to_string_lossy().ends_with(".csproj")))
        .is_some()
    {
        return Some("dotnet".into());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("create temp dir")
    }

    #[test]
    fn finds_package_json_in_cwd() {
        let dir = temp_dir();
        fs::write(dir.path().join("package.json"), r#"{"name": "my-app"}"#).unwrap();
        let root = find_project_root(dir.path()).unwrap();
        assert_eq!(root, dir.path());
        let info = project_info(&root);
        assert_eq!(info.name, "my-app");
        assert_eq!(info.icon.as_deref(), Some("node"));
    }

    #[test]
    fn finds_cargo_toml_in_parent() {
        let dir = temp_dir();
        let subdir = dir.path().join("src").join("deep");
        fs::create_dir_all(&subdir).unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            "[package]\nname = \"my-crate\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        let root = find_project_root(&subdir).unwrap();
        assert_eq!(root, dir.path());
        let info = project_info(&root);
        assert_eq!(info.name, "my-crate");
        assert_eq!(info.icon.as_deref(), Some("rust"));
    }

    #[test]
    fn finds_go_mod() {
        let dir = temp_dir();
        fs::write(dir.path().join("go.mod"), "module example.com/foo\n").unwrap();
        let info = project_info(dir.path());
        assert_eq!(info.icon.as_deref(), Some("go"));
        // go.mod name parsing is not implemented; falls back to dir name.
        assert!(!info.name.is_empty());
    }

    #[test]
    fn returns_none_when_no_manifest() {
        let dir = temp_dir();
        assert!(find_project_root(dir.path()).is_none());
    }

    #[test]
    fn finds_csproj_with_any_name() {
        let dir = temp_dir();
        fs::write(dir.path().join("MyApp.csproj"), "<Project></Project>").unwrap();
        let info = project_info(dir.path());
        assert_eq!(info.icon.as_deref(), Some("dotnet"));
    }
}
