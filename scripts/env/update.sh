#!/bin/bash
set -e

# rustup
rustup update

# cargo
cargo install \
    cargo-audit cargo-bloat cargo-expand cargo-machete cargo-llvm-lines \
    cargo-sort cargo-tarpaulin cargo-sort-derives \
    candid-extractor ic-wasm

# cleanup
cargo audit
cargo sort -w 1>/dev/null

# update last
cargo update --verbose
cargo sort-derives --check || cargo sort-derives

# dfx
dfxvm update
