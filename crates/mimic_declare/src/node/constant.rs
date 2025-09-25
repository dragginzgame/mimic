use crate::prelude::*;

///
/// Constant
///

#[derive(Debug, FromMeta)]
pub struct Constant {
    #[darling(default, skip)]
    pub def: Def,

    pub ty: ConstantType,
    pub value: Arg,
}

impl HasDef for Constant {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Constant {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Constant
    }
}

impl HasSchemaPart for Constant {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let ty = &self.ty;
        let value = &self.value.schema_part();

        quote! {
            ::mimic::schema::node::Constant {
                def: #def,
                ty: #ty,
                value: #value,
            }
        }
    }
}

impl HasTraits for Constant {}

impl HasType for Constant {}

impl HasTypePart for Constant {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let ty = &self.ty.as_type();
        let value = &self.value;

        quote! {
            pub const #ident: #ty = #value;
        }
    }
}

impl ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
