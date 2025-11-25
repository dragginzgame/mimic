pub mod family;
pub mod tests;
pub mod text;

// re-export the main entrypoint so callers can do:
//   coerce::coerce_basic(...)
pub use family::coerce_basic;
