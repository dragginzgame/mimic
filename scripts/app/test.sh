#!/bin/bash

# Set up environment
$(dirname "$0")/../env.sh
cd "$SCRIPTS"

# build
dfx canister create test
dfx build test
dfx ledger fabricate-cycles --canister test --cycles 9000000000000000
dfx canister install -y test --mode=reinstall
dfx canister call test test
