use proc_macro2::TokenStream;
use quote::quote;

///
/// CratePaths
///
/// Resolves crate roots for generated code. Defaults target direct crates to
/// avoid meta-crate cycles in-workspace. Env vars allow overrides:
/// `ICYDB_CORE_CRATE`, `ICYDB_SCHEMA_CRATE`, `ICYDB_ERROR_CRATE`.
///

#[derive(Clone, Debug, Default)]
pub struct CratePaths {
    pub core: TokenStream,
    pub schema: TokenStream,
    pub error: TokenStream,
}

impl CratePaths {
    #[must_use]
    pub fn new() -> Self {
        if false {
            Self {
                core: quote!(icydb::core),
                schema: quote!(icydb::schema),
                error: quote!(icydb::error),
            }
        } else {
            Self {
                core: quote!(icydb_core),
                schema: quote!(icydb_schema),
                error: quote!(icydb_error),
            }
        }
    }
}

/// Singleton accessor for proc-macro contexts.
#[must_use]
pub fn paths() -> CratePaths {
    CratePaths::new()
}
