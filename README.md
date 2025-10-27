![alt text](https://media.discordapp.net/attachments/1055231169479843870/1086803851056783481/cywf_ultra_realistic_Network_Ninja_in_a_cyberpunk_neon_age_who__d9457b3c-feb9-4969-b7d1-4e5d5e1dac3e.png?width=1138&height=1138)

# NetNinja

Streamline Linux server troubleshooting with NetNinja - the ultimate tool for network and system admins. Now featuring a **fully immersive Rust CLI** with **tmux multiplexer** for real-time network monitoring!

## Features

### 🥷 Advanced Rust CLI
- **Immersive Monitoring Dashboard**: Real-time network monitoring with tmux multiplexer
- **Network Status Monitoring**: Live interface status, IP addresses, and MAC addresses
- **VPN Detection**: Automatic detection of VPN connections (WireGuard, OpenVPN, etc.)
- **Live Traffic Analysis**: Integration with iftop for real-time network traffic visualization
- **Open Ports Scanner**: Real-time monitoring of listening ports and connections
- **Network Peers Detection**: Discover active network devices with OS fingerprinting
- **Security Alerts Panel**: Monitor failed login attempts, unusual traffic, and firewall events
- **Robust Error Handling**: Built in Rust for reliability and performance

### 📊 Tmux Monitoring Dashboard
Launch an immersive 6-pane dashboard showing:
1. **Network Interface Status** - Live IP addresses and interface states
2. **VPN Status** - Connection status, IP, and VPN type
3. **Live iftop Data** - Real-time network traffic visualization
4. **Open Ports** - Listening services and connections
5. **Security Alerts** - Failed logins, firewall blocks, and security events
6. **Active Network Peers** - Connected devices with device type detection

## Installation

To install NetNinja, clone the repository and run the `install.sh` script:

```sh
git clone https://github.com/cywf/netninja.git
cd netninja
./install.sh
```

This will:
- Install required dependencies (tmux, iproute2, etc.)
- Build the Rust CLI application
- Create symbolic links to the executables in `/usr/local/bin`

### Prerequisites

- **Rust**: The installer will automatically install Rust if not present
- **tmux**: Required for the monitoring dashboard
- **Linux**: Designed for Linux systems (tested on Ubuntu/Debian)
- **Optional**: iftop for enhanced traffic monitoring (recommended)

## Usage

### Rust CLI Commands

#### Quick Status Summary
```sh
netninja-cli status
```
Shows a comprehensive overview of:
- Network interfaces and IP addresses
- VPN connection status
- Open ports and listening services
- Active network peers
- Security alerts and firewall status

#### Immersive Monitoring Dashboard
```sh
sudo netninja-cli monitor
```
Launches a full-screen tmux session with 6 panes showing real-time monitoring data.

**Note**: Use sudo for full functionality (iftop access, security logs, etc.)

**Tmux Controls**:
- `Ctrl+B` then `D` - Detach from session (keeps running in background)
- `tmux attach -t netninja-monitor` - Reattach to running session
- `Ctrl+B` then arrow keys - Navigate between panes
- `Ctrl+B` then `[` - Enter scroll mode (q to exit)

#### Help
```sh
netninja-cli --help
netninja-cli --version
```

### Legacy Bash Script

The original bash script is still available:

```sh
netninja --help
netninja --ping <ip>
netninja --dig <domain>
netninja --traceroute <ip>
netninja --netstat
```

## Making NetNinja Available System-wide

The install script automatically creates symbolic links in `/usr/local/bin`. To manually create them:

```sh
sudo ln -s /path/to/netninja/target/release/netninja-cli /usr/local/bin/netninja-cli
sudo ln -s /path/to/netninja/bin/netninja /usr/local/bin/netninja
```

Note: Replace `/path/to/netninja` with the actual path to your NetNinja directory.

## Technical Details

### Architecture
- **Language**: Rust 2021 edition for performance and safety
- **Async Runtime**: Tokio for efficient async operations
- **CLI Framework**: Clap for robust argument parsing
- **Network Monitoring**: pnet for low-level network interface access
- **UI Multiplexing**: tmux for multi-pane dashboard

### Dependencies
- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `pnet` - Network interface and packet manipulation
- `chrono` - Date and time handling
- `serde/serde_json` - Serialization
- `anyhow/thiserror` - Error handling
- `nix/libc` - Unix system calls

### Security Features
- Failed login detection (SSH monitoring)
- Unusual network traffic alerts
- Firewall status monitoring
- Port scan detection
- Security event logging

## Contributing

If you would like to contribute to NetNinja, please see the [CONTRIBUTING.md](https://github.com/cywf/netninja/docs/CONTRIBUTING.md) file for guidelines on how to contribute. We welcome bug reports, feature requests, and pull requests!

## License

NetNinja is licensed under the [MIT License](https://github.com/cywf/netninja/docs/LICENSE.txt).

## Troubleshooting

### iftop not available
Install iftop for enhanced traffic monitoring:
```sh
sudo apt-get install iftop
```

### Permission denied errors
Some monitoring features require elevated privileges:
```sh
sudo netninja-cli monitor
```

### tmux session already exists
Kill the existing session:
```sh
tmux kill-session -t netninja-monitor
```

### Rust not installed
The install script will install Rust automatically, or install manually:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
