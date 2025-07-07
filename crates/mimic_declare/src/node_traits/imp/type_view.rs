use crate::{
    node::{Entity, Enum, EnumValue, FieldList, List, Map, Newtype, Record, Set, Tuple},
    node_traits::{Imp, Implementor, Trait},
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
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let tokens = field_list(view_ident, &node.fields);

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(tokens)
                .to_token_stream(),
        )
    }
}

///
/// Enum
///

impl Imp<Enum> for TypeViewTrait {
    fn tokens(node: &Enum, t: Trait) -> Option<TokenStream> {
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
                    Self::View::#variant_name(v) => Self::#variant_name(::mimic::core::traits::TypeView::from_view(v))
                }
            } else {
                quote! {
                    Self::View::#variant_name => Self::#variant_name
                }
            }
        });

        let view_ident = node.def.view_ident();
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

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}

///
/// EnumValue
///

impl Imp<EnumValue> for TypeViewTrait {
    fn tokens(node: &EnumValue, t: Trait) -> Option<TokenStream> {
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

        let view_ident = node.def.view_ident();
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

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}

///
/// List
///

impl Imp<List> for TypeViewTrait {
    fn tokens(node: &List, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let q = quote_typeview_linear(&view_ident, quote!(Self));

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}
///
/// Map
///

impl Imp<Map> for TypeViewTrait {
    fn tokens(node: &Map, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let key = &node.key;
        let value = &node.value;
        let q = quote_typeview_map(&view_ident, &quote!(#key), &quote!(#value));

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}
///
/// Newtype
///

impl Imp<Newtype> for TypeViewTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                #view_ident(self.inner())
            }

            fn from_view(view: Self::View) -> Self {
                Self(view.0.into())
            }
        };

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}

///
/// Record
///

impl Imp<Record> for TypeViewTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let tokens = field_list(view_ident, &node.fields);

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(tokens)
                .to_token_stream(),
        )
    }
}

///
/// Set
///

impl Imp<Set> for TypeViewTrait {
    fn tokens(node: &Set, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let q = quote_typeview_linear(&view_ident, quote!(Self));

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}

///
/// Tuple
///

impl Imp<Tuple> for TypeViewTrait {
    fn tokens(node: &Tuple, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let self_ident = &node.def.ident;

        // Number of tuple values
        let values = node.values.len();

        // Create bindings: f0, f1, ...
        let field_idents: Vec<syn::Ident> = (0..values)
            .map(|i| syn::Ident::new(&format!("f{i}"), proc_macro2::Span::call_site()))
            .collect();

        // Accessor expressions: self.0, self.1, ...
        let self_fields: Vec<proc_macro2::TokenStream> = (0..values)
            .map(|i| {
                let index = syn::Index::from(i);
                quote! { ::mimic::core::traits::TypeView::to_view(&self.#index) }
            })
            .collect();

        // Reverse conversion expressions
        let view_fields: Vec<proc_macro2::TokenStream> = field_idents
            .iter()
            .map(|ident| {
                quote! { ::mimic::core::traits::TypeView::from_view(#ident) }
            })
            .collect();

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                #view_ident(
                    #(#self_fields),*
                )
            }

            fn from_view(view: Self::View) -> Self {
                let #view_ident( #(#field_idents),* ) = view;
                #self_ident(
                    #(#view_fields),*
                )
            }
        };

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}
///
/// Helpers
///

// field_list
fn field_list(view_ident: Ident, fields: &FieldList) -> TokenStream {
    let to_pairs: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            quote! {
                #name: ::mimic::core::traits::TypeView::to_view(&self.#name)
            }
        })
        .collect();

    let from_pairs: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            quote! {
                #name: ::mimic::core::traits::TypeView::from_view(view.#name)
            }
        })
        .collect();

    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            #view_ident {
                #(#to_pairs),*
            }
        }

        fn from_view(view: Self::View) -> Self {
            Self {
                #(#from_pairs),*
            }
        }
    }
}

fn quote_typeview_linear(view_ident: &Ident, wrap_self: TokenStream) -> TokenStream {
    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            let inner = self.0.iter()
                .map(|v| ::mimic::core::traits::TypeView::to_view(v))
                .collect();

            #view_ident(inner)
        }

        fn from_view(view: Self::View) -> Self {
            #wrap_self(view.0.into_iter()
                .map(|v| ::mimic::core::traits::TypeView::from_view(v))
                .collect())
        }
    }
}

fn quote_typeview_map(view_ident: &Ident, key: &TokenStream, value: &TokenStream) -> TokenStream {
    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            let inner = self.0.iter()
                .map(|(k, v)| (
                    ::mimic::core::traits::TypeView::to_view(k),
                    ::mimic::core::traits::TypeView::to_view(v),
                ))
                .collect();

            #view_ident(inner)
        }

        fn from_view(view: Self::View) -> Self {
            Self(view.0.into_iter()
                .map(|(k, v)| (
                    <#key as ::mimic::core::traits::TypeView>::from_view(k),
                    <#value as ::mimic::core::traits::TypeView>::from_view(v),
                ))
                .collect())
        }
    }
}
