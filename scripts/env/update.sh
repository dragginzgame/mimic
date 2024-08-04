#!/bin/bash
set -e

# rustup
rustup update

# cargo
cargo install \
    cargo-audit cargo-expand cargo-machete cargo-llvm-lines \
    cargo-outdated cargo-sort cargo-udeps cargo-tarpaulin \
    candid-extractor ic-wasm

# dfx
dfxvm update

# cleanup
cargo audit
cargo outdated
cargo sort -w 1>/dev/null

# test coverage
cargo tarpaulin --out Xml

# update last
cargo update --verbose
