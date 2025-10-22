use crate::prelude::*;

///
/// EntityCreateTrait
///

pub struct EntityCreateTrait {}

///
/// Entity
///

impl Imp<Entity> for EntityCreateTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let create_ident = &node.create_ident();

        // non pk fields
        let pk_ident = &node.primary_key;
        let fields = node.fields.iter().filter(|f| f.ident != *pk_ident);

        // tokens
        let from_pairs: Vec<_> = fields
            .map(|field| {
                let ident = &field.ident;
                quote! {
                    #ident: ::mimic::core::traits::TypeView::from_view(view.#ident)
                }
            })
            .collect();

        let q = quote! {
            type Create = #create_ident;

            fn from_create_view(view: Self::Create) -> Self {
                Self {
                    #(#from_pairs),*,
                    ..Default::default()
                }
            }
        };

        let tokens = Implementor::new(node.def(), Trait::EntityCreate)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
