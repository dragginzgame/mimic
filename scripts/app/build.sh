#!/bin/bash

# don't allow errors
set -e
export RUST_BACKTRACE=1

# VARS
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../.. && pwd)"

# Check if an argument was provided
if [ $# -eq 0 ]; then
    echo "usage: build.sh [canister_name]"
    exit 1
fi
CAN=$1

#
# Build Wasm
#

mkdir -p $ROOT/.dfx/local/canisters/$CAN
WASM_TARGET=$ROOT/.dfx/local/canisters/$CAN/$CAN.wasm

cargo build --target wasm32-unknown-unknown -p $CAN --locked
cp -f $ROOT/target/wasm32-unknown-unknown/debug/$CAN.wasm $WASM_TARGET

# extract candid

candid-extractor "$ROOT/.dfx/local/canisters/$CAN/$CAN.wasm" \
    > "$ROOT/.dfx/local/canisters/$CAN/${CAN}.did"