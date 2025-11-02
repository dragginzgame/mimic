use crate::{prelude::*, view::ItemView};

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
    fn view_part(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let item_view = ItemView(&node.item).view_expr();

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}
