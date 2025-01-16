#!/bin/bash
set -e

# rustup
rustup update

# cargo
cargo install \
    cargo-audit cargo-expand cargo-machete cargo-llvm-lines \
    cargo-sort cargo-tarpaulin \
    candid-extractor ic-wasm

# cleanup
cargo audit
cargo sort -w 1>/dev/null

# update last
cargo update --verbose

# dfx
dfxvm update
