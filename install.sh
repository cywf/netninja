#!/bin/bash

# Define the target directory for NetNinja
NETNINJA_DIR="$(dirname "$(dirname "$(readlink -f "$0")")")"

# Create the NetNinja directory and subdirectories if they do not exist
mkdir -p $NETNINJA_DIR/{bin,lib,test,docs}

# Copy the NetNinja files to the appropriate directories
cp bin/* $NETNINJA_DIR/bin
cp lib/* $NETNINJA_DIR/lib
cp test/* $NETNINJA_DIR/test
cp README.md $NETNINJA_DIR
cp docs/* $NETNINJA_DIR/docs

# Add the symbolic link to the NetNinja executable
ln -sf $NETNINJA_DIR/bin/netninja /usr/local/bin/netninja

# Set the appropriate permissions for the NetNinja files
chmod -R 755 $NETNINJA_DIR/bin
chmod -R 755 $NETNINJA_DIR/lib
chmod -R 755 $NETNINJA_DIR/test
chmod 755 $NETNINJA_DIR/lib/helpers.sh
chmod 755 $NETNINJA_DIR/lib/troubleshooting.sh
chmod 644 $NETNINJA_DIR/README.md
chmod 644 $NETNINJA_DIR/docs/*

echo "NetNinja has been installed successfully!"
