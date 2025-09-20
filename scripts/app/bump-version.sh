#!/usr/bin/env bash
set -euo pipefail

BUMP_TYPE=${1:-patch}

if ! cargo set-version --help >/dev/null 2>&1; then
  echo "❌ cargo set-version not available. Install cargo-edit or upgrade Rust." >&2
  exit 1
fi

# Current version (from [workspace.package])
PREV=$(cargo metadata --no-deps --format-version=1 \
  | jq -r '.workspace_metadata.workspace.package.version // .packages[0].version')


# Bump
cargo set-version --workspace --bump "$BUMP_TYPE" >/dev/null

# New version
NEW=$(cargo pkgid | cut -d# -f2 | cut -d: -f2)

if [[ "$PREV" == "$NEW" ]]; then
  echo "Version unchanged ($NEW)"
  exit 0
fi

# Update lockfile if present
[[ -f Cargo.lock ]] && cargo generate-lockfile >/dev/null

# Stage changes
git add Cargo.toml Cargo.lock $(git ls-files -m -- */Cargo.toml || true)

# Ensure tag doesn't already exist
if git rev-parse "v$NEW" >/dev/null 2>&1; then
  echo "❌ Tag v$NEW already exists. Aborting." >&2
  exit 1
fi

# Commit + tag + push
git commit -m "Release $NEW"
git tag -a "v$NEW" -m "Release $NEW"
git push --follow-tags

echo "✅ Bumped: $PREV → $NEW"
