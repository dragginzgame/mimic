use crate::{prelude::*, view::traits::View};

///
/// NewtypeView
///

pub struct NewtypeView<'a>(pub &'a Newtype);

impl View for NewtypeView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.0;
        let view_ident = node.view_ident();
        let view_type = node.item.type_expr();

        quote! {
            pub type #view_ident = <#view_type as ::mimic::core::traits::View>::ViewType;
        }
    }
}

impl ToTokens for NewtypeView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
