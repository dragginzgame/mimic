use crate::{
    helper::{quote_one, quote_option, quote_slice, to_path},
    node::TypeValidator,
    traits::{AsSchema, AsType},
};
use darling::FromMeta;
use mimic_schema::types::Primitive;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Path;

///
/// Item
///

#[derive(Clone, Debug, FromMeta)]
pub struct Item {
    #[darling(default)]
    pub is: Option<Path>,

    #[darling(default, rename = "prim")]
    pub primitive: Option<Primitive>,

    #[darling(default, rename = "rel")]
    pub relation: Option<Path>,

    #[darling(default)]
    pub selector: Option<Path>,

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,

    #[darling(default)]
    pub indirect: bool,

    #[darling(default)]
    pub todo: bool,
}

impl Item {
    pub fn target(&self) -> ItemTarget {
        match (&self.is, &self.primitive, &self.relation) {
            (Some(path), None, None) => ItemTarget::Is(path.clone()),
            (None, Some(prim), None) => ItemTarget::Prim(*prim),
            (None, None, Some(path)) => ItemTarget::Relation(path.clone()),
            (None, None, None) => ItemTarget::Prim(Primitive::Unit),
            _ => panic!("item should not have more than one target selected (is, prim, relation)"),
        }
    }

    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }
}

impl AsSchema for Item {
    fn schema(&self) -> TokenStream {
        let target = self.target().schema();
        let selector = quote_option(self.selector.as_ref(), to_path);
        let validators = quote_slice(&self.validators, TypeValidator::schema);
        let indirect = self.indirect;
        let todo = self.todo;

        quote! {
            ::mimic::schema::node::Item{
                target: #target,
                selector: #selector,
                validators: #validators,
                indirect: #indirect,
                todo: #todo,
            }
        }
    }
}

impl AsType for Item {
    fn as_type(&self) -> TokenStream {
        let ty = self.target().as_type();

        if self.indirect {
            quote!(Box<#ty>)
        } else {
            quote!(#ty)
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let view = self.target().as_view_type();

        if self.indirect {
            quote!(Box<#view>)
        } else {
            quote!(#view)
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}

///
/// ItemTarget
///

pub enum ItemTarget {
    Is(Path),
    Relation(Path),
    Prim(Primitive),
}

impl AsSchema for ItemTarget {
    fn schema(&self) -> TokenStream {
        match self {
            Self::Is(path) => {
                let path = quote_one(path, to_path);
                quote! {
                    ::mimic::schema::node::ItemTarget::Is(#path)
                }
            }
            Self::Prim(prim) => {
                quote! {
                    ::mimic::schema::node::ItemTarget::Prim(#prim)
                }
            }
            Self::Relation(path) => {
                let path = quote_one(path, to_path);
                quote! {
                    ::mimic::schema::node::ItemTarget::Relation(#path)
                }
            }
        }
    }
}

impl AsType for ItemTarget {
    fn as_type(&self) -> TokenStream {
        match self {
            Self::Is(path) => quote!(#path),
            Self::Prim(prim) => {
                let ty = prim.as_type();
                quote!(#ty)
            }
            Self::Relation(_) => quote!(::mimic::core::Reference),
        }
    }

    fn as_view_type(&self) -> TokenStream {
        match self {
            Self::Is(path) => {
                quote!(<#path as ::mimic::core::traits::TypeView>::View)
            }
            Self::Prim(prim) => {
                let ty = prim.as_type();
                quote!(<#ty as ::mimic::core::traits::TypeView>::View)
            }
            Self::Relation(_) => quote!(::mimic::core::Reference),
        }
    }
}
