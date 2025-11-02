use crate::{prelude::*, view::ItemView};

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
    fn view_part(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let item_view = ItemView(&node.item).view_expr();

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}
