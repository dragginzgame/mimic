use crate::{
    prelude::*,
    view::{
        ItemView,
        traits::{View, ViewType},
    },
};

///
/// ListView
///

pub struct ListView<'a>(pub &'a List);

impl View for ListView<'_> {
    type Node = List;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for ListView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let item_view = ItemView(&node.item).view_expr();

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
