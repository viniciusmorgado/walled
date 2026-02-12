use std::collections::HashSet;
use std::io;
use std::process::{Command, Stdio};

/// Returns the list of *privileged* (1‑1023) UDP ports that are
/// currently **listening** on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one privileged UDP port is in use.
///   * `Ok(None)`      – no privileged UDP ports are listening (empty set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero
///                status, or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all filtering in pure Rust.
pub fn privileged_udp_used() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-ulnH"])
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

    if used.is_empty() {
        return Ok(None);
    }

    let mut ports: Vec<u16> = used.into_iter().collect();
    ports.sort_unstable();
    Ok(Some(ports))
}

/// Returns the list of *unprivileged* (1024‑65535) UDP ports that are **not**
/// currently listening on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one unprivileged UDP port is free.
///   * `Ok(None)`      – every unprivileged UDP port (1024‑65535) is in use
///                        (empty free set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero
///                status, or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all set arithmetic in pure Rust.
pub fn unprivileged_udp_free() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-ulnH"])
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

/// Returns the list of *unprivileged* (1024‑65535) UDP ports that are
/// currently **listening** on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one unprivileged UDP port is in use.
///   * `Ok(None)`      – no unprivileged UDP ports are listening (empty set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero
///                status, or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all filtering in pure Rust.
pub fn unprivileged_udp_used() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-ulnH"])
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

/// Returns the list of *privileged* (1‑1023) UDP ports that are **not**
/// currently listening on the host.
///
/// Success variants:
///   * `Ok(Some(vec))` – at least one privileged UDP port is free.
///   * `Ok(None)`      – every privileged UDP port (1‑1023) is in use
///                        (empty free set).
///
/// Failure variant:
///   * `Err(e)` – the `ss` command could not be started, exited with a non‑zero
///                status, or its output could not be parsed.
///
/// The implementation avoids any shell pipelines; it invokes `ss` directly and
/// performs all set arithmetic in pure Rust.
pub fn privileged_udp_free() -> io::Result<Option<Vec<u16>>> {
    let output = Command::new("ss")
        .args(&["-ulnH"])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privileged_udp_used_test() {
        match privileged_udp_used() {
            Ok(Some(ports)) => {
                println!(
                    "Privileged UDP ports IN USE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("No privileged UDP ports (1‑1023) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }

    #[test]
    fn unprivileged_udp_used_test() {
        match unprivileged_udp_used() {
            Ok(Some(ports)) => {
                println!(
                    "Unprivileged UDP ports IN USE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("No unprivileged UDP ports (1024‑65535) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }

    #[test]
    fn privileged_udp_free_test() {
        match privileged_udp_free() {
            Ok(Some(ports)) => {
                println!(
                    "Privileged UDP ports FREE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("All privileged UDP ports (1‑1023) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }

    #[test]
    fn unprivileged_udp_free_test() {
        match unprivileged_udp_free() {
            Ok(Some(ports)) => {
                println!(
                    "Unprivileged UDP ports FREE ({} total): {:?}",
                    ports.len(),
                    ports
                );
            }
            Ok(None) => {
                println!("All unprivileged UDP ports (1024‑65535) are currently listening.");
            }
            Err(e) => {
                eprintln!("Failed to run `ss`: {}", e);
            }
        }
    }
}
