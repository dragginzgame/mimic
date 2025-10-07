use crate::prelude::*;

///
/// InnerTrait
///

pub struct InnerTrait {}

///
/// EntityId
///

impl Imp<Newtype> for InnerTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let primitive = &node.primitive.as_type();

        let q = quote! {
            fn inner(&self) -> &#primitive {
                self.0.inner()
            }

            fn into_inner(self) -> #primitive {
                self.0.into_inner()
            }
        };

        let tokens = Implementor::new(node.def(), Trait::Inner)
            .add_trait_generic(quote!(#primitive))
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
