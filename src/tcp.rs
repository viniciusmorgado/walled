use std::collections::HashSet;
use std::io;
use std::process::{Command, Stdio};

/// Returns the list of *privileged* (1‑1023) TCP ports that are currently
/// **listening** on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one port was found.
///   * `Ok(None)`      – the command ran fine but no privileged TCP ports are listening (empty set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero status,
///                or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all filtering in Rust.
pub fn privileged_tcp_used() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-tlnH"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`ss` exited with status {}", output.status),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut used_ports = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let local = parts[3];
        if let Some(port_str) = local.rsplit(':').next() {
            if let Ok(port) = port_str.parse::<u16>() {
                if (1..=1023).contains(&port) {
                    used_ports.push(port);
                }
            }
        }
    }

    if used_ports.is_empty() {
        Ok(None)
    } else {
        used_ports.sort_unstable();
        used_ports.dedup();
        Ok(Some(used_ports))
    }
}

/// Returns the list of *privileged* (1‑1023) TCP ports that are **not**
/// currently listening on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one free privileged TCP port was found.
///   * `Ok(None)`      – every privileged TCP port (1‑1023) is in use (empty free set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero status,
///                or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all set arithmetic in pure Rust.
pub fn privileged_tcp_free() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-tlnH"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`ss` exited with status {}", output.status),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut used = HashSet::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let local = parts[3];
        if let Some(port_str) = local.rsplit(':').next() {
            if let Ok(port) = port_str.parse::<u16>() {
                if (1..=1023).contains(&port) {
                    used.insert(port);
                }
            }
        }
    }

    let mut free_ports = Vec::new();
    for port in 1u16..=1023 {
        if !used.contains(&port) {
            free_ports.push(port);
        }
    }

    if free_ports.is_empty() {
        Ok(None)
    } else {
        Ok(Some(free_ports))
    }
}

/// Returns the list of *unprivileged* (1024‑65535) TCP ports that are
/// currently **listening** on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one unprivileged TCP port is in use.
///   * `Ok(None)`      – no unprivileged TCP ports are listening (empty set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero
///                status, or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all filtering in pure Rust.
pub fn unprivileged_tcp_used() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-tlnH"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`ss` exited with status {}", output.status),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut used = HashSet::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let local = parts[3];
        if let Some(port_str) = local.rsplit(':').next() {
            if let Ok(port) = port_str.parse::<u16>() {
                if (1024..=65535).contains(&port) {
                    used.insert(port);
                }
            }
        }
    }

    if used.is_empty() {
        return Ok(None);
    }

    let mut ports: Vec<u16> = used.into_iter().collect();
    ports.sort_unstable();
    Ok(Some(ports))
}

/// Returns the list of *unprivileged* (1024‑65535) TCP ports that are **not**
/// currently listening on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one free unprivileged TCP port was found.
///   * `Ok(None)`      – every unprivileged TCP port (1024‑65535) is in use
///                        (empty free set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero
///                status, or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all set arithmetic in pure Rust.
pub fn unprivileged_tcp_free() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-tlnH"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`ss` exited with status {}", output.status),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut used = HashSet::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let local = parts[3];
        if let Some(port_str) = local.rsplit(':').next() {
            if let Ok(port) = port_str.parse::<u16>() {
                if (1024..=65535).contains(&port) {
                    used.insert(port);
                }
            }
        }
    }

    let mut free_ports = Vec::new();
    for port in 1024u16..=65535 {
        if !used.contains(&port) {
            free_ports.push(port);
        }
    }

    if free_ports.is_empty() {
        Ok(None)
    } else {
        Ok(Some(free_ports))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unprivileged_tcp_free_test() {
        match unprivileged_tcp_free() {
            Ok(Some(ports)) => {
                println!(
                    "Unprivileged TCP ports FREE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("All unprivileged TCP ports (1024‑65535) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }

    #[test]
    fn unprivileged_tcp_used_test() {
        match unprivileged_tcp_used() {
            Ok(Some(ports)) => {
                println!(
                    "Unprivileged TCP ports IN USE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("No unprivileged TCP ports (1024‑65535) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }

    #[test]
    fn privileged_tcp_free_test() {
        match privileged_tcp_free() {
            Ok(Some(ports)) => {
                println!(
                    "Privileged TCP ports FREE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("All privileged TCP ports (1‑1023) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }

    #[test]
    fn privileged_tcp_used_test() {
        match privileged_tcp_used() {
            Ok(Some(ports)) => {
                println!("Privileged TCP ports in use ({} total): {:?}", ports.len(), ports);
            }
            Ok(None) => {
                println!("No privileged TCP ports are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }
}
