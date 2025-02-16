use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    sync::{LazyLock, Mutex},
    time::SystemTime,
};
use tinyrand::{Rand, Seeded, StdRand};

//
// Create a static, lazily-initialized StdRng instance wrapped in a Mutex
//
static RNG: LazyLock<Mutex<StdRand>> = LazyLock::new(|| {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let now_u64 = u64::try_from(now).unwrap();

    Mutex::new(StdRand::seed(now_u64))
});

///
/// Schemable
///
/// Any data structure requires this trait to be part of the ctor structure
/// that populates the Schema
///
pub trait Schemable {
    // schema
    // generates the structure which is passed to the static Schema data structure
    // via the ctor crate
    fn schema(&self) -> TokenStream;

    // ctor_schema
    // formats the code needed to send something via ctor to the schema
    #[must_use]
    fn ctor_schema(&self) -> TokenStream {
        let mut rng = RNG.lock().expect("Failed to lock RNG");
        let ctor_fn = format_ident!("ctor_{}", rng.next_u32());

        let schema = self.schema();

        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[ctor::ctor]
            fn #ctor_fn() {
                ::mimic::schema::build::schema_write().insert_node(
                    #schema
                );
            }
        }
    }
}
