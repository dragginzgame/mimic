#!/bin/bash

# Check versioning system setup
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

# VARS
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../.. && pwd)"
CARGO_TOML="$ROOT/Cargo.toml"
CHANGELOG="$ROOT/CHANGELOG.md"
VERSION_SCRIPT="$ROOT/scripts/app/version.sh"
MAKEFILE="$ROOT/Makefile"

echo "🔍 Checking versioning system setup..."
echo ""

# Check if version script exists and is executable
if [ -f "$VERSION_SCRIPT" ]; then
    if [ -x "$VERSION_SCRIPT" ]; then
        print_success "Version script exists and is executable"
    else
        print_error "Version script exists but is not executable"
        chmod +x "$VERSION_SCRIPT"
        print_info "Made version script executable"
    fi
else
    print_error "Version script not found at $VERSION_SCRIPT"
    exit 1
fi

# Check if Makefile exists
if [ -f "$MAKEFILE" ]; then
    print_success "Makefile exists"
else
    print_error "Makefile not found"
    exit 1
fi

# Check if Cargo.toml exists and has version
if [ -f "$CARGO_TOML" ]; then
    if grep -q '^version = ' "$CARGO_TOML"; then
        current_version=$(grep '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/')
        print_success "Cargo.toml exists with version: $current_version"
    else
        print_error "Cargo.toml exists but has no version field"
        exit 1
    fi
else
    print_error "Cargo.toml not found"
    exit 1
fi

# Check if CHANGELOG.md exists
if [ -f "$CHANGELOG" ]; then
    print_success "CHANGELOG.md exists"
else
    print_error "CHANGELOG.md not found"
    exit 1
fi

# Check if GitHub Actions workflows exist
if [ -d "$ROOT/.github/workflows" ]; then
    if [ -f "$ROOT/.github/workflows/ci.yml" ]; then
        print_success "CI workflow exists"
    else
        print_warning "CI workflow not found"
    fi
    
    if [ -f "$ROOT/.github/workflows/release.yml" ]; then
        print_success "Release workflow exists"
    else
        print_warning "Release workflow not found"
    fi
else
    print_warning ".github/workflows directory not found"
fi

# Test version script functionality
echo ""
print_info "Testing version script functionality..."

# Test current version command
if ./scripts/app/version.sh current > /dev/null 2>&1; then
    print_success "Version script 'current' command works"
else
    print_error "Version script 'current' command failed"
fi

# Test help command
if ./scripts/app/version.sh help > /dev/null 2>&1; then
    print_success "Version script 'help' command works"
else
    print_error "Version script 'help' command failed"
fi

# Test Makefile targets
echo ""
print_info "Testing Makefile targets..."

if make version > /dev/null 2>&1; then
    print_success "Makefile 'version' target works"
else
    print_error "Makefile 'version' target failed"
fi

if make help > /dev/null 2>&1; then
    print_success "Makefile 'help' target works"
else
    print_error "Makefile 'help' target failed"
fi

# Check git status
echo ""
print_info "Checking git status..."

if git status --porcelain | grep -q .; then
    print_warning "Working directory has uncommitted changes"
    print_info "Run 'git status' to see changes"
else
    print_success "Working directory is clean"
fi

# Check if we're in a git repository
if git rev-parse --git-dir > /dev/null 2>&1; then
    print_success "Git repository detected"
    
    # Check for existing tags
    tag_count=$(git tag | wc -l)
    if [ "$tag_count" -gt 0 ]; then
        print_info "Found $tag_count existing git tags"
        latest_tag=$(git tag --sort=-version:refname | head -n1)
        print_info "Latest tag: $latest_tag"
        
        # Check tag integrity
        print_info "Checking tag integrity..."
        for tag in $(git tag); do
            # Check if tag points to a commit
            if git rev-parse "$tag" > /dev/null 2>&1; then
                # Check if tag is annotated (more secure)
                if git cat-file -t "$tag" 2>/dev/null | grep -q "tag"; then
                    print_success "✓ $tag (annotated tag)"
                else
                    print_warning "⚠ $tag (lightweight tag - consider using annotated tags)"
                fi
            else
                print_error "✗ $tag (broken tag)"
            fi
        done
        
        # Check if current HEAD matches any tag
        current_commit=$(git rev-parse HEAD)
        matching_tags=$(git tag --points-at HEAD)
        if [ -n "$matching_tags" ]; then
            print_warning "⚠ HEAD is at tagged commit(s): $matching_tags"
            print_info "Any changes will require a new version bump"
        fi
    else
        print_info "No git tags found yet"
    fi
else
    print_warning "Not in a git repository"
fi

echo ""
print_success "Versioning system check complete!"
echo ""
print_info "Next steps:"
echo "  1. Update CHANGELOG.md with your changes"
echo "  2. Run 'make patch' (or minor/major) to bump version"
echo "  3. Run 'git push --follow-tags' to create a release"
echo ""
print_info "For more information, see VERSIONING.md" 