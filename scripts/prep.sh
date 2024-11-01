#!/bin/bash

# Run With Ubuntu
cd ..
# Generate Node Sources
flatpak-node-generator --no-requests-cache -r -o node-sources.json npm package-lock.json

# Generate Cargo Sources

python3 flatpak-builder-tools/cargo/flatpak-cargo-generator.py -o cargo-sources.json src-tauri/Cargo.lock