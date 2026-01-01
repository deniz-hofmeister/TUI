#!/bin/bash
# Build and upload TUI to hofmeister.dev

set -e

echo "Building release..."
cargo build --release

echo "Uploading to hofmeister.dev..."
scp target/release/TUI root@hofmeister.dev:/home/guest/TUI

echo "Done! Binary deployed to /home/guest/TUI"
