use crate::{
    prelude::*,
    view::{
        ItemView,
        traits::{View, ViewType},
    },
};

///
/// SetView
///

pub struct SetView<'a>(pub &'a Set);

impl View for SetView<'_> {
    type Node = Set;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for SetView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let item_view = ItemView(&node.item).view_expr();

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
