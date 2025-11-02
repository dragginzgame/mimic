use crate::prelude::*;

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

impl HasDef for Store {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Store {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Store
    }
}

impl HasSchemaPart for Store {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let ident = quote_one(&self.ident, to_str_lit);
        let ty = &self.ty;
        let canister = quote_one(&self.canister, to_path);
        let memory_id = &self.memory_id;

        quote! {
            ::mimic::schema::node::Store{
                def: #def,
                ident: #ident,
                ty: #ty,
                canister: #canister,
                memory_id: #memory_id,
            }
        }
    }
}

impl HasTraits for Store {
    fn traits(&self) -> TraitList {
        let mut traits = Traits::default().with_path_trait();
        traits.add(Trait::StoreKind);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::StoreKind => StoreKindTrait::strategy(self),
            _ => None,
        }
    }
}

impl HasType for Store {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();

        quote! {
            pub struct #ident;
        }
    }
}

impl HasTypeViews for Store {}

impl ToTokens for Store {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
