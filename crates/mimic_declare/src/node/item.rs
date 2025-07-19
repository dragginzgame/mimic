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
    // if relation is Some and no type is set, we default to Ulid
    pub fn target(&self) -> ItemTarget {
        match (&self.is, &self.primitive, &self.relation) {
            (Some(path), None, _) => ItemTarget::Is(path.clone()),
            (None, Some(prim), _) => ItemTarget::Primitive(*prim),
            (None, None, Some(_)) => ItemTarget::Primitive(Primitive::Ulid),
            (None, None, None) => ItemTarget::Primitive(Primitive::Unit),
            _ => panic!("item should not have more than one target selected (is, prim, relation)"),
        }
    }

    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }

    pub const fn is_primitive(&self) -> bool {
        self.primitive.is_some()
    }
}

impl AsSchema for Item {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        let target = self.target().schema();
        let relation = quote_option(self.relation.as_ref(), to_path);
        let selector = quote_option(self.selector.as_ref(), to_path);
        let validators = quote_slice(&self.validators, TypeValidator::schema);
        let indirect = self.indirect;
        let todo = self.todo;

        quote! {
            ::mimic::schema::node::Item{
                target: #target,
                relation: #relation,
                selector: #selector,
                validators: #validators,
                indirect: #indirect,
                todo: #todo,
            }
        }
    }
}

impl AsType for Item {
    fn as_type(&self) -> Option<TokenStream> {
        let ty = self.target().as_type();

        let q = if self.indirect {
            quote!(Box<#ty>)
        } else {
            quote!(#ty)
        };

        Some(q)
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let view = self.target().as_view_type();

        let q = if self.indirect {
            quote!(Box<#view>)
        } else {
            quote!(#view)
        };

        Some(q)
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
    Primitive(Primitive),
}

impl AsSchema for ItemTarget {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        match self {
            Self::Is(path) => {
                let path = quote_one(path, to_path);
                quote! {
                    ::mimic::schema::node::ItemTarget::Is(#path)
                }
            }
            Self::Primitive(prim) => {
                quote! {
                    ::mimic::schema::node::ItemTarget::Primitive(#prim)
                }
            }
        }
    }
}

impl AsType for ItemTarget {
    fn as_type(&self) -> Option<TokenStream> {
        let q = match self {
            Self::Is(path) => quote!(#path),
            Self::Primitive(prim) => {
                let ty = prim.as_type();
                quote!(#ty)
            }
        };

        Some(q)
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let q = match self {
            Self::Is(path) => {
                quote!(<#path as ::mimic::core::traits::TypeView>::View)
            }
            Self::Primitive(prim) => {
                let ty = prim.as_type();
                quote!(<#ty as ::mimic::core::traits::TypeView>::View)
            }
        };

        Some(q)
    }
}
