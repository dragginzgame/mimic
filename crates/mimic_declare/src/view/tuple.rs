use crate::{
    prelude::*,
    view::{
        ValueView,
        traits::{View, ViewExpr},
    },
};

///
/// TupleView
///

pub struct TupleView<'a>(pub &'a Tuple);

impl View for TupleView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let view_ident = node.view_ident();
        let view_values = node.values.iter().map(|v| ValueView(v).expr());

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

///
/// TupleFilter
///

pub struct TupleFilter<'a>(pub &'a Tuple);

impl View for TupleFilter<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let filter_ident = node.filter_ident();

        quote! {
            pub type #filter_ident = ::mimic::db::primitives::filter::NoFilter;
        }
    }
}

impl ToTokens for TupleFilter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
