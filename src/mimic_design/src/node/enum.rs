use crate::{
    helper::{quote_one, quote_option, quote_vec, to_string},
    imp::{self, Imp},
    node::{Def, MacroNode, Node, Trait, TraitNode, TraitTokens, Traits, Type, Value},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

///
/// Enum
///

#[derive(Debug, FromMeta)]
pub struct Enum {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<EnumVariant>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Enum {
    pub fn has_default(&self) -> bool {
        self.variants.iter().any(|v| v.default)
    }

    pub fn is_unit_enum(&self) -> bool {
        self.variants.iter().all(|v| v.value.is_none())
    }

    pub fn is_orderable(&self) -> bool {
        self.is_unit_enum()
    }
}

impl Node for Enum {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let variants = self.variants.iter().map(Node::expand);
        let q = quote! {
            #schema
            #derive
            pub enum #ident {
                #(#variants,)*
            }
            #impls
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for Enum {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Enum {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();

        // extra traits
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy, Trait::FromStr, Trait::Hash]);
        }
        if self.is_orderable() {
            traits.extend(vec![Trait::Ord, Trait::PartialOrd]);
        }
        if self.has_default() {
            traits.add(Trait::Default);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Orderable => imp::OrderableTrait::tokens(self, t),
            Trait::ValidateAuto => imp::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => imp::VisitableTrait::tokens(self, t),

            _ => imp::any(self, t),
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}

impl Schemable for Enum {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_vec(&self.variants, EnumVariant::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Enum(::mimic::schema::node::Enum {
                def: #def,
                variants: #variants,
                ty: #ty,
            })
        }
    }
}

///
/// EnumVariant
///

#[derive(Clone, Debug, FromMeta)]
pub struct EnumVariant {
    #[darling(default = EnumVariant::unspecified_ident)]
    pub name: Ident,

    #[darling(default)]
    pub value: Option<Value>,

    #[darling(default)]
    pub default: bool,

    #[darling(default)]
    pub unspecified: bool,
}

impl EnumVariant {
    fn unspecified_ident() -> Ident {
        format_ident!("Unspecified")
    }
}

impl Node for EnumVariant {
    fn expand(&self) -> TokenStream {
        let mut q = quote!();

        // default
        if self.default {
            q.extend(quote!(#[default]));
        }

        // name
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        // quote
        q.extend(if let Some(value) = &self.value {
            quote! (#name(#value))
        } else {
            quote! (#name)
        });

        q
    }
}

impl Schemable for EnumVariant {
    fn schema(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;

        // quote
        let name = quote_one(&self.name, to_string);
        let value = quote_option(self.value.as_ref(), Value::schema);

        quote! {
            ::mimic::schema::node::EnumVariant {
                name: #name,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}
