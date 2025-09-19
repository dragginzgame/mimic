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
    if git rev-parse --git-dir >/dev/null 2>&1; then
        if latest_tag=$(git tag --sort=-version:refname | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | head -n1); then
            if [ -n "${latest_tag:-}" ]; then
                echo "${latest_tag#v}"
                return 0
            fi
        fi
    fi
    workspace_manifest_version
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

update_workspace_manifest_version() {
    local new_version=$1
    python3 - "$CARGO_TOML" "$new_version" <<'PY'
from pathlib import Path
import sys

manifest = Path(sys.argv[1])
new_version = sys.argv[2]
text = manifest.read_text().splitlines()
output = []
in_workspace_package = False
updated = False
for raw_line in text:
    line = raw_line.strip()
    if line.startswith("["):
        in_workspace_package = line == "[workspace.package]"
        output.append(raw_line)
        continue
    if in_workspace_package and line.startswith("version"):
        indent = raw_line[: len(raw_line) - len(raw_line.lstrip())]
        output.append(f"{indent}version = \"{new_version}\"")
        updated = True
        continue
    output.append(raw_line)

if not updated:
    raise SystemExit("version field not found in [workspace.package]")

manifest.write_text("\n".join(output) + "\n")
PY
}

update_lockfile_versions() {
    local new_version=$1
    if [ ! -f "${CARGO_LOCK}" ]; then
        return 0
    fi
    python3 - "$CARGO_LOCK" "$ROOT_DIR" "$new_version" <<'PY'
from pathlib import Path
import subprocess
import sys
import json

lock_path = Path(sys.argv[1])
root_dir = Path(sys.argv[2])
new_version = sys.argv[3]

result = subprocess.run(
    ["cargo", "metadata", "--format-version", "1", "--no-deps"],
    cwd=root_dir,
    check=True,
    capture_output=True,
    text=True,
)
metadata = json.loads(result.stdout)
workspace_ids = set(metadata["workspace_members"])
workspace_names = {pkg["name"] for pkg in metadata["packages"] if pkg["id"] in workspace_ids}

lines = lock_path.read_text().splitlines()
changed = False
index = 0
while index < len(lines):
    line = lines[index]
    if line.startswith("name = \""):
        name = line.split('"')[1]
        if name in workspace_names:
            j = index + 1
            while j < len(lines) and not lines[j].startswith("version = "):
                j += 1
            if j < len(lines):
                lines[j] = f'version = "{new_version}"'
                changed = True
    index += 1

if changed:
    lock_path.write_text("\n".join(lines) + "\n")
PY
}

stage_release_artifacts() {
    git add "$CARGO_TOML"
    if [ -f "$CARGO_LOCK" ]; then
        git add "$CARGO_LOCK"
    fi
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

    update_workspace_manifest_version "$new"
    update_lockfile_versions "$new"

    create_release_commit_and_tag "$new"
    echo "Bumped: $cur → $new"
    ;;
  *)
    usage
    exit 1
    ;;
esac
