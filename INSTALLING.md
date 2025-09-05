### Installing

Use a pinned git tag for reproducible builds.

##### Cargo.toml

```toml
[dependencies]
mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.17.0" }

# If your types derive serde
serde = { version = "1.0", default-features = false, features = ["derive"] }
```

##### Toolchain

- Rust 1.89.0 (edition 2024). Install with: `rustup toolchain install 1.89.0`.

##### Verify tags (optional)

```bash
git ls-remote --tags git@github.com:dragginzgame/mimic.git | sort -V
```

For integration examples and feature flags, see INTEGRATION.md.
