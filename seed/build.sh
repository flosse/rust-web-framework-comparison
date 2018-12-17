#!/usr/bin/env bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/seed.wasm --no-modules --out-dir ./pkg
