### Installing

Use a pinned git tag for reproducible builds.

##### Cargo.toml

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.15.2" }

# If your types derive serde
serde = { version = "1.0", default-features = false, features = ["derive"] }
```

##### Toolchain

- Rust 1.89.0 (edition 2024). Install with: `rustup toolchain install 1.89.0`.

##### Verify tags (optional)

```bash
git ls-remote --tags https://github.com/dragginzgame/mimic | sort -V
```

For integration examples and feature flags, see INTEGRATION.md.
