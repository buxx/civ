#!/bin/bash

set -euo pipefail
arg=${1:-}
arg=$(echo "$arg" | tr '[:upper:]' '[:lower:]')

export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
cargo build --target wasm32-unknown-unknown --release --bin gui
echo "bindgen ..."
wasm-bindgen --no-typescript --out-name gui --out-dir wasm/ --target web target/wasm32-unknown-unknown/release/gui.wasm
