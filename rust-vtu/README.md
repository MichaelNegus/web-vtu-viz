# rust-vtu

Rust components for the visualiser app

## Pre-requisites 

- Working Rust installation
- Run `rustup target add wasm32-unknown-unknown`
- Run `cargo install wasm-bindgen-cli` 

## Build for web
1. `cargo build --target wasm32-unknown-unknown --release`
2. `wasm-bindgen target/wasm32-unknown-unknown/release/rust-vtu.wasm --target web --out-dir ../next-vtu-viz/src/app/wasm-app/`

At this point you'll be able to see your changes in the next app

## Run locally
- `cargo run --release`
