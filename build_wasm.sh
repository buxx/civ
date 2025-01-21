#!/bin/bash

set -euo pipefail
arg=${1:-}
arg=$(echo "$arg" | tr '[:upper:]' '[:lower:]')

cargo build --target wasm32-unknown-unknown --release --bin wui
wasm-bindgen --no-typescript --out-name wui --out-dir wasm/ --target web target/wasm32-unknown-unknown/release/wui.wasm
