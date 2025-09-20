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
    python3 - "$CARGO_TOML" <<'PY'
from pathlib import Path
import sys

manifest = Path(sys.argv[1])
if not manifest.exists():
    sys.exit(1)
in_workspace_package = False
for raw_line in manifest.read_text().splitlines():
    line = raw_line.strip()
    if line.startswith("["):
        in_workspace_package = line == "[workspace.package]"
        continue
    if in_workspace_package and line.startswith("version"):
        parts = line.split("=", 1)
        if len(parts) == 2:
            print(parts[1].strip().strip('"'))
            sys.exit(0)
print("", end="")
PY
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

    python3 - "$manifest" "$latest_version" <<'PY'
import sys

def parse(version: str):
    parts = [int(p) for p in version.split('.')]
    while len(parts) < 3:
        parts.append(0)
    return parts[:3]

manifest, tag = sys.argv[1], sys.argv[2]
print(manifest if parse(manifest) >= parse(tag) else tag)
PY
}

bump_version() {
    local current=$1 type=$2
    IFS='.' read -ra parts <<< "$current"
    local major=${parts[0]:-0} minor=${parts[1]:-0} patch=${parts[2]:-0}

    case "$type" in
        major)
            echo "$((major + 1)).0.0"
            ;;
        minor)
            echo "$major.$((minor + 1)).0"
            ;;
        patch)
            echo "$major.$minor.$((patch + 1))"
            ;;
        *)
            echo "Unknown bump type: $type" >&2
            exit 1
            ;;
    esac
}

apply_version_with_cargo() {
    local new_version=$1

    if ! cargo set-version --help >/dev/null 2>&1; then
        echo "'cargo set-version' is not available. Install it via 'cargo install cargo-edit' or upgrade to Rust 1.75+." >&2
        exit 1
    fi

    if ! cargo set-version --workspace "$new_version" >/dev/null; then
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
    new=$(bump_version "$cur" "$type")

    apply_version_with_cargo "$new"

    create_release_commit_and_tag "$new"
    echo "Bumped: $cur → $new"
    ;;
  *)
    usage
    exit 1
    ;;
esac
