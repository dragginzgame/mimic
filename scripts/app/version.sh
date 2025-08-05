#!/bin/bash

# Version management script for Mimic
# Usage: ./version.sh [major|minor|patch|release]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# VARS
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../.. && pwd)"
CARGO_TOML="$ROOT/Cargo.toml"
CHANGELOG="$ROOT/CHANGELOG.md"

# Function to print colored output
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

# Function to get current version
get_current_version() {
    grep '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/'
}

# Function to bump version
bump_version() {
    local current_version=$1
    local bump_type=$2
    
    IFS='.' read -ra VERSION_PARTS <<< "$current_version"
    local major=${VERSION_PARTS[0]}
    local minor=${VERSION_PARTS[1]}
    local patch=${VERSION_PARTS[2]}
    
    case $bump_type in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            print_error "Invalid bump type: $bump_type. Use major, minor, or patch"
            exit 1
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Function to update version in Cargo.toml
update_cargo_version() {
    local new_version=$1
    local temp_file=$(mktemp)
    
    sed "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML" > "$temp_file"
    mv "$temp_file" "$CARGO_TOML"
    
    print_success "Updated version to $new_version in Cargo.toml"
}

# Function to update changelog
update_changelog() {
    local new_version=$1
    local temp_file=$(mktemp)
    local current_date=$(date +%Y-%m-%d)
    
    # Add new version entry at the top of the changelog
    {
        echo "# Mimic Changelog"
        echo ""
        echo "All notable, and occasionally less notable changes to this project will be documented in this file."
        echo ""
        echo "The format is based on [Keep a Changelog](http://keepachangelog.com/)"
        echo "and this project adheres to [Semantic Versioning](http://semver.org/)."
        echo ""
        echo "## [Unreleased]"
        echo ""
        echo "## [$new_version] - $(date +%Y-%m-%d)"
        echo "- Initial release"
        echo ""
        tail -n +8 "$CHANGELOG"
    } > "$temp_file"
    
    mv "$temp_file" "$CHANGELOG"
    print_success "Updated changelog with version $new_version"
}

# Function to create git tag
create_git_tag() {
    local version=$1
    
    if git tag -l "v$version" | grep -q "v$version"; then
        print_warning "Tag v$version already exists"
        return
    fi
    
    git add "$CARGO_TOML" "$CHANGELOG"
    git commit -m "Bump version to $version"
    git tag -a "v$version" -m "Release version $version"
    
    print_success "Created git tag v$version"
}

# Function to check if working directory is clean
check_clean_working_dir() {
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory is not clean. Please commit or stash your changes first."
        exit 1
    fi
}

# Function to check if tag already exists and prevent modification
check_tag_exists() {
    local version=$1
    local tag="v$version"
    
    # Check if tag exists locally
    if git tag -l "$tag" | grep -q "$tag"; then
        print_error "Tag $tag already exists locally. Cannot modify existing tags."
        print_info "If you need to update this version, you must bump to a new version."
        exit 1
    fi
    
    # Check if tag exists remotely
    if git ls-remote --tags origin "$tag" | grep -q "$tag"; then
        print_error "Tag $tag already exists remotely. Cannot modify existing tags."
        print_info "If you need to update this version, you must bump to a new version."
        exit 1
    fi
}

# Function to check if current version has uncommitted changes
check_version_changes() {
    local current_version=$(get_current_version)
    local tag="v$current_version"
    
    # Check if the current version tag exists
    if git tag -l "$tag" | grep -q "$tag" || git ls-remote --tags origin "$tag" | grep -q "$tag"; then
        # Tag exists, check if there are uncommitted changes
        if ! git diff-index --quiet HEAD --; then
            print_error "Current version $current_version is already tagged, but you have uncommitted changes."
            print_error "You must either:"
            print_info "  1. Commit your changes and bump to a new version"
            print_info "  2. Stash your changes if they're not ready"
            print_info "  3. Reset to the tagged commit if changes are unwanted"
            exit 1
        fi
        
        # Check if HEAD is at the tagged commit
        local head_commit=$(git rev-parse HEAD)
        local tag_commit=$(git rev-parse "$tag" 2>/dev/null || echo "")
        
        if [ "$head_commit" = "$tag_commit" ]; then
            print_warning "You are already at version $current_version with no changes."
            print_info "No version bump needed."
            return 0
        else
            print_error "Current version $current_version is tagged but HEAD is not at the tagged commit."
            print_error "This indicates the tag has been modified or HEAD has moved."
            print_info "You must bump to a new version to continue."
            exit 1
        fi
    fi
}

# Function to validate version bump is actually needed
validate_version_bump() {
    local new_version=$1
    local current_version=$(get_current_version)
    
    if [ "$new_version" = "$current_version" ]; then
        print_error "Cannot bump to the same version: $current_version"
        print_info "Use a different version or run 'make release' to create a release with current version."
        exit 1
    fi
}

# Function to show current version
show_version() {
    local current_version=$(get_current_version)
    print_info "Current version: $current_version"
}

# Function to list available tags
list_tags() {
    print_info "Available git tags:"
    git tag --sort=-version:refname | head -10
    echo ""
    print_info "To see all tags: git tag --sort=-version:refname"
}

# Function to show help
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  current              Show current version"
    echo "  major                Bump major version (x.0.0)"
    echo "  minor                Bump minor version (0.x.0)"
    echo "  patch                Bump patch version (0.0.x)"
    echo "  release [VERSION]    Create a release (optional version)"
    echo "  tags                 List available git tags"
    echo "  help                 Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 patch             # Bump patch version"
    echo "  $0 minor             # Bump minor version"
    echo "  $0 release           # Create release with current version"
    echo "  $0 release 1.0.0     # Create release with specific version"
    echo "  $0 tags              # List available tags for git dependencies"
}

# Main script logic
case "${1:-help}" in
    current)
        show_version
        ;;
    tags)
        list_tags
        ;;
    major|minor|patch)
        check_clean_working_dir
        check_version_changes
        
        current_version=$(get_current_version)
        new_version=$(bump_version "$current_version" "$1")
        
        validate_version_bump "$new_version"
        check_tag_exists "$new_version"
        
        print_info "Bumping version from $current_version to $new_version"
        update_cargo_version "$new_version"
        update_changelog "$new_version"
        create_git_tag "$new_version"
        
        print_success "Version bumped to $new_version"
        print_info "Run 'git push --follow-tags' to push changes and tags"
        ;;
    release)
        if [ -n "$2" ]; then
            # Specific version provided
            new_version=$2
            current_version=$(get_current_version)
            
            if [ "$current_version" != "$new_version" ]; then
                check_clean_working_dir
                check_version_changes
                validate_version_bump "$new_version"
                check_tag_exists "$new_version"
                print_info "Setting version to $new_version"
                update_cargo_version "$new_version"
                update_changelog "$new_version"
            fi
        else
            # Use current version
            new_version=$(get_current_version)
            check_version_changes
        fi
        
        check_tag_exists "$new_version"
        create_git_tag "$new_version"
        print_success "Release $new_version created"
        print_info "Run 'git push --follow-tags' to push changes and tags"
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac 