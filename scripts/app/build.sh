#!/bin/bash

# don't allow errors
set -e

# Set up environment
source "$(dirname "$0")/../env.sh"
cd "$SCRIPTS"

# Check if an argument was provided
if [ $# -eq 0 ]; then
    echo "usage: build.sh [canister_name]"
    exit 1
fi
CAN=$1

#
# Build Wasm
#

mkdir -p $PROJECT_ROOT/.dfx/local/canisters/$CAN
WASM_TARGET=$PROJECT_ROOT/.dfx/local/canisters/$CAN/$CAN.wasm

cargo build --target wasm32-unknown-unknown -p canister_$CAN
cp -f $PROJECT_ROOT/target/wasm32-unknown-unknown/debug/canister_$CAN.wasm $WASM_TARGET

# extract candid

candid-extractor "$PROJECT_ROOT/.dfx/local/canisters/$CAN/$CAN.wasm" \
    > "$PROJECT_ROOT/.dfx/local/canisters/$CAN/${CAN}.did"
