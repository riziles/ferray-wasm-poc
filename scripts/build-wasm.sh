#!/usr/bin/env bash
# Build the ferray WASM binary and copy bindings into the Svelte app.
# Run this before `pnpm dev` or `pnpm build` whenever Rust source changes.
set -euo pipefail

cd "$(dirname "$0")/.."

echo "Building ferray WASM..."
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript \
  --out-dir svelte-app/src/lib/wasm \
  target/wasm32-unknown-unknown/release/ferray_wasm_poc.wasm
echo "Done — WASM written to svelte-app/src/lib/wasm/"
