#!/bin/bash
set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "${ROOT_DIR}"

CARGO_TOML="${ROOT_DIR}/Cargo.toml"
CARGO_LOCK="${ROOT_DIR}/Cargo.lock"

if [ ! -f "${CARGO_TOML}" ]; then
    echo "Cargo.toml not found at ${CARGO_TOML}" >&2
    exit 1
fi

workspace_manifest_version() {
    awk '
        /^\[workspace\.package\]/ { in_section=1; next }
        /^\[/ { in_section=0 }
        in_section {
            if (match($0, /version[[:space:]]*=[[:space:]]*"([^"]+)"/, m)) {
                print m[1]
                exit
            }
        }
    ' "$CARGO_TOML"
}

current_version() {
    local manifest
    manifest=$(workspace_manifest_version || true)

    local latest_tag=""
    if git rev-parse --git-dir >/dev/null 2>&1; then
        latest_tag=$(git tag --sort=-version:refname | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | head -n1)
    fi

    local latest_version="${latest_tag#v}"

    if [ -z "${manifest}" ]; then
        echo "${latest_version}"
        return 0
    fi

    if [ -z "${latest_version}" ]; then
        echo "${manifest}"
        return 0
    fi

    if [ "$manifest" = "$latest_version" ]; then
        echo "$manifest"
        return 0
    fi

    printf '%s\n%s\n' "$manifest" "$latest_version" | LC_ALL=C sort -V | tail -n1
}

apply_version_with_cargo_bump() {
    local bump_kind=$1

    if ! cargo set-version --help >/dev/null 2>&1; then
        echo "'cargo set-version' is not available. Install it via 'cargo install cargo-edit' or upgrade to Rust 1.75+." >&2
        exit 1
    fi

    if ! cargo set-version --workspace --bump "$bump_kind" >/dev/null; then
        echo "cargo set-version failed. Ensure Rust 1.75+ is installed or cargo-edit is available." >&2
        exit 1
    fi

    if [ -f "$CARGO_LOCK" ]; then
        cargo generate-lockfile >/dev/null
    fi
}

stage_release_artifacts() {
    git add "$CARGO_TOML"
    if [ -f "$CARGO_LOCK" ]; then
        git add "$CARGO_LOCK"
    fi
    git ls-files -m -- '*/Cargo.toml' | xargs -r git add
}

create_release_commit_and_tag() {
    local new=$1

    if git rev-parse "v${new}" >/dev/null 2>&1; then
        echo "Tag v${new} already exists. Choose a new version." >&2
        exit 1
    fi

    stage_release_artifacts
    git commit -m "Release ${new}"
    git tag -a "v${new}" -m "Release ${new}"
    git push --follow-tags
}

usage() {
    echo "Usage: $0 {current|major|minor|patch}" >&2
}

type=${1:-}
case "$type" in
  current)
    current_version
    ;;
  major|minor|patch)
    cur=$(current_version)
    apply_version_with_cargo_bump "$type"
    new=$(workspace_manifest_version)
    if [ -z "$new" ]; then
        echo "Failed to determine new version after bump." >&2
        exit 1
    fi

    create_release_commit_and_tag "$new"
    echo "Bumped: $cur → $new"
    ;;
  *)
    usage
    exit 1
    ;;
esac
