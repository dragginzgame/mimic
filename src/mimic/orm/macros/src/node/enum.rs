use crate::{
    helper::{quote_one, quote_option, quote_vec, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits, Value},
};
use darling::FromMeta;
use orm::types::Sorted;
use orm_schema::Schemable;
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

    #[darling(default)]
    pub sorted: Sorted,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<EnumVariant>,

    #[darling(default)]
    pub traits: Traits,
}

impl Enum {
    pub fn is_unit_enum(&self) -> bool {
        self.variants.iter().all(|variant| variant.value.is_none())
    }

    pub fn is_orderable(&self) -> bool {
        self.is_unit_enum()
    }
}

impl Node for Enum {
    fn expand(&self) -> TokenStream {
        let Self { sorted, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let derive = self.derive();
        let schema = self.ctor_schema();
        let imp = self.imp();
        let variants = self.variants.iter().map(Node::expand);
        let q = quote! {
            #schema
            #derive
            #sorted
            pub enum #ident {
                #(#variants,)*
            }
            #imp
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
        traits.add_db_traits();

        // extra traits
        // unit enum needs both Hash and Display for hash keys
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy, Trait::EnumDisplay, Trait::Hash]);
        }
        if self.is_orderable() {
            traits.extend(vec![Trait::Ord, Trait::PartialOrd]);
        }

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Orderable => imp::orderable::enum_(self, t),
            Trait::ValidateAuto => imp::validate_auto::enum_(self, t),
            Trait::Visitable => imp::visitable::enum_(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Enum {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_vec(&self.variants, EnumVariant::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Enum(::mimic::orm::schema::node::Enum {
                def: #def,
                variants: #variants,
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
            format_ident!("Unspecified")
        } else {
            self.name.clone()
        };

        // quote
        q.extend(match &self.value {
            Some(value) => quote! (#name(#value)),
            None => quote! (#name),
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
        let name = quote_one(&self.name, to_string);
        let value = quote_option(&self.value, Value::schema);

        quote! {
            ::mimic::orm::schema::node::EnumVariant {
                name: #name,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}
