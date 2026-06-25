//! Dev server framework recognition.
//!
//! Pattern-matches a process's name + cmdline against known dev server
//! signatures (vite, next, rails, uvicorn, etc.). Returns `None` for
//! unrecognized processes — they're still ports, just not dev servers.

use crate::models::DevServerKind;

/// Detect the dev server framework from process info.
///
/// `name` is the executable name (e.g. "node", "python.exe", "ruby").
/// `cmdline` is the full argument vector.
pub fn detect(name: &str, cmdline: &[String]) -> Option<DevServerKind> {
    let name_lower = name.to_lowercase();
    let cmdline_joined: String = cmdline
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");

    // Node / JS ecosystem — vite, next, nuxt, angular, astro, remix, etc.
    if name_lower.contains("node") || name_lower.contains("npm") || name_lower.contains("pnpm")
        || name_lower.contains("yarn") || name_lower.contains("bun")
    {
        if cmdline_joined.contains("vite") {
            return Some(DevServerKind::Vite);
        }
        if cmdline_joined.contains("next dev") || cmdline_joined.contains("next-server") {
            return Some(DevServerKind::Next);
        }
        if cmdline_joined.contains("nuxt dev") || cmdline_joined.contains("nuxt-server") {
            return Some(DevServerKind::Nuxt);
        }
        if cmdline_joined.contains("ng serve") || cmdline_joined.contains("angular") {
            return Some(DevServerKind::Other("angular".into()));
        }
        if cmdline_joined.contains("astro dev") {
            return Some(DevServerKind::Other("astro".into()));
        }
        if cmdline_joined.contains("remix dev") || cmdline_joined.contains("remix-serve") {
            return Some(DevServerKind::Other("remix".into()));
        }
        if cmdline_joined.contains("webpack-dev") || cmdline_joined.contains("webpack serve") {
            return Some(DevServerKind::WebpackDev);
        }
        if cmdline_joined.contains("nest start") {
            return Some(DevServerKind::Other("nest".into()));
        }
        if cmdline_joined.contains("storybook") || cmdline_joined.contains("start-storybook") {
            return Some(DevServerKind::Other("storybook".into()));
        }
        if cmdline_joined.contains("express") {
            return Some(DevServerKind::Express);
        }
    }

    // Deno — separate runtime
    if name_lower.contains("deno") {
        if cmdline_joined.contains("run") || cmdline_joined.contains("task") || cmdline_joined.contains("serve") {
            return Some(DevServerKind::Other("deno".into()));
        }
    }

    // Bun — separate runtime (also caught by JS ecosystem above)
    if name_lower == "bun" {
        return Some(DevServerKind::Other("bun".into()));
    }

    // Python ecosystem — uvicorn, gunicorn, flask, django
    if name_lower.contains("python") || name_lower.contains("uvicorn") || name_lower.contains("gunicorn") {
        if cmdline_joined.contains("uvicorn") {
            return Some(DevServerKind::Uvicorn);
        }
        if cmdline_joined.contains("gunicorn") {
            return Some(DevServerKind::Gunicorn);
        }
        if cmdline_joined.contains("flask") || cmdline_joined.contains("flask run") {
            return Some(DevServerKind::Flask);
        }
        if cmdline_joined.contains("manage.py runserver") || cmdline_joined.contains("django") {
            return Some(DevServerKind::Django);
        }
    }

    // Ruby — rails, puma
    if name_lower.contains("ruby") || name_lower.contains("rails") || name_lower.contains("puma") {
        if cmdline_joined.contains("rails server") || cmdline_joined.contains("rails s") {
            return Some(DevServerKind::Rails);
        }
        if cmdline_joined.contains("puma") {
            return Some(DevServerKind::Puma);
        }
    }

    // PHP — built-in dev server
    if name_lower.contains("php") {
        if cmdline_joined.contains("-s") && (cmdline_joined.contains("localhost") || cmdline_joined.contains("127.0.0.1")) {
            return Some(DevServerKind::Other("php-dev".into()));
        }
        if cmdline_joined.contains("artisan serve") {
            return Some(DevServerKind::Other("laravel".into()));
        }
    }

    // Java — gradle bootRun, spring-boot, mvn
    if name_lower.contains("java") || name_lower.contains("gradle") {
        if cmdline_joined.contains("bootrun") || cmdline_joined.contains("spring-boot") {
            return Some(DevServerKind::GradleBootRun);
        }
        if cmdline_joined.contains("tomcat") || cmdline_joined.contains("jetty") {
            return Some(DevServerKind::Other("java-server".into()));
        }
    }

    // .NET — dotnet watch run
    if name_lower.contains("dotnet") {
        if cmdline_joined.contains("watch") && cmdline_joined.contains("run") {
            return Some(DevServerKind::DotnetWatch);
        }
    }

    // Rust — cargo run, cargo watch
    if name_lower.contains("cargo") || name_lower.contains("rustc") {
        if cmdline_joined.contains("cargo run") || cmdline_joined.contains("cargo watch") {
            return Some(DevServerKind::CargoRun);
        }
    }

    // Go — go run, or compiled Go binary
    if name_lower == "go" || name_lower == "go.exe" || name_lower.starts_with("go_") {
        if cmdline_joined.contains("go run") || cmdline_joined.contains("run .") {
            return Some(DevServerKind::GoRun);
        }
    }

    // Hugo static site generator
    if name_lower.contains("hugo") {
        if cmdline_joined.contains("server") || cmdline_joined.contains("serve") {
            return Some(DevServerKind::Other("hugo".into()));
        }
    }

    // Wrangler (Cloudflare Workers)
    if name_lower.contains("wrangler") || name_lower.contains("workerd") {
        return Some(DevServerKind::Other("wrangler".into()));
    }

    None
}

