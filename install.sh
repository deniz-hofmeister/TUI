#!/bin/bash
# Install Rust and dependencies for building TUI on Ubuntu

set -e

echo "Installing build dependencies..."
sudo apt update
sudo apt install -y build-essential curl

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo "Loading Rust environment..."
source "$HOME/.cargo/env"

echo "Done! Run 'source ~/.cargo/env' or restart your terminal, then run ./upload.sh"
