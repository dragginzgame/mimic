use crate::{
    node::{Entity, Enum, EnumValue, FieldList, List, Map, Newtype, Record, Set, Tuple},
    node_traits::{Imp, Implementor, Trait},
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
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let view_ident = &node.view_ident();

        // tokens
        let q = field_list(view_ident, &node.fields);
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// Enum
///

impl Imp<Enum> for TypeViewTrait {
    fn tokens(node: &Enum) -> Option<TokenStream> {
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
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// EnumValue
///

impl Imp<EnumValue> for TypeViewTrait {
    fn tokens(node: &EnumValue) -> Option<TokenStream> {
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
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// List
///

impl Imp<List> for TypeViewTrait {
    fn tokens(node: &List) -> Option<TokenStream> {
        let view_ident = &node.view_ident();

        // tokens
        let q = quote_typeview_linear(view_ident);
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// Map
///

impl Imp<Map> for TypeViewTrait {
    fn tokens(node: &Map) -> Option<TokenStream> {
        let view_ident = &node.view_ident();
        let key = &node.key.type_part();
        let value = &node.value.type_part();

        // tokens
        let q = quote_typeview_map(view_ident, &quote!(#key), &quote!(#value));
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}
///
/// Newtype
///

impl Imp<Newtype> for TypeViewTrait {
    fn tokens(node: &Newtype) -> Option<TokenStream> {
        let view_ident = &node.view_ident();

        let from_view = if node.item.is_primitive() {
            quote!(Self(view))
        } else {
            quote!(Self(view.into()))
        };

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                self.0.to_view()
            }

            fn from_view(view: Self::View) -> Self {
                #from_view
            }
        };

        // tokens
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// Record
///

impl Imp<Record> for TypeViewTrait {
    fn tokens(node: &Record) -> Option<TokenStream> {
        let view_ident = &node.view_ident();
        let q = field_list(view_ident, &node.fields);

        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// Set
///

impl Imp<Set> for TypeViewTrait {
    fn tokens(node: &Set) -> Option<TokenStream> {
        let view_ident = &node.view_ident();

        let q = quote_typeview_linear(view_ident);
        let type_view = Implementor::new(node.ident(), Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
    }
}

///
/// Tuple
///

impl Imp<Tuple> for TypeViewTrait {
    fn tokens(node: &Tuple) -> Option<TokenStream> {
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

        let type_view = Implementor::new(ident, Trait::TypeView)
            .set_tokens(q)
            .to_token_stream();

        Some(quote! {
            #type_view
        })
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
