#!/bin/bash

# Define the target directory for NetNinja
NETNINJA_DIR="$(dirname "$(dirname "$(readlink -f "$0")")")"

# Log function for consistent output
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Function to check if a command is available
check_command() {
    local command=$1
    local package=$2

    if ! command -v "$command" &> /dev/null; then
        log "$command not found. Installing $package..."
        if sudo apt-get update -y && sudo apt-get install -y "$package"; then
            log "$package installed successfully."
        else
            log "Failed to install $package. Please install it manually."
            exit 1
        fi
    else
        log "$command is already installed."
    fi
}

# Ensure required tools are available
log "Checking system dependencies..."
check_command "bash" "bash"
check_command "ping" "iputils-ping"
check_command "route" "net-tools"
check_command "nslookup" "dnsutils"
check_command "iptables" "iptables"
check_command "ufw" "ufw"
check_command "tmux" "tmux"
check_command "ss" "iproute2"

# Optional but recommended tools
log "Checking optional tools..."
if ! command -v iftop &> /dev/null; then
    log "iftop not found. For best experience, install it with: sudo apt-get install -y iftop"
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    log "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    log "Rust installed successfully."
else
    log "Rust is already installed."
fi

# Build the Rust CLI
log "Building NetNinja Rust CLI..."
cd "$NETNINJA_DIR" || exit 1
if cargo build --release; then
    log "Rust CLI built successfully."
else
    log "Failed to build Rust CLI. Please check the error messages above."
    exit 1
fi

# Create the NetNinja directory and subdirectories if they do not exist
log "Setting up NetNinja directories..."
mkdir -p "$NETNINJA_DIR"/{bin,lib,test,docs}

# Copy/update the NetNinja files to the appropriate directories
log "Updating NetNinja files..."
cp -r bin/* "$NETNINJA_DIR/bin" 2>/dev/null || true
cp -r lib/* "$NETNINJA_DIR/lib" 2>/dev/null || true
cp -r test/* "$NETNINJA_DIR/test" 2>/dev/null || true
cp README.md "$NETNINJA_DIR" 2>/dev/null || true
cp -r docs/* "$NETNINJA_DIR/docs" 2>/dev/null || true

# Add a symbolic link to the Rust CLI executable
log "Creating symbolic link for NetNinja CLI..."
if sudo ln -sf "$NETNINJA_DIR/target/release/netninja-cli" /usr/local/bin/netninja-cli; then
    log "Symbolic link created successfully at /usr/local/bin/netninja-cli."
else
    log "Failed to create symbolic link. Please check your permissions."
    exit 1
fi

# Keep the old bash script accessible as well
if [ -f "$NETNINJA_DIR/bin/netninja" ]; then
    log "Creating symbolic link for legacy bash script..."
    sudo ln -sf "$NETNINJA_DIR/bin/netninja" /usr/local/bin/netninja || log "Warning: Could not create legacy link"
fi

# Set the appropriate permissions for the NetNinja files
log "Setting permissions for NetNinja files..."
chmod -R 755 "$NETNINJA_DIR/bin" 2>/dev/null || true
chmod -R 755 "$NETNINJA_DIR/lib" 2>/dev/null || true
chmod -R 755 "$NETNINJA_DIR/test" 2>/dev/null || true
chmod 755 "$NETNINJA_DIR/lib/helpers.sh" 2>/dev/null || true
chmod 755 "$NETNINJA_DIR/lib/troubleshooting.sh" 2>/dev/null || true
chmod 644 "$NETNINJA_DIR/README.md" 2>/dev/null || true
chmod 644 "$NETNINJA_DIR/docs/"* 2>/dev/null || true

# Optional: Add alias for NetNinja
if ! grep -q "alias netninja-cli" ~/.bashrc; then
    log "Adding alias for NetNinja CLI in ~/.bashrc..."
    echo 'alias netninja-cli="/usr/local/bin/netninja-cli"' >> ~/.bashrc
    log "Alias added. Use 'netninja-cli' to run the tool."
fi

log "NetNinja has been installed successfully!"
log ""
log "Usage:"
log "  netninja-cli status   - Show network status summary"
log "  netninja-cli monitor  - Launch immersive tmux monitoring dashboard"
log "  netninja              - Use legacy bash script (if needed)"
log ""
log "For the monitoring dashboard, you may need to run with sudo for full functionality:"
log "  sudo netninja-cli monitor"
log ""

