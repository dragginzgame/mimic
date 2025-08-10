use crate::{
    node::{Entity, Enum, EnumValue, FieldList, List, Map, Newtype, Record, Set, Tuple},
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::{HasIdent, HasType, HasTypePart},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// TypeViewTrait
///

pub struct TypeViewTrait {}

///
/// Entity
///

impl Imp<Entity> for TypeViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let view_ident = &node.view_ident();

        // tokens
        let q = field_list(view_ident, &node.fields);
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Enum
///

impl Imp<Enum> for TypeViewTrait {
    fn strategy(node: &Enum) -> Option<TraitStrategy> {
        let view_ident = &node.view_ident();

        // to_view_arms
        let to_view_arms = node.variants.iter().map(|variant| {
            let variant_name = &variant.name;

            if variant.value.is_some() {
                quote! {
                    Self::#variant_name(v) => Self::View::#variant_name(v.to_view())
                }
            } else {
                quote! {
                    Self::#variant_name => Self::View::#variant_name
                }
            }
        });

        // from_view_arms
        let from_view_arms = node.variants.iter().map(|variant| {
            let variant_name = &variant.name;

            if variant.value.is_some() {
                quote! {
                    Self::View::#variant_name(v) => Self::#variant_name(TypeView::from_view(v))
                }
            } else {
                quote! {
                    Self::View::#variant_name => Self::#variant_name
                }
            }
        });

        let q = quote! {
                type View = #view_ident;

                fn to_view(&self) -> Self::View {
                    match self {
                        #(#to_view_arms,)*
                    }
                }

                fn from_view(view: Self::View) -> Self {
                    use ::mimic::core::traits::TypeView;

                    match view {
                        #(#from_view_arms,)*
                    }
                }
        };

        // tokens
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// EnumValue
///

impl Imp<EnumValue> for TypeViewTrait {
    fn strategy(node: &EnumValue) -> Option<TraitStrategy> {
        let view_ident = node.view_ident();

        // to_view_arms
        let to_view_arms = node.variants.iter().map(|variant| {
            let variant_name = &variant.name;

            quote! {
                Self::#variant_name => Self::View::#variant_name
            }
        });

        // from_view_arms
        let from_view_arms = node.variants.iter().map(|variant| {
            let variant_name = &variant.name;

            quote! {
                Self::View::#variant_name => Self::#variant_name
            }
        });

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                match self {
                    #(#to_view_arms,)*
                }
            }

            fn from_view(view: Self::View) -> Self {
                match view {
                    #(#from_view_arms,)*
                }
            }
        };

        // tokens
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// List
///

impl Imp<List> for TypeViewTrait {
    fn strategy(node: &List) -> Option<TraitStrategy> {
        let view_ident = &node.view_ident();

        // tokens
        let q = quote_typeview_linear(view_ident);
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Map
///

impl Imp<Map> for TypeViewTrait {
    fn strategy(node: &Map) -> Option<TraitStrategy> {
        let view_ident = &node.view_ident();
        let key = &node.key.type_part();
        let value = &node.value.type_part();

        // tokens
        let q = quote_typeview_map(view_ident, &quote!(#key), &quote!(#value));
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
///
/// Newtype
///

impl Imp<Newtype> for TypeViewTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let item = &node.item;
        let view_ident = &node.view_ident();

        let to_view = if let Some(primitive) = item.primitive
            && primitive.supports_copy()
        {
            quote!(self.0)
        } else {
            quote!(self.0.to_view())
        };

        let from_view = if item.is_primitive() {
            quote!(Self(view))
        } else {
            quote!(Self(view.into()))
        };

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                #to_view
            }

            fn from_view(view: Self::View) -> Self {
                #from_view
            }
        };

        // tokens
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Record
///

impl Imp<Record> for TypeViewTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        let view_ident = &node.view_ident();
        let q = field_list(view_ident, &node.fields);

        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Set
///

impl Imp<Set> for TypeViewTrait {
    fn strategy(node: &Set) -> Option<TraitStrategy> {
        let view_ident = &node.view_ident();

        let q = quote_typeview_linear(view_ident);
        let tokens = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Tuple
///

impl Imp<Tuple> for TypeViewTrait {
    fn strategy(node: &Tuple) -> Option<TraitStrategy> {
        let ident = node.ident();
        let view_ident = node.view_ident();

        let indices: Vec<_> = (0..node.values.len()).collect();

        let to_view_fields = indices.iter().map(|i| {
            let index = syn::Index::from(*i);
            quote! {
                ::mimic::core::traits::TypeView::to_view(&self.#index)
            }
        });

        let from_view_fields = indices.iter().map(|i| {
            let index = syn::Index::from(*i);
            quote! {
                ::mimic::core::traits::TypeView::from_view(view.#index)
            }
        });

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                (
                    #(#to_view_fields),*
                )
            }

            fn from_view(view: Self::View) -> Self {
                #ident(
                    #(#from_view_fields),*
                )
            }
        };

        let tokens = Implementor::new(ident, Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Helpers
///

// field_list
fn field_list(view_ident: &Ident, fields: &FieldList) -> TokenStream {
    let to_pairs: Vec<_> = fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            quote! {
                #ident: TypeView::to_view(&self.#ident)
            }
        })
        .collect();

    let from_pairs: Vec<_> = fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            quote! {
                #ident: TypeView::from_view(view.#ident)
            }
        })
        .collect();

    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            use ::mimic::core::traits::TypeView;

            #view_ident {
                #(#to_pairs),*
            }
        }

        fn from_view(view: Self::View) -> Self {
            use ::mimic::core::traits::TypeView;

            Self {
                #(#from_pairs),*
            }
        }
    }
}

fn quote_typeview_linear(view_ident: &Ident) -> TokenStream {
    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            use ::mimic::core::traits::TypeView;

            self.iter()
                .map(TypeView::to_view)
                .collect()
        }

        fn from_view(view: Self::View) -> Self {
            use ::mimic::core::traits::TypeView;

            Self(view.into_iter()
                .map(TypeView::from_view)
                .collect())
        }
    }
}

fn quote_typeview_map(view_ident: &Ident, key: &TokenStream, value: &TokenStream) -> TokenStream {
    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            use ::mimic::core::traits::TypeView;

            self.0.iter()
                .map(|(k, v)| (
                    TypeView::to_view(k),
                    TypeView::to_view(v),
                ))
                .collect()
        }

        fn from_view(view: Self::View) -> Self {
            use ::mimic::core::traits::TypeView;

            Self(view.into_iter()
                .map(|(k, v)| (
                    <#key as TypeView>::from_view(k),
                    <#value as TypeView>::from_view(v),
                ))
                .collect())
        }
    }
}
