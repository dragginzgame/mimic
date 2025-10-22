use crate::prelude::*;

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
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
            let variant_ident = &variant.ident;

            if variant.value.is_some() {
                quote! {
                    Self::#variant_ident(v) => Self::View::#variant_ident(v.to_view())
                }
            } else {
                quote! {
                    Self::#variant_ident => Self::View::#variant_ident
                }
            }
        });

        // from_view_arms
        let from_view_arms = node.variants.iter().map(|variant| {
            let variant_ident = &variant.ident;

            if variant.value.is_some() {
                quote! {
                    Self::View::#variant_ident(v) => Self::#variant_ident(TypeView::from_view(v))
                }
            } else {
                quote! {
                    Self::View::#variant_ident => Self::#variant_ident
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
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
            let ident = variant.effective_ident();

            quote! {
                Self::#ident => Self::View::#ident
            }
        });

        // from_view_arms
        let from_view_arms = node.variants.iter().map(|variant| {
            let ident = variant.effective_ident();

            quote! {
                Self::View::#ident => Self::#ident
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
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
        let key = &node.key.type_expr();
        let value = &node.value.type_expr();

        // tokens
        let q = quote_typeview_map(view_ident, &quote!(#key), &quote!(#value));
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
        let item_ty = node.item.type_expr();
        let item_ty_for_from = item_ty.clone();
        let view_ident = &node.view_ident();

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                <#item_ty as ::mimic::core::traits::TypeView>::to_view(&self.0)
            }

            fn from_view(view: Self::View) -> Self {
                Self(<#item_ty_for_from as ::mimic::core::traits::TypeView>::from_view(view))
            }
        };

        // tokens
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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

        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
        let ident = node.def.ident();
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

        let tokens = Implementor::new(node.def(), Trait::TypeView)
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
