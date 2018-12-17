#!/usr/bin/env bash
mkdir -p ./pkg
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/draco.wasm --no-modules --out-dir ./pkg
