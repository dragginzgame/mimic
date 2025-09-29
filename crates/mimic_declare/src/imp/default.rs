use crate::prelude::*;

///
/// DefaultTrait
///

pub struct DefaultTrait {}

///
/// Entity
///

impl Imp<Entity> for DefaultTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        // If no fields have default, just derive Default
        if node.fields.iter().all(|f| f.default.is_none()) {
            return Some(TraitStrategy::from_derive(Trait::Default));
        }

        let tokens = Implementor::new(node.def(), Trait::Default)
            .set_tokens(field_list(&node.fields))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Record
///

impl Imp<Record> for DefaultTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        // If no fields have default, just derive Default
        if node.fields.iter().all(|f| f.default.is_none()) {
            return Some(TraitStrategy::from_derive(Trait::Default));
        }

        let tokens = Implementor::new(node.def(), Trait::Default)
            .set_tokens(field_list(&node.fields))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

// field_list
fn field_list(fields: &FieldList) -> TokenStream {
    let assignments = fields.iter().map(|field| {
        let ident = &field.ident;
        let expr = if let Some(default) = &field.default {
            quote!(#default.into())
        } else {
            match field.value.cardinality() {
                Cardinality::One => quote!(Default::default()),
                Cardinality::Opt => quote!(Option::default()),
                Cardinality::Many => quote!(Vec::default()),
            }
        };

        quote! { #ident: #expr }
    });

    quote! {
        fn default() -> Self {
            Self {
                #(#assignments),*
            }
        }
    }
}

///
/// Newtype
///

impl Imp<Newtype> for DefaultTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let inner = match &node.default {
            Some(arg) => quote!(#arg.into()),
            None => panic!("newtype {} is missing a default value", node.def.ident()),
        };

        // quote
        let q = quote! {
            fn default() -> Self {
                Self(#inner)
            }
        };

        let tokens = Implementor::new(node.def(), Trait::Default)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
