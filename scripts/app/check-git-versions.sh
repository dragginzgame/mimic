#!/bin/bash

# Check available git versions for Mimic integration
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

REPO_URL="https://github.com/dragginzgame/mimic"

echo "🔍 Checking available Mimic versions for git dependencies..."
echo ""

# Check if git is available
if ! command -v git &> /dev/null; then
    print_error "Git is not installed or not in PATH"
    exit 1
fi

# Fetch latest tags from remote
print_info "Fetching latest tags from $REPO_URL..."

# Check if we can access the remote
if git ls-remote --tags "$REPO_URL" > /dev/null 2>&1; then
    print_success "Repository is accessible"
else
    print_error "Cannot access repository $REPO_URL"
    print_info "Check your internet connection and repository URL"
    exit 1
fi

# Get all tags and sort by version
print_info "Available versions (sorted by latest first):"
echo ""

# Fetch and display tags
git ls-remote --tags "$REPO_URL" | \
    grep -E 'refs/tags/v[0-9]+\.[0-9]+\.[0-9]+' | \
    sed 's/.*refs\/tags\///' | \
    sort -V -r | \
    head -10 | \
    while read tag; do
        echo "  📦 $tag"
    done

echo ""
print_info "Integration examples:"
echo ""

# Show integration examples for latest versions
latest_tags=$(git ls-remote --tags "$REPO_URL" | \
    grep -E 'refs/tags/v[0-9]+\.[0-9]+\.[0-9]+' | \
    sed 's/.*refs\/tags\///' | \
    sort -V -r | \
    head -3)

echo "# Latest stable versions:"
echo ""

for tag in $latest_tags; do
    version=${tag#v}
    echo "## $tag"
    echo '```toml'
    echo "[dependencies]"
    echo "mimic = { git = \"$REPO_URL\", tag = \"$tag\", features = [] }"
    echo '```'
    echo ""
done

echo "# Development version (latest main branch):"
echo '```toml'
echo "[dependencies]"
echo "mimic = { git = \"$REPO_URL\", branch = \"main\", features = [] }"
echo '```'
echo ""

print_info "Feature flags you can use:"
echo "  - default (always enabled)"
echo "  - serde (serialization support)"
echo "  - icu (internationalization)"
echo "  - ulid (ULID support)"
echo "  - decimal (decimal arithmetic)"
echo ""

print_info "Example with features:"
echo '```toml'
echo "[dependencies]"
echo "mimic = { git = \"$REPO_URL\", tag = \"v0.9.3\", features = [\"serde\", \"ulid\"] }"
echo '```'
echo ""

print_info "To check if a specific version exists:"
echo "  git ls-remote --tags $REPO_URL | grep v0.9.3"
echo ""

print_info "For more information:"
echo "  - Integration guide: INTEGRATION.md"
echo "  - Versioning guide: VERSIONING.md"
echo "  - Changelog: CHANGELOG.md" 