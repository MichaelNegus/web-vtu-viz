#!/usr/bin/env bash

export RUSTFLAGS='--cfg getrandom_backend="linux_getrandom"'
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rust-vtu.wasm --target web --out-dir ../next-vtu-viz/src/app/wasm-app/
