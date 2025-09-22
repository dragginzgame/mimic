#!/usr/bin/env bash
set -euo pipefail

# Root of the repo
PROJECT_ROOT="${PROJECT_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)}"
SCRIPTS="${SCRIPTS:-$PROJECT_ROOT/scripts}"

# Network: default to "local" if not set
NETWORK="${NETWORK:-local}"

# Environment: default to "dev" if not set
ENV="${ENV:-dev}"

# Export so other commands see them
export PROJECT_ROOT SCRIPTS NETWORK ENV

echo "📁 PROJECT_ROOT=$PROJECT_ROOT ($NETWORK/$ENV)"

# Rust debug output
export RUST_BACKTRACE=1

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export PURPLE='\033[0;35m'
export NC='\033[0m' # No Color
