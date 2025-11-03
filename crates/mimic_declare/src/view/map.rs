use crate::{
    prelude::*,
    view::{
        ItemView, ValueView,
        traits::{View, ViewType},
    },
};

///
/// MapView
///

pub struct MapView<'a>(pub &'a Map);

impl View for MapView<'_> {
    type Node = Map;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for MapView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let key_view = ItemView(&node.key).view_expr();
        let value_view = ValueView(&node.value).view_expr();

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
