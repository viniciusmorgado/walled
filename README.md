# walled

`walled` is a Rust crate designed to provide programmatic access to information about network port usage on Linux systems. It allows you to query which TCP and UDP ports, both privileged (1-1023) and unprivileged (1024-65535), are currently in use or are free.

The library achieves this by internally executing the `ss` command-line utility, parsing its output, and presenting the information through a simple Rust API. It avoids shell pipelines for robustness and performs all filtering and set arithmetic in pure Rust.

## Features

-   **Privileged TCP Ports:**
    -   `privileged_tcp_used()`: Lists all privileged TCP ports (1-1023) currently listening.
    -   `privileged_tcp_free()`: Lists all privileged TCP ports (1-1-1023) not currently listening.
-   **Unprivileged TCP Ports:**
    -   `unprivileged_tcp_used()`: Lists all unprivileged TCP ports (1024-65535) currently listening.
    -   `unprivileged_tcp_free()`: Lists all unprivileged TCP ports (1024-65535) not currently listening.
-   **Privileged UDP Ports:**
    -   `privileged_udp_used()`: Lists all privileged UDP ports (1-1023) currently listening.
    -   `privileged_udp_free()`: Lists all privileged UDP ports (1-1023) not currently listening.
-   **Unprivileged UDP Ports:**
    -   `unprivileged_udp_used()`: Lists all unprivileged UDP ports (1024-65535) currently listening.
    -   `unprivileged_udp_free()`: Lists all unprivileged UDP ports (1024-65535) not currently listening.

## Usage

### Add to your `Cargo.toml`

To use `walled` in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
walled = "0.1.0" # Use the latest version available
```

### Examples

Here's how you can use `walled` functions in your Rust code:

```rust
use walled::{
    privileged_tcp_used, privileged_tcp_free,
    unprivileged_tcp_used, unprivileged_tcp_free,
    privileged_udp_used, privileged_udp_free,
    unprivileged_udp_used, unprivileged_udp_free,
};
use std::io;

fn main() -> io::Result<()> {
    println!("--- TCP Ports ---");

    match privileged_tcp_used()? {
        Some(ports) => println!("Privileged TCP ports in use: {:?}", ports),
        None => println!("No privileged TCP ports are currently listening."),
    }

    match privileged_tcp_free()? {
        Some(ports) => println!("Privileged TCP ports free: {:?}", ports),
        None => println!("All privileged TCP ports are currently listening."),
    }

    match unprivileged_tcp_used()? {
        Some(ports) => println!("Unprivileged TCP ports in use: {:?}", ports),
        None => println!("No unprivileged TCP ports are currently listening."),
    }

    match unprivileged_tcp_free()? {
        Some(ports) => println!("Unprivileged TCP ports free: {:?}", ports),
        None => println!("All unprivileged TCP ports are currently listening."),
    }

    println!("\n--- UDP Ports ---");

    match privileged_udp_used()? {
        Some(ports) => println!("Privileged UDP ports in use: {:?}", ports),
        None => println!("No privileged UDP ports are currently listening."),
    }

    match privileged_udp_free()? {
        Some(ports) => println!("Privileged UDP ports free: {:?}", ports),
        None => println!("All privileged UDP ports are currently listening."),
    }

    match unprivileged_udp_used()? {
        Some(ports) => println!("Unprivileged UDP ports in use: {:?}", ports),
        None => println!("No unprivileged UDP ports are currently listening."),
    }

    match unprivileged_udp_free()? {
        Some(ports) => println!("Unprivileged UDP ports free: {:?}", ports),
        None => println!("All unprivileged UDP ports are currently listening."),
    }

    Ok(())
}
```

## Requirements

The `walled` library relies on the `ss` utility being available and executable on your Linux system. `ss` is part of the `iproute2` package and is generally available on most modern Linux distributions. If you encounter errors related to `ss` not being found or not executing correctly, ensure `iproute2` is installed.

On Debian/Ubuntu:
```bash
sudo apt-get install iproute2
```

On Fedora/RHEL/CentOS:
```bash
sudo dnf install iproute2
# or for older versions
sudo yum install iproute2
```

## License

`walled` is licensed under the BSD 3-Clause License. See the `LICENSE` file for more details.
