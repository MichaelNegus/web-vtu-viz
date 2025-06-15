#!/usr/bin/env bash

echo "Building Rust VTU WASM for native HTML/JS version..."

# Clean previous build
rm -rf ../web-vtu-viz/wasm/

# Build the WASM module
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings and output to web-vtu-viz/wasm/
wasm-bindgen target/wasm32-unknown-unknown/release/rust-vtu.wasm \
  --target web \
  --out-dir ../web-vtu-viz/wasm/ \
  --no-typescript

echo "WASM build complete! Files generated in web-vtu-viz/wasm/"
echo ""
echo "Generated files:"
ls -la ../web-vtu-viz/wasm/
