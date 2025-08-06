# Mimic Integration Guide

This guide explains how to integrate Mimic into your Internet Computer project using git dependencies with version pinning.

## Quick Start

Add Mimic to your `Cargo.toml`:

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
```

## Available Versions

### Latest Stable Versions

| Version | Tag | Release Date | Breaking Changes |
|---------|-----|--------------|------------------|
| 0.9.3 | `v0.9.3` | Latest | No |
| 0.9.2 | `v0.9.2` | - | No |
| 0.9.1 | `v0.9.1` | - | No |
| 0.9.0 | `v0.9.0` | - | Yes |

### Version Compatibility

- **0.9.x**: Stable API, backward compatible within series
- **0.8.x**: Previous stable series
- **0.7.x**: Legacy series

## Integration Methods

### 1. Git Dependency with Tag (Recommended)

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
```

**Pros:**
- Exact version pinning
- Reproducible builds
- No dependency on crates.io availability
- **Immutable tags** - code at `v0.9.2` will never change
- **Security** - prevents supply chain attacks

**Cons:**
- Manual updates required
- Larger download size

### 2. Git Dependency with Branch

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", branch = "main", features = [] }
```

**Pros:**
- Always latest changes
- Automatic updates

**Cons:**
- Unstable API
- Build reproducibility issues
- Not recommended for production

### 3. Git Dependency with Commit

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", rev = "abc123...", features = [] }
```

**Pros:**
- Exact commit pinning
- Reproducible builds

**Cons:**
- Hard to track updates
- Manual commit hash management

## Features

Mimic supports several feature flags to customize functionality:

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [
    "default",           # Default features (recommended)
    "serde",            # Serde serialization support
    "icu",              # ICU internationalization
    "ulid",             # ULID support
    "decimal",          # Decimal number support
] }
```

### Feature Descriptions

- **default**: Core functionality (always enabled)
- **serde**: Enable serde serialization/deserialization
- **icu**: Internationalization support via ICU
- **ulid**: ULID (Universally Unique Lexicographically Sortable Identifier) support
- **decimal**: High-precision decimal arithmetic

## Basic Usage Example

```rust
use mimic::db::{Db, query::load};
use mimic::entity;

#[entity(
    sk(field = "id"),
    fields(
        field(name = "id", value(item(is = "types::Ulid"))),
        field(name = "name", value(item(is = "text::Name"))),
        field(name = "description", value(item(is = "text::Description"))),
    ),
)]
pub struct User {}

// Initialize database
let db = Db::new();

// Query users
let users = load::<User>(&db)
    .all()
    .execute()?
    .entities()
    .collect::<Vec<_>>();
```

## Migration Between Versions

### Upgrading from 0.8.x to 0.9.x

1. Update your dependency:
   ```toml
   mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
   ```

2. Check the [changelog](CHANGELOG.md) for breaking changes

3. Update your code according to the migration guide

### Breaking Changes in 0.9.0

- Query API changes
- Entity macro syntax updates
- Database initialization changes

## Troubleshooting

### Common Issues

#### 1. Compilation Errors

```bash
error: failed to select a version for `mimic`
```

**Solution:** Ensure the tag exists and is spelled correctly:
```bash
git ls-remote --tags https://github.com/dragginzgame/mimic
```

#### 2. Feature Not Found

```bash
error: feature `some_feature` is not available
```

**Solution:** Check available features in the [features section](#features) above.

#### 3. Version Conflicts

```bash
error: failed to resolve dependencies
```

**Solution:** Use exact version pinning with tags:
```toml
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
```

### Getting Help

1. Check the [changelog](CHANGELOG.md) for version-specific notes
2. Review the [versioning guide](VERSIONING.md) for release information
3. Open an issue on [GitHub](https://github.com/dragginzgame/mimic)

## Security

### 🔒 Tag Immutability

Mimic enforces **tag immutability** - once a version is tagged and pushed, the code at that version will never change. This ensures:

- **Reproducible builds** - `v0.9.2` always contains the same code
- **Supply chain security** - prevents malicious code injection
- **Dependency stability** - your builds won't break unexpectedly

### Security Verification

```bash
# Check if a specific version exists and is immutable
git ls-remote --tags https://github.com/dragginzgame/mimic | grep v0.9.2

# Verify the commit hash hasn't changed
git ls-remote https://github.com/dragginzgame/mimic v0.9.2
```

## Best Practices

### 1. Version Pinning

Always use tag-based dependencies for production:

```toml
# ✅ Good - pinned version
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }

# ❌ Bad - floating version
mimic = { git = "https://github.com/dragginzgame/mimic", branch = "main", features = [] }
```

### 2. Feature Selection

Only enable features you need:

```toml
# ✅ Good - minimal features
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = ["serde"] }

# ❌ Bad - unnecessary features
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = ["serde", "icu", "ulid", "decimal"] }
```

### 3. Regular Updates

Keep your dependency updated:

```bash
# Check for new versions
git ls-remote --tags https://github.com/dragginzgame/mimic | grep "v0.9"

# Update to latest patch version
# Change tag from v0.9.2 to v0.9.3
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
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }

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
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = ["test-utils"] }
```

## Version History

For a complete version history and detailed changelog, see [CHANGELOG.md](CHANGELOG.md).

### Recent Releases

- **v0.9.3**: Save queries now return the new entity
- **v0.9.2**: Query rewrite with improved validation
- **v0.9.1**: Performance improvements and metadata optimization
- **v0.9.0**: Major codegen rewrite

## Support

- **Documentation**: [docs.rs/mimic](https://docs.rs/mimic)
- **Issues**: [GitHub Issues](https://github.com/dragginzgame/mimic/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dragginzgame/mimic/discussions) 