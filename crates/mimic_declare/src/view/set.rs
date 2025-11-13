use crate::{
    prelude::*,
    view::{
        ItemView,
        traits::{View, ViewExpr},
    },
};

///
/// SetView
///

pub struct SetView<'a>(pub &'a Set);

impl View for SetView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let view_ident = node.view_ident();
        let item_view = ItemView(&node.item).expr();

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}

impl ToTokens for SetView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}

///
/// SetFilter
///

pub struct SetFilter<'a>(pub &'a Set);

impl View for SetFilter<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let filter_ident = node.filter_ident();

        quote! {
            // Each Set<T> field gets a filter alias that maps to the runtime SetFilter struct.
            // This ties generated filter types to the shared runtime logic in mimic::db::query.
            pub type #filter_ident = ::mimic::db::query::SetFilter;
        }
    }
}

impl ToTokens for SetFilter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
