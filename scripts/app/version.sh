#!/bin/bash
set -euo pipefail

get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

bump_version() {
    local current=$1 type=$2
    IFS='.' read -ra parts <<< "$current"
    local major=${parts[0]} minor=${parts[1]} patch=${parts[2]}

    case "$type" in
        major) echo "$((major + 1)).0.0" ;;
        minor) echo "$major.$((minor + 1)).0" ;;
        patch) echo "$major.$minor.$((patch + 1))" ;;
        *) echo "Unknown bump type: $type" >&2; exit 1 ;;
    esac
}

update_version() {
    local new=$1
    sed -i.bak "s/^version = \".*\"/version = \"$new\"/" Cargo.toml
    rm Cargo.toml.bak
}

case "${1:-}" in
  current)
    get_current_version
    ;;
  major|minor|patch)
    current=$(get_current_version)
    new=$(bump_version "$current" "$1")
    update_version "$new"
    git add Cargo.toml
    git commit -m "Bump version to $new"
    git tag -a "v$new" -m "Release $new"
    git push --follow-tags
    echo "Bumped: $current → $new"
    ;;
  *)
    echo "Usage: $0 {current|major|minor|patch|release}"
    ;;
esac
