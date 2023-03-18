#!/bin/bash

# A function to print a message to the console with a timestamp
log() {
  echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# A function to check if a file exists and is readable
is_readable() {
  if [ -r "$1" ]; then
    return 0
  else
    return 1
  fi
}

# A function to check if a directory exists and is writable
is_writable() {
  if [ -w "$1" ]; then
    return 0
  else
    return 1
  fi
}

# A function to check if a program is installed
is_installed() {
  if command -v $1 &> /dev/null; then
    return 0
  else
    return 1
  fi
}
