use crate::{
    prelude::*,
    view::{
        ItemView, ValueView,
        traits::{View, ViewExpr},
    },
};

///
/// MapView
///

pub struct MapView<'a>(pub &'a Map);

impl View for MapView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let view_ident = node.view_ident();
        let key_view = ItemView(&node.key).expr();
        let value_view = ValueView(&node.value).expr();

        quote! {
            pub type #view_ident = Vec<(#key_view, #value_view)>;
        }
    }
}

impl ToTokens for MapView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}

///
/// MapFilter
///

pub struct MapFilter<'a>(pub &'a Map);

impl View for MapFilter<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let filter_ident = node.filter_ident();

        quote! {
            pub type #filter_ident = ::mimic::db::primitives::filter::NoFilter;
        }
    }
}

impl ToTokens for MapFilter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
