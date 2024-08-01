#!/bin/bash
set -e

cd $HOME/projects/dragginz/backend

# rustup
rustup update

# cargo
cargo install \
    cargo-audit cargo-expand cargo-machete cargo-llvm-lines \
    cargo-outdated cargo-sort cargo-udeps  \
    candid-extractor ic-wasm
cargo update --verbose

# dfx
dfxvm update