#!/bin/bash
set -e

# rustup
rustup update

# cargo
cargo install \
    cargo-audit cargo-bloat cargo-deny cargo-expand cargo-machete \
    cargo-llvm-lines cargo-sort cargo-tarpaulin cargo-sort-derives \
    candid-extractor ic-wasm

# cleanup
cargo audit

# update last
cargo update --verbose

# dfx
dfxvm self update
dfxvm update

# git hooks (ensure local repo uses tracked hooks)
if [ -d .git ]; then
  git config --local core.hooksPath .githooks || true
fi
