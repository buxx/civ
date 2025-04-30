# civ

[benchmarks](https://buxx.github.io/civ/dev/bench/)

test

    cargo xtest

test wui

    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli
    cargo install wasm-pack
    wasm-pack test --node crates/civ_gui