/// Enrich a `ProcessInfo` in place by setting `dev_server` if detected.
pub fn enrich(name: &str, cmdline: &[String], dev_server: &mut Option<DevServerKind>) {
    if dev_server.is_none() {
        *dev_server = detect(name, cmdline);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|p| p.to_string()).collect()
    }

    #[test]
    fn detects_vite() {
        assert_eq!(
            detect("node", &s(&["node", "vite"])),
            Some(DevServerKind::Vite)
        );
        assert_eq!(
            detect("node.exe", &s(&["node", "vite", "dev"])),
            Some(DevServerKind::Vite)
        );
    }

    #[test]
    fn detects_next() {
        assert_eq!(
            detect("node", &s(&["next", "dev"])),
            Some(DevServerKind::Next)
        );
    }

    #[test]
    fn detects_angular() {
        assert!(matches!(
            detect("node", &s(&["ng", "serve"])),
            Some(DevServerKind::Other(ref s)) if s == "angular"
        ));
    }

    #[test]
    fn detects_astro() {
        assert!(matches!(
            detect("node", &s(&["astro", "dev"])),
            Some(DevServerKind::Other(ref s)) if s == "astro"
        ));
    }

    #[test]
    fn detects_rails() {
        assert_eq!(
            detect("ruby", &s(&["bin/rails", "server"])),
            Some(DevServerKind::Rails)
        );
    }

    #[test]
    fn detects_uvicorn() {
        assert_eq!(
            detect("python", &s(&["uvicorn", "main:app"])),
            Some(DevServerKind::Uvicorn)
        );
    }

    #[test]
    fn detects_dotnet_watch() {
        assert_eq!(
            detect("dotnet", &s(&["dotnet", "watch", "run"])),
            Some(DevServerKind::DotnetWatch)
        );
    }

    #[test]
    fn detects_cargo_run() {
        assert_eq!(
            detect("cargo", &s(&["cargo", "run"])),
            Some(DevServerKind::CargoRun)
        );
    }

    #[test]
    fn detects_php_dev() {
        assert!(matches!(
            detect("php", &s(&["php", "-S", "localhost:8000"])),
            Some(DevServerKind::Other(ref s)) if s == "php-dev"
        ));
    }

    #[test]
    fn detects_storybook() {
        assert!(matches!(
            detect("node", &s(&["start-storybook"])),
            Some(DevServerKind::Other(ref s)) if s == "storybook"
        ));
    }

    #[test]
    fn returns_none_for_unrelated_process() {
        assert_eq!(detect("explorer", &s(&["explorer.exe"])), None);
    }
}
