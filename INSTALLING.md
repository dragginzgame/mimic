### Installing

##### Cargo.toml

This is currently how it's set up in the private Dragginz repo.  You need to add the mimic crates into your Cargo.toml.

```toml
mimic = { git = "https://github.com/dragginzgame/mimic", package = "mimic" }
mimic_base = { git = "https://github.com/dragginzgame/mimic", package = "mimic_base" }
mimic_common = { git = "https://github.com/dragginzgame/mimic", package = "mimic_common" }
mimic_derive = { git = "https://github.com/dragginzgame/mimic", package = "mimic_derive" }
```

##### Shared Crates

Then you also need to add some (unavoidable) crates as we can't change the macros they emit.

```toml
serde = { version = "1.0", default-features = false, features = ["derive"] }
snafu = "0.8"
```

## MORE COMING SOON
