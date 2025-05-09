#!/bin/bash

# dir
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../../ && pwd)"
echo $PROJECT_ROOT
cd $PROJECT_ROOT

# build
dfx canister create test
dfx build test
dfx canister install -y test --mode=reinstall
dfx canister call test test
