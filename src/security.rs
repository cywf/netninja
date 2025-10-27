use anyhow::{Context, Result};
use std::process::Command;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub timestamp: DateTime<Utc>,
    pub severity: AlertSeverity,
    pub category: AlertCategory,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCategory {
    FailedLogin,
    PortScan,
    UnusualTraffic,
    FirewallBlock,
    SuspiciousProcess,
    SystemChange,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSeverity::Critical => write!(f, "CRITICAL"),
            AlertSeverity::High => write!(f, "HIGH"),
            AlertSeverity::Medium => write!(f, "MEDIUM"),
            AlertSeverity::Low => write!(f, "LOW"),
            AlertSeverity::Info => write!(f, "INFO"),
        }
    }
}

/// Monitor system logs for security events
pub fn scan_security_logs() -> Result<Vec<SecurityAlert>> {
    let mut alerts = Vec::new();
    
    // Check auth logs for failed login attempts
    if let Ok(failed_logins) = check_failed_logins() {
        alerts.extend(failed_logins);
    }
    
    // Check for unusual network connections
    if let Ok(network_alerts) = check_network_connections() {
        alerts.extend(network_alerts);
    }
    
    // Check firewall logs
    if let Ok(firewall_alerts) = check_firewall_logs() {
        alerts.extend(firewall_alerts);
    }
    
    Ok(alerts)
}

fn check_failed_logins() -> Result<Vec<SecurityAlert>> {
    let output = Command::new("journalctl")
        .args(&["-u", "ssh", "-n", "100", "--no-pager"])
        .output();
    
    let mut alerts = Vec::new();
    
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            if line.contains("Failed password") || line.contains("Invalid user") {
                alerts.push(SecurityAlert {
                    timestamp: Utc::now(),
                    severity: AlertSeverity::Medium,
                    category: AlertCategory::FailedLogin,
                    message: "Failed SSH login attempt detected".to_string(),
                    details: Some(line.to_string()),
                });
            }
        }
    }
    
    Ok(alerts)
}

fn check_network_connections() -> Result<Vec<SecurityAlert>> {
    let output = Command::new("ss")
        .args(&["-tan"])
        .output()
        .context("Failed to check network connections")?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut alerts = Vec::new();
    
    // Count connections per IP
    let mut connection_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    // Configurable threshold - can be adjusted based on system capacity
    const CONNECTION_THRESHOLD: usize = 50;
    
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 4 {
            // Get the local address field
            if let Some(addr) = parts.get(4) {
                if let Some(ip) = addr.split(':').next() {
                    if !ip.starts_with("127.") && !ip.starts_with("::1") && !ip.is_empty() {
                        *connection_counts.entry(ip.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    // Alert on suspicious connection counts
    for (ip, count) in connection_counts {
        if count > CONNECTION_THRESHOLD {
            alerts.push(SecurityAlert {
                timestamp: Utc::now(),
                severity: AlertSeverity::High,
                category: AlertCategory::UnusualTraffic,
                message: format!("High connection count from {}: {} connections", ip, count),
                details: Some("Possible port scan or DDoS attempt".to_string()),
            });
        }
    }
    
    Ok(alerts)
}

fn check_firewall_logs() -> Result<Vec<SecurityAlert>> {
    let output = Command::new("dmesg")
        .args(&["-T", "--level=warn,err"])
        .output();
    
    let mut alerts = Vec::new();
    
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            if line.contains("UFW BLOCK") || line.contains("iptables") {
                alerts.push(SecurityAlert {
                    timestamp: Utc::now(),
                    severity: AlertSeverity::Info,
                    category: AlertCategory::FirewallBlock,
                    message: "Firewall block detected".to_string(),
                    details: Some(line.to_string()),
                });
            }
        }
    }
    
    Ok(alerts)
}

/// Check if firewall is active
pub fn check_firewall_status() -> Result<bool> {
    let output = Command::new("ufw")
        .arg("status")
        .output();
    
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return Ok(output_str.contains("Status: active"));
    }
    
    // Try iptables as fallback
    let output = Command::new("iptables")
        .args(&["-L", "-n"])
        .output();
    
    if let Ok(output) = output {
        return Ok(output.status.success());
    }
    
    Ok(false)
}

/// Get summary of security status
pub fn get_security_summary() -> Result<String> {
    let alerts = scan_security_logs()?;
    let firewall_active = check_firewall_status()?;
    
    let critical_count = alerts.iter().filter(|a| matches!(a.severity, AlertSeverity::Critical)).count();
    let high_count = alerts.iter().filter(|a| matches!(a.severity, AlertSeverity::High)).count();
    let medium_count = alerts.iter().filter(|a| matches!(a.severity, AlertSeverity::Medium)).count();
    
    Ok(format!(
        "Firewall: {}\nAlerts - Critical: {}, High: {}, Medium: {}",
        if firewall_active { "Active" } else { "Inactive" },
        critical_count,
        high_count,
        medium_count
    ))
}
