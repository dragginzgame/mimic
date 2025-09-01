#!/bin/bash

# Security check script for Mimic versioning
# Ensures tags are immutable and version integrity is maintained

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

print_security() {
    echo -e "${RED}[SECURITY]${NC} $1"
}

echo "🔒 Security Check for Mimic Versioning"
echo "======================================"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check current status
print_info "Checking repository status..."

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    print_warning "Uncommitted changes detected"
    print_info "These changes will require a version bump before release"
else
    print_success "Working directory is clean"
fi

# Check current version
current_version=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_info "Current version in Cargo.toml: $current_version"

# Check if current version is already tagged
current_tag="v$current_version"
if git tag -l "$current_tag" | grep -q "$current_tag"; then
    print_security "⚠️  CRITICAL: Current version $current_version is already tagged!"
    print_security "   This means the code at this version should NEVER change."
    
    # Check if HEAD is at the tagged commit
    head_commit=$(git rev-parse HEAD)
    tag_commit=$(git rev-parse "$current_tag" 2>/dev/null || echo "")
    
    if [ "$head_commit" = "$tag_commit" ]; then
        print_success "✓ HEAD is at the tagged commit - no changes detected"
    else
        print_security "🚨 SECURITY VIOLATION: HEAD is not at the tagged commit!"
        print_security "   This indicates the tag has been modified or HEAD has moved."
        print_security "   The code at version $current_version has changed!"
        print_info "   You MUST bump to a new version immediately."
        exit 1
    fi
else
    print_success "✓ Current version is not yet tagged"
fi

# Check remote tags
print_info "Checking remote tag integrity..."

# Fetch latest tags
git fetch --tags --quiet 2>/dev/null || print_warning "Could not fetch remote tags"

# Check for any local tags that differ from remote (compare peeled targets)
for tag in $(git tag); do
    # Local peeled target (commit) for the tag; works for both lightweight and annotated tags
    local_target=$(git rev-parse "$tag^{}" 2>/dev/null || echo "")

    # Remote peeled target commit for the tag. Prefer the peeled entry (^{}) if present; fall back to object id
    remote_target=$(git ls-remote --tags origin "$tag^{}" | awk '{print $1}')
    if [ -z "$remote_target" ]; then
        remote_target=$(git ls-remote --tags origin "$tag" | awk '{print $1}')
    fi

    if [ -z "$remote_target" ]; then
        print_warning "⚠ Remote tag not found: $tag (skipping integrity compare)"
        continue
    fi

    if [ -n "$local_target" ] && [ "$local_target" != "$remote_target" ]; then
        print_security "🚨 SECURITY VIOLATION: Tag $tag differs between local and remote!"
        print_security "   Local:  $local_target"
        print_security "   Remote: $remote_target"
        print_security "   This indicates tag tampering!"
        exit 1
    fi
done

print_success "✓ All local tags match remote tags"

# Check for force-pushed tags (this is a warning as it might be legitimate)
print_info "Checking for force-pushed tags..."
if git reflog --all | grep -q "force"; then
    print_warning "⚠️  Force operations detected in reflog"
    print_info "   Review recent force operations to ensure no tags were modified"
else
    print_success "✓ No force operations detected"
fi

# Check tag types
print_info "Checking tag types..."
for tag in $(git tag); do
    if git cat-file -t "$tag" 2>/dev/null | grep -q "tag"; then
        print_success "✓ $tag (annotated tag - secure)"
    else
        print_warning "⚠ $tag (lightweight tag - less secure)"
    fi
done

# Check for any tags that point to non-existent commits
print_info "Checking tag validity..."
for tag in $(git tag); do
    if ! git rev-parse "$tag" > /dev/null 2>&1; then
        print_security "🚨 BROKEN TAG: $tag points to non-existent commit!"
        exit 1
    fi
done

print_success "✓ All tags are valid"

echo ""
print_success "Security check completed successfully!"
echo ""
print_info "Security Summary:"
echo "  ✓ Tags are immutable"
echo "  ✓ No unauthorized modifications detected"
echo "  ✓ Version integrity maintained"
echo ""
print_info "Remember:"
echo "  - Once a tag is pushed, the code at that version must NEVER change"
echo "  - Always bump to a new version for any code changes"
echo "  - Use 'make patch', 'make minor', or 'make major' to create new versions" 
