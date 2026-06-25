use clap::{Parser, Subcommand};
use helm_core::{find_free, scan_all, services};
use std::ops::RangeInclusive;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "helm", version, about = "Manage local dev ports")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all listening and established ports.
    List {
        #[arg(long)]
        json: bool,
        /// Filter by port, process name, or cmdline substring.
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Show details for a specific port.
    Info { port: u16 },
    /// Show the parent process chain for the process owning a port.
    Tree { port: u16 },
    /// Kill the process listening on a port.
    Kill {
        port: u16,
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(long, default_value = "3s")]
        timeout: String,
    },
    /// Find a free TCP port.
    Free {
        /// Comma-separated preferred ports tried first.
        #[arg(short, long, value_delimiter = ',')]
        prefer: Vec<u16>,
        #[arg(long, default_value = "3000")]
        min: u16,
        #[arg(long, default_value = "9999")]
        max: u16,
    },
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Command::List { json, filter } => {
            let services = services();
            let entries = match scan_all(&services) {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            };
            let filtered: Vec<_> = match &filter {
                None => entries,
                Some(f) => entries
                    .into_iter()
                    .filter(|e| matches_filter(e, f))
                    .collect(),
            };
            if json {
                println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
            } else {
                print_table(&filtered);
            }
        }
        Command::Info { port } => {
            let services = services();
            match helm_core::find_port_info(&services, port) {
                Ok(Some(p)) => println!("{}", serde_json::to_string_pretty(&p).unwrap()),
                Ok(None) => {
                    eprintln!("no process found on port {port}");
                    std::process::exit(1);
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            }
        }
        Command::Tree { port } => {
            let services = services();
            match helm_core::find_port_info(&services, port) {
                Ok(Some(p)) => {
                    let chain = services.process.process_tree(p.pid);
                    print!("process tree for port {} (pid {}):\n", port, p.pid);
                    for (i, pid) in chain.iter().enumerate() {
                        let indent = "  ".repeat(i);
                        let label = services
                            .process
                            .by_pid(*pid)
                            .map(|p| p.name)
                            .unwrap_or_else(|| "<unknown>".into());
                        println!("{indent}└─ pid {pid}: {label}");
                    }
                }
                Ok(None) => {
                    eprintln!("no process found on port {port}");
                    std::process::exit(1);
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            }
        }
        Command::Kill { port, force, timeout } => {
            let services = services();
            let timeout = parse_duration(&timeout).unwrap_or(Duration::from_secs(3));
            match helm_core::kill_port(&services, port, force, timeout) {
                Ok(result) => {
                    println!(
                        "killed pid {} on port {} ({})",
                        result.pid,
                        port,
                        if result.graceful { "graceful" } else { "forced" }
                    );
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            }
        }
        Command::Free { prefer, min, max } => {
            let range: RangeInclusive<u16> = min..=max;
            match find_free(&prefer, range) {
                Ok(port) => println!("{port}"),
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn matches_filter(e: &helm_core::PortEntry, needle: &str) -> bool {
    let n = needle.to_lowercase();
    if e.port.to_string().contains(&n) {
        return true;
    }
    match &e.process {
        Some(p) => {
            if p.name.to_lowercase().contains(&n) {
                return true;
            }
            p.cmdline
                .iter()
                .any(|c| c.to_lowercase().contains(&n))
        }
        None => false,
    }
}

fn print_table(entries: &[helm_core::PortEntry]) {
    if entries.is_empty() {
        println!("(no ports)");
        return;
    }
    println!(
        "{:<7} {:<6} {:<12} {:<8} {:<20} {}",
        "PORT", "PROTO", "STATE", "PID", "PROCESS", "CMDLINE"
    );
    for e in entries {
        let proto = match e.protocol {
            helm_core::Protocol::Tcp => "tcp",
            helm_core::Protocol::Tcp6 => "tcp6",
            helm_core::Protocol::Udp => "udp",
            helm_core::Protocol::Udp6 => "udp6",
        };
        let state = format!("{:?}", e.state).to_lowercase();
        let (pid, name, cmdline) = match &e.process {
            Some(p) => (p.pid.to_string(), p.name.clone(), p.cmdline.join(" ")),
            None => (e.pid.to_string(), "—".into(), String::new()),
        };
        let cmdline_display: String = cmdline.chars().take(60).collect();
        println!(
            "{:<7} {:<6} {:<12} {:<8} {:<20} {}",
            e.port,
            proto,
            state,
            pid,
            truncate(&name, 20),
            cmdline_display
        );
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n - 1).collect();
        out.push('…');
        out
    }
}

fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim();
    if let Some(n) = s.strip_suffix("ms") {
        return n.parse().ok().map(Duration::from_millis);
    }
    if let Some(n) = s.strip_suffix('s') {
        return n.parse().ok().map(Duration::from_secs);
    }
    s.parse().ok().map(Duration::from_secs)
}
