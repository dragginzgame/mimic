#!/bin/bash
set -euo pipefail

# Determine the current version, preferring git tags (vX.Y.Z),
# falling back to Cargo.toml if no tags exist.
current_version() {
    # Latest semver tag (strict X.Y.Z)
    if git rev-parse --git-dir >/dev/null 2>&1; then
        if latest_tag=$(git tag --sort=-version:refname | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | head -n1); then
            if [ -n "${latest_tag:-}" ]; then
                echo "${latest_tag#v}"
                return 0
            fi
        fi
    fi

    # Fallback to Cargo.toml workspace version
    if grep -q '^version = ' Cargo.toml; then
        grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
        return 0
    fi

    # Default if nothing found
    echo "0.0.0"
}

bump_version() {
    local current=$1 type=$2
    IFS='.' read -ra parts <<< "$current"
    local major=${parts[0]:-0} minor=${parts[1]:-0} patch=${parts[2]:-0}

    case "$type" in
        major) echo "$((major + 1)).0.0" ;;
        minor) echo "$major.$((minor + 1)).0" ;;
        patch) echo "$major.$minor.$((patch + 1))" ;;
        *) echo "Unknown bump type: $type" >&2; exit 1 ;;
    esac
}

create_release_commit_and_tag() {
    local new=$1

    # Refuse to overwrite existing tag
    if git rev-parse "v${new}" >/dev/null 2>&1; then
        echo "Tag v${new} already exists. Choose a new version." >&2
        exit 1
    fi

    # Empty commit to mark the release without changing files
    git commit --allow-empty -m "Release ${new}"
    git tag -a "v${new}" -m "Release ${new}"
    git push --follow-tags
}

usage() {
    echo "Usage: $0 {current|major|minor|patch}" >&2
}

case "${1:-}" in
  current)
    current_version
    ;;
  major|minor|patch)
    cur=$(current_version)
    new=$(bump_version "$cur" "$1")
    create_release_commit_and_tag "$new"
    echo "Bumped (tag only): $cur → $new"
    ;;
  *)
    usage
    exit 1
    ;;
esac
