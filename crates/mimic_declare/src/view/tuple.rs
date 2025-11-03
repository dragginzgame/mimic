use crate::{
    prelude::*,
    view::{
        ValueView,
        traits::{View, ViewType},
    },
};

///
/// TupleView
///

pub struct TupleView<'a>(pub &'a Tuple);

impl View for TupleView<'_> {
    type Node = Tuple;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for TupleView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let view_values = node.values.iter().map(|v| ValueView(v).view_expr());

        quote! {
            pub type #view_ident = (#(#view_values),*);
        }
    }
}

impl ToTokens for TupleView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
