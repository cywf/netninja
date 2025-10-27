use anyhow::Result;
use crate::{network, security, tmux};
use std::process::Command;

/// Launch the full tmux monitoring dashboard
pub async fn launch_dashboard() -> Result<()> {
    println!("🚀 Launching NetNinja Monitoring Dashboard...\n");
    
    // Create the monitor script
    tmux::create_monitor_script()?;
    
    // Execute the script
    let status = Command::new("bash")
        .arg("/tmp/netninja-monitor.sh")
        .status()?;
    
    if !status.success() {
        eprintln!("Failed to launch monitoring dashboard");
    }
    
    Ok(())
}

/// Show quick network status
pub async fn show_status() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("              🥷  NetNinja Status Report  🥷              ");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // Network Interface Status
    println!("📡 NETWORK INTERFACES");
    println!("───────────────────────────────────────────────────────────");
    match network::get_primary_interface() {
        Ok(iface) => {
            println!("Primary Interface: {}", iface.name);
            println!("Status: {}", if iface.is_up { "🟢 UP" } else { "🔴 DOWN" });
            if let Some(mac) = &iface.mac_address {
                println!("MAC Address: {}", mac);
            }
            println!("IP Addresses:");
            for ip in &iface.ip_addresses {
                println!("  • {}", ip);
            }
        }
        Err(e) => {
            println!("⚠️  Error: {}", e);
        }
    }
    
    println!();
    
    // VPN Status
    println!("🔒 VPN STATUS");
    println!("───────────────────────────────────────────────────────────");
    match network::get_vpn_status() {
        Ok(vpn) => {
            if vpn.is_connected {
                println!("Status: 🟢 CONNECTED");
                if let Some(iface) = &vpn.interface {
                    println!("Interface: {}", iface);
                }
                if let Some(ip) = &vpn.ip_address {
                    println!("VPN IP: {}", ip);
                }
                if let Some(vpn_type) = &vpn.vpn_type {
                    println!("Type: {}", vpn_type);
                }
            } else {
                println!("Status: 🔴 NOT CONNECTED");
            }
        }
        Err(e) => {
            println!("⚠️  Error: {}", e);
        }
    }
    
    println!();
    
    // Open Ports
    println!("🔓 OPEN PORTS");
    println!("───────────────────────────────────────────────────────────");
    match network::get_open_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("No listening ports detected");
            } else {
                println!("{:<10} {:<10} {:<15}", "Protocol", "Port", "State");
                println!("{}", "─".repeat(35));
                for (proto, port, state) in ports.iter().take(15) {
                    println!("{:<10} {:<10} {:<15}", proto, port, state);
                }
                if ports.len() > 15 {
                    println!("... and {} more", ports.len() - 15);
                }
            }
        }
        Err(e) => {
            println!("⚠️  Error: {}", e);
        }
    }
    
    println!();
    
    // Network Peers
    println!("👥 NETWORK PEERS");
    println!("───────────────────────────────────────────────────────────");
    match network::get_network_peers() {
        Ok(peers) => {
            if peers.is_empty() {
                println!("No active network peers detected");
            } else {
                println!("{:<20} {:<20} {:<15} {:<10}", "IP Address", "MAC Address", "Device Type", "State");
                println!("{}", "─".repeat(65));
                for peer in peers.iter().take(10) {
                    let mac = peer.mac.as_deref().unwrap_or("N/A");
                    println!("{:<20} {:<20} {:<15} {:<10}", 
                        peer.ip, mac, peer.device_type, peer.state);
                }
                if peers.len() > 10 {
                    println!("... and {} more", peers.len() - 10);
                }
            }
        }
        Err(e) => {
            println!("⚠️  Error: {}", e);
        }
    }
    
    println!();
    
    // Security Status
    println!("🛡️  SECURITY STATUS");
    println!("───────────────────────────────────────────────────────────");
    match security::scan_security_logs() {
        Ok(alerts) => {
            let critical = alerts.iter().filter(|a| matches!(a.severity, security::AlertSeverity::Critical)).count();
            let high = alerts.iter().filter(|a| matches!(a.severity, security::AlertSeverity::High)).count();
            let medium = alerts.iter().filter(|a| matches!(a.severity, security::AlertSeverity::Medium)).count();
            let low = alerts.iter().filter(|a| matches!(a.severity, security::AlertSeverity::Low)).count();
            
            println!("Alert Summary:");
            if critical > 0 {
                println!("  🔴 Critical: {}", critical);
            }
            if high > 0 {
                println!("  🟠 High: {}", high);
            }
            if medium > 0 {
                println!("  🟡 Medium: {}", medium);
            }
            if low > 0 {
                println!("  🟢 Low: {}", low);
            }
            
            if critical == 0 && high == 0 && medium == 0 && low == 0 {
                println!("  ✅ No alerts detected");
            }
            
            // Show recent alerts
            if !alerts.is_empty() {
                println!("\nRecent Alerts:");
                for alert in alerts.iter().take(5) {
                    println!("  [{}] {}: {}", 
                        alert.severity, 
                        alert.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        alert.message);
                }
            }
        }
        Err(e) => {
            println!("⚠️  Error scanning security logs: {}", e);
        }
    }
    
    match security::check_firewall_status() {
        Ok(active) => {
            println!("\nFirewall: {}", if active { "🟢 Active" } else { "🔴 Inactive" });
        }
        Err(_) => {
            println!("\nFirewall: ⚠️  Status unknown");
        }
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("\n💡 Tip: Run 'netninja-cli monitor' for live monitoring dashboard");
    println!();
    
    Ok(())
}
