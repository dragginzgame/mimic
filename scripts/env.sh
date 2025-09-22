#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../ && pwd)"
SCRIPTS="$ROOT/scripts/"
echo $SCRIPTS
NETWORK=${NETWORK:-local}

# rust
export RUST_BACKTRACE=1

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color
