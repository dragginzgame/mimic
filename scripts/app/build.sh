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
# Schema
# DO THIS FIRST as it'll exit if there are any schema errors
#

#$ROOT/scripts/gen/actor.sh $CAN
#$ROOT/scripts/gen/schema.sh

#
# Build Wasm
#

mkdir -p $ROOT/.dfx/local/canisters/$CAN
WASM_TARGET=$ROOT/.dfx/local/canisters/$CAN/$CAN.wasm

cargo build --target wasm32-unknown-unknown -p canister_$CAN --locked
cp -f $ROOT/target/wasm32-unknown-unknown/debug/canister_$CAN.wasm $WASM_TARGET

