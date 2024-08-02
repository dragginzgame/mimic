use crate::{
    helper::{as_self, quote_one, quote_option, quote_vec, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits, Value},
};
use darling::FromMeta;
use orm::types::Sorted;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;

///
/// Enum
///

#[derive(Debug, FromMeta)]
pub struct Enum {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,

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

        // repr
        let repr = if self.is_unit_enum() {
            quote!(#[repr(i32)])
        } else {
            quote!()
        };

        // quote
        let derive = self.derive();
        let schema = self.ctor_schema();
        let imp = self.imp();
        let variants = self.variants.iter().map(Node::expand);
        let q = quote! {
            #schema
            #derive
            #sorted
            #repr
            pub enum #ident {
                #(#variants)*
            }
            #imp
        };

        // debug
        assert!(!self.debug, "{q}");

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
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy]);
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
    pub name: Ident,

    #[darling(default)]
    pub value: Option<Value>,

    #[darling(default)]
    pub discriminant: Option<i32>,

    #[darling(default)]
    pub default: bool,

    #[darling(default)]
    pub invalid: bool,
}

impl Node for EnumVariant {
    fn expand(&self) -> TokenStream {
        let mut q = quote!();

        if self.default {
            q.extend(quote!(#[default]));
        }

        // quote
        let name = &self.name;
        q.extend(match (&self.value, &self.discriminant) {
            (Some(_), Some(_)) => panic!("cannot set both value and discriminant"),

            // value
            (Some(value), None) => quote! {
                #name(#value),
            },

            // discriminant
            (None, Some(disc)) => {
                quote! {
                    #name = #disc,
                }
            }

            // none
            (None, None) => quote! {
                #name,
            },
        });

        q
    }
}

impl Schemable for EnumVariant {
    fn schema(&self) -> TokenStream {
        let Self {
            default, invalid, ..
        } = self;
        let name = quote_one(&self.name, to_string);
        let value = quote_option(&self.value, Value::schema);
        let discriminant = quote_option(&self.discriminant, as_self);

        quote! {
            ::mimic::orm::schema::node::EnumVariant {
                name: #name,
                value : #value,
                discriminant : #discriminant,
                default: #default,
                invalid: #invalid,
            }
        }
    }
}
