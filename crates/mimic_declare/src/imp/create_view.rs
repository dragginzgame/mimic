use crate::prelude::*;

///
/// CreateViewTrait
///

pub struct CreateViewTrait {}

///
/// Entity
///

impl Imp<Entity> for CreateViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let create_ident = &node.create_ident();

        // tokens
        let q = quote! {
            type View = #create_ident;
        };

        let tokens = Implementor::new(node.def(), Trait::CreateView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
