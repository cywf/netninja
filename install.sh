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
check_command "zerotier-cli" "zerotier-one"

# Create the NetNinja directory and subdirectories if they do not exist
log "Setting up NetNinja directories..."
mkdir -p "$NETNINJA_DIR"/{bin,lib,test,docs}

# Copy the NetNinja files to the appropriate directories
log "Copying NetNinja files..."
cp -r bin/* "$NETNINJA_DIR/bin"
cp -r lib/* "$NETNINJA_DIR/lib"
cp -r test/* "$NETNINJA_DIR/test"
cp README.md "$NETNINJA_DIR"
cp -r docs/* "$NETNINJA_DIR/docs"

# Add a symbolic link to the NetNinja executable
log "Creating symbolic link for NetNinja..."
if ln -sf "$NETNINJA_DIR/bin/netninja" /usr/local/bin/netninja; then
    log "Symbolic link created successfully at /usr/local/bin/netninja."
else
    log "Failed to create symbolic link. Please check your permissions."
    exit 1
fi

# Set the appropriate permissions for the NetNinja files
log "Setting permissions for NetNinja files..."
chmod -R 755 "$NETNINJA_DIR/bin"
chmod -R 755 "$NETNINJA_DIR/lib"
chmod -R 755 "$NETNINJA_DIR/test"
chmod 755 "$NETNINJA_DIR/lib/helpers.sh"
chmod 755 "$NETNINJA_DIR/lib/troubleshooting.sh"
chmod 644 "$NETNINJA_DIR/README.md"
chmod 644 "$NETNINJA_DIR/docs/"*

# Optional: Add alias for NetNinja
if ! grep -q "alias netninja" ~/.bashrc; then
    log "Adding alias for NetNinja in ~/.bashrc..."
    echo 'alias netninja="/usr/local/bin/netninja"' >> ~/.bashrc
    source ~/.bashrc
    log "Alias added. Use 'netninja' to run the tool."
fi

log "NetNinja has been installed successfully! Use 'netninja' to start troubleshooting."
