# Mimic Integration Guide

Use a pinned git tag for reproducible builds and immutable versions.

## Quick Start

Add Mimic to your `Cargo.toml`:

```toml
[dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0" }
```

### Toolchain

- Rust 1.90.0 (edition 2024). Install with:
  - `rustup toolchain install 1.90.0`
  - Ensure CI and local dev use the same toolchain.

## Integration Methods

### 1) Git dependency with tag (recommended)

Use the Quick Start snippet above (pinned tag) for production.

**Pros:**
- Exact version pinning
- Reproducible builds
- No dependency on crates.io availability
- **Immutable tags** - code at `v0.21.0` will never change
- **Security** - prevents supply chain attacks

**Cons:**
- Manual updates required
- Larger download size

### 2) Git dependency with branch

```toml
[dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", branch = "main", features = [] }
```

**Pros:**
- Always latest changes
- Automatic updates

**Cons:**
- Unstable API
- Build reproducibility issues
- Not recommended for production

### 3) Git dependency with commit

```toml
[dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", rev = "abc123...", features = [] }
```

**Pros:**
- Exact commit pinning
- Reproducible builds

**Cons:**
- Hard to track updates
- Manual commit hash management

## Optional features

```toml
[dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0", features = [
  "serde",   # serde derive/support in types
] }
```

## Basic usage

```rust
use mimic::prelude::*;

#[entity(
    sk(field = "id"),
    fields(
        field(name = "id", value(item(is = "types::Ulid"))),
        field(name = "name", value(item(is = "text::Name"))),
        field(name = "description", value(item(is = "text::Description"))),
    ),
)]
pub struct User {}

// Build a query and execute it via db()
let query = mimic::db::query::load()
    .filter(|f| f.contains("name", "ann"))
    .sort(|s| s.asc("name"))
    .limit(50);

let views: Vec<<User as mimic::core::traits::TypeView>::View> =
    db().load::<User>().execute(&query)?.views();
```

## Migration

- Check [CHANGELOG.md](CHANGELOG.md) between tags for any breaking notes.

## Troubleshooting

### Common Issues

#### 1. Compilation Errors

```bash
error: failed to select a version for `mimic`
```

**Solution:** Ensure the tag exists and is spelled correctly:
```bash
git ls-remote --tags git@github.com:dragginzgame/mimic.git
```

#### 2. Feature Not Found

```bash
error: feature `some_feature` is not available
```

**Solution:** Check available features in the [optional features](#optional-features) section above.

#### 3. Version Conflicts

```bash
error: failed to resolve dependencies
```

**Solution:** Use exact version pinning with tags (see Quick Start).

### Getting help

1. Check the [changelog](CHANGELOG.md) for version-specific notes
2. Review the [versioning guide](VERSIONING.md) for release information
3. Open an issue in this repo

## Security

### 🔒 Tag immutability

Mimic enforces **tag immutability** - once a version is tagged and pushed, the code at that version will never change. This ensures:

- **Reproducible builds** - `v0.21.0` always contains the same code
- **Supply chain security** - prevents malicious code injection
- **Dependency stability** - your builds won't break unexpectedly

### Security Verification

```bash
# Check if a specific version exists and is immutable
git ls-remote --tags git@github.com:dragginzgame/mimic.git | grep v0.21

# Verify the commit hash hasn't changed
git ls-remote git@github.com:dragginzgame/mimic.git v0.21.0
```

## Best Practices

### 1. Version Pinning

Always use tag-based dependencies for production:

```toml
# ✅ Good - pinned version
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0" }

# ❌ Bad - floating version
mimic = { git = "git@github.com:dragginzgame/mimic.git", branch = "main", features = [] }
```

### 2. Feature Selection

Only enable features you need:

```toml
# ✅ Good - minimal features
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0", features = ["serde"] }

# ❌ Bad - unnecessary features
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0", features = ["serde"] }
```

### 3. Regular Updates

Keep your dependency updated:

```bash
# Check for new versions
git ls-remote --tags git@github.com:dragginzgame/mimic.git | grep "v0.21"

# Update to latest patch version
# Change tag from v0.20.4 to v0.21.0
```

### 4. Testing

Always test after version updates:

```bash
cargo test
cargo build --target wasm32-unknown-unknown
```

## Advanced Configuration

### Workspace Dependencies

For workspace projects, add Mimic to the workspace dependencies:

```toml
[workspace.dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0" }

[workspace.members]
member1 = "crates/member1"
member2 = "crates/member2"

# In each member's Cargo.toml
[dependencies]
mimic = { workspace = true }
```

### Development Dependencies

For testing and development:

```toml
[dev-dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0" }
```

## Version History

For a complete version history and detailed changelog, see [CHANGELOG.md](CHANGELOG.md).

### Recent Releases

See this repo’s Releases page for notes and tags.

## Support

- Source: `crates/mimic` (no crates.io/docs.rs)
- **Issues**: Open an issue in this repo
- **Discussions**: Use internal channels (e.g., Slack/Teams)
