use crate::prelude::*;

///
/// HasType
///
/// A node that emits a Rust type definition.
///

pub trait HasType: HasDef {
    /// Emit the main Rust type definition (struct, enum, etc.)
    fn type_part(&self) -> TokenStream {
        quote!()
    }
}

///
/// HasTypeExpr
///

pub trait HasTypeExpr {
    fn type_expr(&self) -> TokenStream {
        quote!()
    }
}

///
/// HasTypeViews
///

pub trait HasTypeViews: HasType {
    fn view_parts(&self) -> Vec<TokenStream> {
        vec![]
    }
}

///
/// HasTraits
///
/// Describes which traits a schema node implements or derives,
/// and provides default strategies for common trait patterns.
///
/// This layer is responsible only for *trait selection* and *impl generation logic*,
/// not for assembling the final macro output.
///

pub trait HasTraits: HasType + ToTokens {
    /// List of traits this node participates in (either derived or implemented).
    fn traits(&self) -> TraitList {
        TraitList::new()
    }

    /// Map a specific trait to a custom implementation.
    /// Return `None` to use the `default_strategy` fallback.
    fn map_trait(&self, _: Trait) -> Option<TraitStrategy> {
        None
    }

    /// Emit a custom `#[attribute(...)]` for this trait.
    fn map_attribute(&self, _: Trait) -> Option<TokenStream> {
        None
    }

    /// Provides built-in fallback strategies for common trait types.
    ///
    /// Most schema nodes rely on these automatically unless overridden in `map_trait`.
    fn default_strategy(&self, tr: Trait) -> Option<TraitStrategy> {
        let def = self.def();
        let ident = def.ident();

        match tr {
            // ─────────────────────────────
            // Inline constant path metadata
            // ─────────────────────────────
            Trait::Path => {
                let q = quote! {
                    const PATH: &'static str = concat!(module_path!(), "::", stringify!(#ident));
                };
                let tokens = Implementor::new(def, tr).set_tokens(q).to_token_stream();

                Some(TraitStrategy::from_impl(tokens))
            }

            // ─────────────────────────────
            // Marker traits — empty impls
            // ─────────────────────────────
            Trait::CanisterKind
            | Trait::EntityIdKind
            | Trait::FieldValue
            | Trait::SanitizeAuto
            | Trait::SanitizeCustom
            | Trait::ValidateAuto
            | Trait::ValidateCustom
            | Trait::Visitable => {
                let tokens = Implementor::new(def, tr).to_token_stream();
                Some(TraitStrategy::from_impl(tokens))
            }

            _ => None,
        }
    }
}
