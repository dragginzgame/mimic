use crate::{
    helper::{quote_one, to_path, to_str_lit},
    node::Def,
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter},
};
use darling::FromMeta;
use mimic_schema::types::StoreType;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

///
/// Store
///

#[derive(Debug, FromMeta)]
pub struct Store {
    #[darling(default, skip)]
    pub def: Def,

    pub ident: Ident,
    pub ty: StoreType,
    pub canister: Path,
    pub memory_id: u8,
}

impl AsMacro for Store {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_path_trait();
        traits.add(Trait::StoreKind);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::StoreKind => StoreKindTrait::tokens(self),
            _ => None,
        }
    }
}

impl AsSchema for Store {
    const EMIT_SCHEMA: bool = true;

    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let ident = quote_one(&self.ident, to_str_lit);
        let ty = &self.ty;
        let canister = quote_one(&self.canister, to_path);
        let memory_id = &self.memory_id;

        quote! {
            ::mimic::schema::node::SchemaNode::Store(::mimic::schema::node::Store{
                def: #def,
                ident: #ident,
                ty: #ty,
                canister: #canister,
                memory_id: #memory_id,
            })
        }
    }
}

impl AsType for Store {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();

        Some(quote! {
            pub struct #ident {}
        })
    }
}

impl ToTokens for Store {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
