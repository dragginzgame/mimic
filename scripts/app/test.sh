#!/bin/bash

# dir
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../../ && pwd)"
echo $PROJECT_ROOT
cd $PROJECT_ROOT

# build
dfx canister create test
dfx build test
dfx ledger fabricate-cycles --canister test --cycles 9000000000000000
dfx canister install -y test --mode=reinstall
dfx canister call test test
