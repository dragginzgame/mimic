use crate::{prelude::*, view::TupleView};

///
/// Tuple
///

#[derive(Debug, Default, FromMeta)]
pub struct Tuple {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "value")]
    pub values: Vec<Value>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl HasDef for Tuple {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Tuple {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Tuple
    }
}

impl HasSchemaPart for Tuple {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let values = quote_slice(&self.values, Value::schema_part);
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Tuple {
                def: #def,
                values: #values,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Tuple {
    fn traits(&self) -> TraitList {
        self.traits.clone().with_type_traits().list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::View => ViewTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for Tuple {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let values = self.values.iter().map(HasTypeExpr::type_expr);

        quote! {
            pub struct #ident(pub #(#values),*);
        }
    }
}

impl HasViews for Tuple {
    fn view_parts(&self) -> Vec<TokenStream> {
        vec![TupleView(self).view_part()]
    }
}

impl ToTokens for Tuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
