use anyhow::{Context, Result};
use pnet::datalink;
use std::net::IpAddr;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub is_up: bool,
    pub mac_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStatus {
    pub is_connected: bool,
    pub interface: Option<String>,
    pub ip_address: Option<String>,
    pub vpn_type: Option<String>,
}

/// Get all network interfaces
pub fn get_interfaces() -> Result<Vec<NetworkInterface>> {
    let interfaces = datalink::interfaces();
    let mut result = Vec::new();
    
    for iface in interfaces {
        let mut ip_addresses = Vec::new();
        
        for ip in &iface.ips {
            ip_addresses.push(ip.ip().to_string());
        }
        
        let mac_address = if let Some(mac) = iface.mac {
            Some(format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                mac.0, mac.1, mac.2, mac.3, mac.4, mac.5))
        } else {
            None
        };
        
        result.push(NetworkInterface {
            name: iface.name.clone(),
            ip_addresses,
            is_up: iface.is_up(),
            mac_address,
        });
    }
    
    Ok(result)
}

/// Get primary network interface (usually with default route)
pub fn get_primary_interface() -> Result<NetworkInterface> {
    let interfaces = get_interfaces()?;
    
    // Try to find interface with IP address that's not loopback
    for iface in interfaces {
        if iface.name != "lo" && !iface.ip_addresses.is_empty() && iface.is_up {
            // Filter out link-local addresses
            for ip_str in &iface.ip_addresses {
                if let Ok(ip) = ip_str.parse::<IpAddr>() {
                    match ip {
                        IpAddr::V4(v4) if !v4.is_loopback() && !v4.is_link_local() => {
                            return Ok(iface.clone());
                        }
                        IpAddr::V6(v6) if !v6.is_loopback() => {
                            return Ok(iface.clone());
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("No suitable network interface found"))
}

/// Detect VPN connection status
pub fn get_vpn_status() -> Result<VpnStatus> {
    let interfaces = get_interfaces()?;
    
    // Common VPN interface names
    let vpn_patterns = vec!["tun", "tap", "wg", "ppp", "vpn"];
    
    for iface in interfaces {
        for pattern in &vpn_patterns {
            if iface.name.contains(pattern) && iface.is_up {
                let ip_address = iface.ip_addresses.first().cloned();
                let vpn_type = if iface.name.starts_with("wg") {
                    Some("WireGuard".to_string())
                } else if iface.name.starts_with("tun") || iface.name.starts_with("tap") {
                    Some("OpenVPN/Generic".to_string())
                } else {
                    Some("Unknown".to_string())
                };
                
                return Ok(VpnStatus {
                    is_connected: true,
                    interface: Some(iface.name),
                    ip_address,
                    vpn_type,
                });
            }
        }
    }
    
    Ok(VpnStatus {
        is_connected: false,
        interface: None,
        ip_address: None,
        vpn_type: None,
    })
}

/// Get open ports using netstat or ss
pub fn get_open_ports() -> Result<Vec<(String, u16, String)>> {
    let output = Command::new("ss")
        .args(&["-tuln"])
        .output()
        .context("Failed to execute ss command")?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut ports = Vec::new();
    
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let proto = parts[0].to_string();
            let local_addr = parts[4];
            
            if let Some(port_str) = local_addr.rsplit(':').next() {
                if let Ok(port) = port_str.parse::<u16>() {
                    let state = if parts.len() > 5 {
                        parts[1].to_string()
                    } else {
                        "LISTEN".to_string()
                    };
                    ports.push((proto, port, state));
                }
            }
        }
    }
    
    Ok(ports)
}

/// Get active network peers
pub fn get_network_peers() -> Result<Vec<NetworkPeer>> {
    let output = Command::new("ip")
        .args(&["neigh", "show"])
        .output()
        .context("Failed to execute ip neigh command")?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut peers = Vec::new();
    
    for line in output_str.lines() {
        if let Some(peer) = parse_neighbor_line(line) {
            peers.push(peer);
        }
    }
    
    Ok(peers)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPeer {
    pub ip: String,
    pub mac: Option<String>,
    pub interface: String,
    pub state: String,
    pub device_type: String,
    pub os_guess: String,
}

fn parse_neighbor_line(line: &str) -> Option<NetworkPeer> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 4 {
        return None;
    }
    
    let ip = parts[0].to_string();
    let interface = parts[2].to_string();
    
    let mut mac = None;
    let mut state = "UNKNOWN".to_string();
    
    for i in 0..parts.len() {
        if parts[i] == "lladdr" && i + 1 < parts.len() {
            mac = Some(parts[i + 1].to_string());
        }
        if parts[i] == "REACHABLE" || parts[i] == "STALE" || parts[i] == "DELAY" {
            state = parts[i].to_string();
        }
    }
    
    let (device_type, os_guess) = guess_device_from_mac(&mac);
    
    Some(NetworkPeer {
        ip,
        mac,
        interface,
        state,
        device_type,
        os_guess,
    })
}

fn guess_device_from_mac(mac: &Option<String>) -> (String, String) {
    if let Some(mac_addr) = mac {
        let oui = mac_addr.split(':').take(3).collect::<Vec<_>>().join(":");
        
        // Simple OUI-based device detection
        match oui.to_lowercase().as_str() {
            s if s.starts_with("00:50:56") || s.starts_with("00:0c:29") => {
                ("Virtual Machine".to_string(), "VMware".to_string())
            }
            s if s.starts_with("08:00:27") => {
                ("Virtual Machine".to_string(), "VirtualBox".to_string())
            }
            s if s.starts_with("dc:a6:32") || s.starts_with("b8:27:eb") => {
                ("IoT Device".to_string(), "Raspberry Pi".to_string())
            }
            s if s.starts_with("00:1b:63") => {
                ("Computer".to_string(), "Apple".to_string())
            }
            _ => ("Unknown".to_string(), "Unknown".to_string())
        }
    } else {
        ("Unknown".to_string(), "Unknown".to_string())
    }
}

/// Get network statistics from /proc/net/dev
pub fn get_network_stats() -> Result<String> {
    let stats = std::fs::read_to_string("/proc/net/dev")
        .context("Failed to read /proc/net/dev")?;
    Ok(stats)
}
