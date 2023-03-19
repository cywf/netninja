#!/bin/bash

# Define the target directory for NetNinja
NETNINJA_DIR="$(dirname "$(dirname "$(readlink -f "$0")")")"

# Create the NetNinja directory and subdirectories if they do not exist
mkdir -p $NETNINJA_DIR/{bin,lib,test,docs}

# Copy the NetNinja files to the appropriate directories
if [ -f helpers.sh ]; then
  cp helpers.sh $NETNINJA_DIR/lib
else
  echo "Error: helpers.sh not found"
  exit 1
fi

if [ -f troubleshooting.sh ]; then
  cp troubleshooting.sh $NETNINJA_DIR/lib
else
  echo "Error: troubleshooting.sh not found"
  exit 1
fi

if [ -f LICENSE.txt ]; then
  cp LICENSE.txt $NETNINJA_DIR
else
  echo "Error: LICENSE.txt not found"
  exit 1
fi

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
chmod 644 $NETNINJA_DIR/LICENSE.txt
chmod 644 $NETNINJA_DIR/docs/*

echo "NetNinja has been installed successfully!"
