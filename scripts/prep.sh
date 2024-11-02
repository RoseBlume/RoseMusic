#!/bin/bash

# Run With Ubuntu
cd ..
# Generate Node Sources
flatpak-node-generator --no-requests-cache -o node-sources.json yarn yarn.lock

# Generate Cargo Sources

python3 flatpak-builder-tools/cargo/flatpak-cargo-generator.py -o cargo-sources.json src-tauri/Cargo.lock