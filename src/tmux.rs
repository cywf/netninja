use anyhow::{Context, Result};
use std::process::Command;

pub struct TmuxSession {
    pub name: String,
}

impl TmuxSession {
    /// Create a new tmux session
    pub fn new(name: &str) -> Result<Self> {
        // Check if tmux is installed
        Command::new("tmux")
            .arg("-V")
            .output()
            .context("tmux is not installed. Please install tmux to use the monitoring dashboard.")?;
        
        Ok(TmuxSession {
            name: name.to_string(),
        })
    }
    
    /// Check if session already exists
    pub fn exists(&self) -> bool {
        Command::new("tmux")
            .args(&["has-session", "-t", &self.name])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    /// Kill existing session
    pub fn kill(&self) -> Result<()> {
        if self.exists() {
            Command::new("tmux")
                .args(&["kill-session", "-t", &self.name])
                .output()
                .context("Failed to kill existing tmux session")?;
        }
        Ok(())
    }
    
    /// Create new session
    pub fn create(&self) -> Result<()> {
        Command::new("tmux")
            .args(&["new-session", "-d", "-s", &self.name])
            .output()
            .context("Failed to create tmux session")?;
        Ok(())
    }
    
    /// Split window horizontally
    pub fn split_horizontal(&self, pane_id: Option<&str>) -> Result<()> {
        let mut args = vec!["split-window", "-h", "-t"];
        args.push(&self.name);
        
        if let Some(pane) = pane_id {
            args.push(pane);
        }
        
        Command::new("tmux")
            .args(&args)
            .output()
            .context("Failed to split window horizontally")?;
        Ok(())
    }
    
    /// Split window vertically
    pub fn split_vertical(&self, pane_id: Option<&str>) -> Result<()> {
        let mut args = vec!["split-window", "-v", "-t"];
        args.push(&self.name);
        
        if let Some(pane) = pane_id {
            args.push(pane);
        }
        
        Command::new("tmux")
            .args(&args)
            .output()
            .context("Failed to split window vertically")?;
        Ok(())
    }
    
    /// Send command to a specific pane
    pub fn send_keys(&self, pane: &str, command: &str) -> Result<()> {
        Command::new("tmux")
            .args(&["send-keys", "-t", &format!("{}:{}", self.name, pane), command, "C-m"])
            .output()
            .context("Failed to send keys to pane")?;
        Ok(())
    }
    
    /// Select a specific pane layout
    pub fn select_layout(&self, layout: &str) -> Result<()> {
        Command::new("tmux")
            .args(&["select-layout", "-t", &self.name, layout])
            .output()
            .context("Failed to select layout")?;
        Ok(())
    }
    
    /// Attach to the session
    pub fn attach(&self) -> Result<()> {
        Command::new("tmux")
            .args(&["attach-session", "-t", &self.name])
            .status()
            .context("Failed to attach to tmux session")?;
        Ok(())
    }
    
    /// Set pane title
    pub fn set_pane_title(&self, pane: &str, title: &str) -> Result<()> {
        self.send_keys(pane, &format!("printf '\\033]2;{}\\033\\\\'", title))?;
        Ok(())
    }
    
    /// Rename window
    pub fn rename_window(&self, window: &str, name: &str) -> Result<()> {
        Command::new("tmux")
            .args(&["rename-window", "-t", &format!("{}:{}", self.name, window), name])
            .output()
            .context("Failed to rename window")?;
        Ok(())
    }
}

/// Setup the complete monitoring dashboard layout
pub fn setup_dashboard_layout(session: &TmuxSession) -> Result<()> {
    // Create the initial session
    session.create()?;
    
    // Rename the first window
    session.rename_window("0", "NetNinja Monitor")?;
    
    // Create 6-pane layout:
    // +----------------+----------------+
    // |                |                |
    // |   Network      |   VPN Status   |
    // |   Status       |                |
    // +----------------+----------------+
    // |                |                |
    // |   iftop        |   Open Ports   |
    // |                |                |
    // +----------------+----------------+
    // |                |                |
    // |   Security     |   Network      |
    // |   Alerts       |   Peers        |
    // +----------------+----------------+
    
    // Split into two columns
    session.split_horizontal(None)?;
    
    // Split left column into 3 rows
    session.split_vertical(Some("0"))?;
    session.split_vertical(Some("1"))?;
    
    // Split right column into 3 rows
    session.split_vertical(Some("3"))?;
    session.split_vertical(Some("4"))?;
    
    // Balance the layout
    session.select_layout("tiled")?;
    
    Ok(())
}

/// Populate panes with monitoring commands
pub fn populate_panes(session: &TmuxSession) -> Result<()> {
    // Pane 0: Network Status
    session.send_keys("0", "clear")?;
    // Use basic commands to avoid recursion
    session.send_keys("0", "watch -n 2 'ip addr show | grep -E \"^[0-9]|inet \" | head -20'")?;
    
    // Pane 1: iftop (live traffic)
    session.send_keys("1", "clear")?;
    // Check if iftop is available, otherwise show message with short refresh
    session.send_keys("1", "if command -v iftop &> /dev/null; then sudo iftop -t -s 2; else while true; do clear; echo 'iftop not installed. Install with: sudo apt-get install iftop'; echo ''; echo 'Alternative: Showing network stats'; cat /proc/net/dev | head -10; sleep 10; done; fi")?;
    
    // Pane 2: Security Alerts
    session.send_keys("2", "clear")?;
    session.send_keys("2", "watch -n 5 'echo \"=== Security Status ===\"  && netninja-cli status 2>/dev/null | grep -A20 \"Security\" || journalctl -u ssh -n 10 --no-pager | tail -5'")?;
    
    // Pane 3: VPN Status
    session.send_keys("3", "clear")?;
    session.send_keys("3", "watch -n 5 'echo \"=== VPN Status ===\" && ip addr show | grep -E \"tun|tap|wg\" || echo \"No VPN detected\"'")?;
    
    // Pane 4: Open Ports
    session.send_keys("4", "clear")?;
    session.send_keys("4", "watch -n 5 'echo \"=== Open Ports ===\" && ss -tuln | head -20'")?;
    
    // Pane 5: Network Peers
    session.send_keys("5", "clear")?;
    session.send_keys("5", "watch -n 10 'echo \"=== Network Peers ===\" && ip neigh show | head -15'")?;
    
    Ok(())
}

/// Create a shell script to launch the monitoring dashboard
pub fn create_monitor_script() -> Result<()> {
    let script_content = r#"#!/bin/bash
# NetNinja Monitoring Dashboard Launcher

SESSION_NAME="netninja-monitor"

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo "Error: tmux is not installed. Please install it first:"
    echo "  sudo apt-get install tmux"
    exit 1
fi

# Kill existing session if it exists
tmux kill-session -t "$SESSION_NAME" 2>/dev/null

# Create new session
tmux new-session -d -s "$SESSION_NAME"

# Rename window
tmux rename-window -t "$SESSION_NAME:0" "NetNinja Monitor"

# Create pane layout (6 panes)
tmux split-window -h -t "$SESSION_NAME"
tmux select-pane -t "$SESSION_NAME:0.0"
tmux split-window -v -t "$SESSION_NAME:0.0"
tmux split-window -v -t "$SESSION_NAME:0.1"
tmux select-pane -t "$SESSION_NAME:0.3"
tmux split-window -v -t "$SESSION_NAME:0.3"
tmux split-window -v -t "$SESSION_NAME:0.4"

# Balance layout
tmux select-layout -t "$SESSION_NAME" tiled

# Populate panes with commands
# Pane 0: Network Status
tmux send-keys -t "$SESSION_NAME:0.0" 'clear && watch -n 2 "ip addr show | grep -E \"^[0-9]|inet \" | head -20"' C-m

# Pane 1: iftop or network stats
tmux send-keys -t "$SESSION_NAME:0.1" 'clear && if command -v iftop &> /dev/null; then sudo iftop -t -s 2 2>/dev/null || echo "Run with sudo for iftop"; else watch -n 2 "cat /proc/net/dev"; fi' C-m

# Pane 2: Security/Auth logs
tmux send-keys -t "$SESSION_NAME:0.2" 'clear && watch -n 5 "echo \"=== Recent Auth Logs ===\" && journalctl -u ssh -n 10 --no-pager 2>/dev/null | tail -5 || tail -10 /var/log/auth.log 2>/dev/null || echo \"No auth logs accessible\""' C-m

# Pane 3: VPN Status
tmux send-keys -t "$SESSION_NAME:0.3" 'clear && watch -n 5 "echo \"=== VPN Status ===\" && ip addr show | grep -E \"tun|tap|wg\" -A2 || echo \"No VPN interface detected\""' C-m

# Pane 4: Open Ports
tmux send-keys -t "$SESSION_NAME:0.4" 'clear && watch -n 5 "echo \"=== Listening Ports ===\" && ss -tuln | head -20"' C-m

# Pane 5: Network Neighbors/Peers
tmux send-keys -t "$SESSION_NAME:0.5" 'clear && watch -n 10 "echo \"=== Network Neighbors ===\" && ip neigh show | head -15"' C-m

# Attach to session
echo "Starting NetNinja Monitoring Dashboard..."
echo "Press Ctrl+B then D to detach from the session"
sleep 1
tmux attach-session -t "$SESSION_NAME"
"#;
    
    let script_path = "/tmp/netninja-monitor.sh";
    std::fs::write(script_path, script_content)
        .context("Failed to create monitor script")?;
    
    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(script_path, perms)?;
    }
    
    Ok(())
}
