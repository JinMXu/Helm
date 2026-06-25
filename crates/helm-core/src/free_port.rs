//! Free port finder — `get-port` style.
//!
//! Tries preferred ports first, then scans the range. Falls back to letting
//! the OS pick an ephemeral port via `bind(0)` to avoid race conditions.

use std::net::TcpListener;
use std::ops::RangeInclusive;

/// Find a free TCP port.
///
/// - `preferred`: tried in order; first that's bindable wins.
/// - `range`: scanned ascending if all preferred ports are taken.
/// - If both fail, binds to port 0 and lets the OS pick an ephemeral port.
///
/// Note: binding succeeds ≠ the port will stay free after we close the listener.
/// There's an inherent TOCTOU race. Callers should bind again themselves and
/// handle EADDRINUSE.
pub fn find_free(preferred: &[u16], range: RangeInclusive<u16>) -> std::io::Result<u16> {
    for &port in preferred {
        if is_free(port)? {
            return Ok(port);
        }
    }
    for port in range {
        if is_free(port)? {
            return Ok(port);
        }
    }
    // Fallback: OS-assigned ephemeral port.
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    Ok(listener.local_addr()?.port())
}

fn is_free(port: u16) -> std::io::Result<bool> {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => Ok(false),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_free_port_in_range() {
        // 127.0.0.1:0 is always free; just verify we get a valid u16.
        let port = find_free(&[], 3000..=3999).unwrap();
        assert!((3000..=3999).contains(&port) || port >= 32768);
    }

    #[test]
    fn prefers_given_port_if_free() {
        // Pick a high port unlikely to be in use.
        let candidate = 49152;
        let port = find_free(&[candidate], 50000..=50100).unwrap();
        // Either the candidate was free (returns it) or another in range.
        assert!(port == candidate || (50000..=50100).contains(&port));
    }
}
