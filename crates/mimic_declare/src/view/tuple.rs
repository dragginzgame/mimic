use crate::{prelude::*, view::ValueView};

///
/// TupleView
///

pub struct TupleView<'a>(pub &'a Tuple);

impl View for TupleView<'_> {
    type Node = Tuple;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for TupleView<'_> {
    fn view_part(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let view_values = node.values.iter().map(|v| ValueView(v).view_expr());

        quote! {
            pub type #view_ident = (#(#view_values),*);
        }
    }
}
