# Mimic Release Guide

This guide covers the complete workflow for developing, versioning, and releasing Mimic, including how users can integrate it as a git dependency.

## 🚀 Quick Start for Users

### Integration as Git Dependency

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
```

### Check Available Versions

```bash
# From your project directory
git ls-remote --tags https://github.com/dragginzgame/mimic | grep "v0.9"

# Or use our helper script (if you have the repo cloned)
./scripts/app/check-git-versions.sh
```

## 📋 Development Workflow

### 1. Development Phase

```bash
# Run tests
make test

# Check code quality
make clippy
make fmt-check

# Build for wasm target
make build-wasm
```

### 2. Prepare for Release

```bash
# Update changelog with your changes
# Edit CHANGELOG.md and add entries under [Unreleased]

# Check current version
make version

# List available tags
make tags
```

### 3. Create Release

```bash
# Bump version (choose one)
make patch    # 0.9.3 -> 0.9.4
make minor    # 0.9.3 -> 0.10.0
make major    # 0.9.3 -> 1.0.0

# Or create a specific version
./scripts/app/version.sh release 1.0.0
```

### 4. Push Release

```bash
# Push changes and tags
git push --follow-tags
```

This triggers:
- ✅ Automated testing
- ✅ Building for all targets
- ✅ Creating GitHub release

## 🔧 Version Management Commands

### Show Information

```bash
make version          # Show current version
make tags             # List available git tags
make check-versioning # Verify system setup
```

### Bump Versions

```bash
make patch            # 0.9.3 -> 0.9.4
make minor            # 0.9.3 -> 0.10.0
make major            # 0.9.3 -> 1.0.0
```

### Create Releases

```bash
make release          # Create release with current version
./scripts/app/version.sh release 1.0.0  # Specific version
```

## 📦 Release Workflow

### Automated Release

When you push a version tag, GitHub Actions automatically:

1. **Tests** the codebase
2. **Builds** for all targets
3. **Creates** a GitHub release with changelog notes

## 🏷️ Git Tag Management

### Creating Tags

Tags are automatically created by the version script:

```bash
# This creates both a commit and a tag
make patch
```

### Managing Tags

```bash
# List all tags
git tag --sort=-version:refname

# Delete a tag (if needed)
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# Push specific tag
git push origin v1.0.0
```

## 📝 Changelog Management

### Adding Changes

Always update the changelog before releasing:

```markdown
## [Unreleased]
- Added new feature X
- Fixed bug in Y
- Breaking: Changed API for Z
```

### Changelog Format

Follow [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
## [1.0.0] - 2024-01-15
### Added
- New feature X

### Changed
- Updated API Y

### Fixed
- Bug in Z

### Breaking
- Removed deprecated function
```

## 🔍 Quality Assurance

### Pre-Release Checklist

- [ ] All tests pass: `make test`
- [ ] Code formatting: `make fmt-check`
- [ ] Linting: `make clippy`
- [ ] WASM build: `make build-wasm`
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Working directory clean

### Post-Release Verification

- [ ] GitHub Actions passed
- [ ] GitHub release created
- [ ] Users can access new version

## 🛠️ Troubleshooting

### Common Issues

#### Version Already Exists

```bash
# Delete existing tag
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# Create new version
make patch
```

#### Release Creation Fails

1. Check GitHub Actions workflow
2. Verify tag was pushed correctly
3. Ensure changelog format is correct

#### Git Dependency Issues

```bash
# Check if tag exists
git ls-remote --tags https://github.com/dragginzgame/mimic | grep v1.0.0

# Verify repository access
git ls-remote https://github.com/dragginzgame/mimic
```

## 📚 Integration Examples

### Basic Integration

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
```

### With Features

```toml
[dependencies]
mimic = { 
    git = "https://github.com/dragginzgame/mimic", 
    tag = "v0.9.2", 
    features = ["serde", "ulid"] 
}
```

### Development Version

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", branch = "main", features = [] }
```

### Workspace Integration

```toml
[workspace.dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }

[dependencies]
mimic = { workspace = true }
```

## 🎯 Best Practices

### For Maintainers

1. **Always test** before releasing
2. **Update changelog** with every change
3. **Use semantic versioning** correctly
4. **Tag releases** immediately after pushing
5. **Monitor CI/CD** pipeline

### For Users

1. **Pin versions** with tags, not branches
2. **Test updates** before deploying
3. **Check changelog** for breaking changes
4. **Use minimal features** for better performance

## 📖 Additional Resources

- [Versioning Guide](VERSIONING.md) - Detailed versioning information
- [Integration Guide](INTEGRATION.md) - Complete integration documentation
- [Changelog](CHANGELOG.md) - Version history and changes
- [Contributing Guide](CONTRIBUTING.md) - How to contribute

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/dragginzgame/mimic/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dragginzgame/mimic/discussions)
- **Documentation**: [docs.rs/mimic](https://docs.rs/mimic) 