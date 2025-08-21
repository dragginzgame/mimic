#!/bin/bash

# Version Management Script for Rust Workspaces
# Handles semantic versioning, changelog updates, and git operations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

# Bump version according to type
bump_version() {
    local current_version=$1
    local bump_type=$2

    IFS='.' read -ra VERSION_PARTS <<< "$current_version"
    local major=${VERSION_PARTS[0]}
    local minor=${VERSION_PARTS[1]}
    local patch=${VERSION_PARTS[2]}

    case $bump_type in
        "major")
            echo "$((major + 1)).0.0"
            ;;
        "minor")
            echo "$major.$((minor + 1)).0"
            ;;
        "patch")
            echo "$major.$minor.$((patch + 1))"
            ;;
        *)
            print_error "Invalid bump type: $bump_type"
            exit 1
            ;;
    esac
}

# Update version in Cargo.toml
update_cargo_version() {
    local new_version=$1
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    print_success "Updated Cargo.toml to version $new_version"
}

# Update changelog
update_changelog() {
    local new_version=$1
    local current_date=$(date +%Y-%m-%d)

    # Create changelog if it doesn't exist
    if [ ! -f "CHANGELOG.md" ]; then
        cat > CHANGELOG.md << CHANGELOG_EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [$new_version] - $current_date

CHANGELOG_EOF
        print_success "Created CHANGELOG.md"
    fi
}

# Create git tag
create_git_tag() {
    local version=$1
    local tag_name="v$version"

    # Check if tag exists locally
    if git tag -l | grep -q "^$tag_name$"; then
        print_error "Tag $tag_name already exists locally!"
        exit 1
    fi

    # Check if tag exists on remote
    if git ls-remote --tags origin "$tag_name" | grep -q "$tag_name"; then
        print_error "Tag $tag_name already exists on remote!"
        exit 1
    fi

    git tag -a "$tag_name" -m "Release version $version"
    print_success "Created git tag $tag_name"
}

# Check if working directory is clean
check_working_directory() {
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory is not clean. Please commit or stash your changes first."
        exit 1
    fi
}

# Main command handler
case "${1:-help}" in
    "current")
        print_info "Current version: $(get_current_version)"
        ;;
    "major")
        check_working_directory
        current_version=$(get_current_version)
        new_version=$(bump_version "$current_version" "major")
        print_info "Bumping major version: $current_version -> $new_version"
        update_cargo_version "$new_version"
        update_changelog "$new_version"
        git add Cargo.toml CHANGELOG.md
        git commit -m "Bump version to $new_version"
        create_git_tag "$new_version"
        print_success "Version bumped to $new_version"
        ;;
    "minor")
        check_working_directory
        current_version=$(get_current_version)
        new_version=$(bump_version "$current_version" "minor")
        print_info "Bumping minor version: $current_version -> $new_version"
        update_cargo_version "$new_version"
        update_changelog "$new_version"
        git add Cargo.toml CHANGELOG.md
        git commit -m "Bump version to $new_version"
        create_git_tag "$new_version"
        print_success "Version bumped to $new_version"
        ;;
    "patch")
        check_working_directory
        current_version=$(get_current_version)
        new_version=$(bump_version "$current_version" "patch")
        print_info "Bumping patch version: $current_version -> $new_version"
        update_cargo_version "$new_version"
        update_changelog "$new_version"
        git add Cargo.toml CHANGELOG.md
        git commit -m "Bump version to $new_version"
        create_git_tag "$new_version"
        print_success "Version bumped to $new_version"
        ;;
    "release")
        version=${2:-$(get_current_version)}
        check_working_directory
        print_info "Creating release for version $version"
        create_git_tag "$version"
        print_success "Release created for version $version"
        ;;
    "tags")
        print_info "Available git tags:"
        git tag --sort=-version:refname | head -10
        print_info "To see all tags: git tag --sort=-version:refname"
        ;;
    "help"|*)
        echo "Usage: $0 {current|major|minor|patch|release|tags}"
        echo ""
        echo "Commands:"
        echo "  current  - Show current version"
        echo "  major    - Bump major version (x.0.0)"
        echo "  minor    - Bump minor version (0.x.0)"
        echo "  patch    - Bump patch version (0.0.x)"
        echo "  release  - Create release with current version"
        echo "  tags     - List available git tags"
        echo "  help     - Show this help message"
        ;;
esac
