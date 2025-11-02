use crate::prelude::*;

///
/// Sanitizer
///

#[derive(Debug, FromMeta)]
pub struct Sanitizer {
    #[darling(default, skip)]
    pub def: Def,
}

impl HasDef for Sanitizer {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Sanitizer {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Sanitizer
    }
}

impl HasSchemaPart for Sanitizer {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();

        quote! {
            ::mimic::schema::node::Sanitizer {
                def: #def,
            }
        }
    }
}

impl HasTraits for Sanitizer {
    fn traits(&self) -> TraitList {
        let mut traits = Traits::default().with_default_traits();
        traits.add(Trait::Default);

        traits.list()
    }
}

impl HasType for Sanitizer {
    fn type_part(&self) -> TokenStream {
        let item = &self.def.item;

        quote!(#item)
    }
}

impl HasTypeViews for Sanitizer {}

impl ToTokens for Sanitizer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
