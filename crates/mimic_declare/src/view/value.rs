use crate::{prelude::*, view::ItemView};

///
/// ValueView
///

pub struct ValueView<'a>(pub &'a Value);

impl View for ValueView<'_> {
    type Node = Value;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ValueView<'_> {
    pub fn view_expr(&self) -> TokenStream {
        let node = self.node();
        let item_view = ItemView(&node.item);
        let item_expr = &item_view.view_expr();

        match node.cardinality() {
            Cardinality::One => quote!(#item_expr),
            Cardinality::Opt => quote!(Option<#item_expr>),
            Cardinality::Many => quote!(Vec<#item_expr>),
        }
    }
}

///
/// ValueFilter
///

pub struct ValueFilter<'a>(pub &'a Value);

impl View for ValueFilter<'_> {
    type Node = Value;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ValueFilter<'_> {
    pub fn filter_expr(&self) -> Option<TokenStream> {
        let node = self.node();

        match (node.cardinality(), node.item.target()) {
            (Cardinality::Many, _) => Some(quote!(::mimic::db::query::ContainsFilter)),
            (_, ItemTarget::Primitive(p)) => p.filter_kind().map(|f| f.as_type()),
            (_, ItemTarget::Is(_)) => None,
        }
    }
}
