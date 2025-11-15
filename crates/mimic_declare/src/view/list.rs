use crate::{
    prelude::*,
    view::{
        ItemView,
        traits::{View, ViewExpr},
    },
};

///
/// ListView
///

pub struct ListView<'a>(pub &'a List);

impl View for ListView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let view_ident = node.view_ident();
        let item_view = ItemView(&node.item).expr();

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}

impl ToTokens for ListView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
