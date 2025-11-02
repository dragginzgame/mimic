use crate::prelude::*;

///
/// ItemView
///

pub struct ItemView<'a>(pub &'a Item);

impl View for ItemView<'_> {
    type Node = Item;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ItemView<'_> {
    pub fn view_expr(&self) -> TokenStream {
        let node = self.node();
        let ty = node.target().type_expr();

        quote!(<#ty as ::mimic::core::traits::View>::ViewType)
    }
}
