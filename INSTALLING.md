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

Then you also need to add some (unavoidable) crates as we can't change the macros they emit (as far as I am aware - still working on that.)

```toml
derive_more = "0.99"
serde = { version = "1.0", default-features = false, features = ["derive"] }
snafu = "0.8"
strum = { version = "0.26", features = ["derive"] }
```

##### Setting up `mimicli`

In order to work `mimicli` needs to read both your local design crate and the mimic_base crate.  We've put this code in `tools\mimicli` and added that crate to our Cargo.toml.

```rust
pub use design;
pub use mimic_base;

// main
fn main() {
    mimic::cli::run();
}
```

## MORE COMING SOON
