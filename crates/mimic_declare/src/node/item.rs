use crate::prelude::*;

///
/// Item
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Item {
    #[darling(default)]
    pub is: Option<Path>,

    #[darling(default, rename = "prim")]
    pub primitive: Option<Primitive>,

    #[darling(default, rename = "rel")]
    pub relation: Option<Path>,

    #[darling(default)]
    pub selector: Option<Path>,

    #[darling(multiple, rename = "sanitizer")]
    pub sanitizers: Vec<TypeSanitizer>,

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,

    #[darling(default)]
    pub indirect: bool,
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

    pub fn created_at() -> Self {
        Self {
            primitive: Some(Primitive::Timestamp),
            sanitizers: vec![TypeSanitizer::new(
                "mimic::design::base::sanitizer::time::CreatedAt",
                Args::none(),
            )],
            ..Default::default()
        }
    }

    pub fn updated_at() -> Self {
        Self {
            primitive: Some(Primitive::Timestamp),
            sanitizers: vec![TypeSanitizer::new(
                "mimic::design::base::sanitizer::time::UpdatedAt",
                Args::none(),
            )],
            ..Default::default()
        }
    }

    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }

    pub const fn is_primitive(&self) -> bool {
        self.primitive.is_some()
    }
}

impl HasSchemaPart for Item {
    fn schema_part(&self) -> TokenStream {
        let target = self.target().schema_part();
        let relation = quote_option(self.relation.as_ref(), to_path);
        let selector = quote_option(self.selector.as_ref(), to_path);
        let validators = quote_slice(&self.validators, TypeValidator::schema_part);
        let sanitizers = quote_slice(&self.sanitizers, TypeSanitizer::schema_part);
        let indirect = self.indirect;

        quote! {
            ::mimic::schema::node::Item{
                target: #target,
                relation: #relation,
                selector: #selector,
                validators: #validators,
                sanitizers: #sanitizers,
                indirect: #indirect,
            }
        }
    }
}

impl HasTypePart for Item {
    fn type_part(&self) -> TokenStream {
        let ty = self.target().type_part();

        if self.indirect {
            quote!(Box<#ty>)
        } else {
            quote!(#ty)
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let view = self.target().view_type_part();

        if self.indirect {
            quote!(Box<#view>)
        } else {
            quote!(#view)
        }
    }
}

///
/// ItemTarget
///

pub enum ItemTarget {
    Is(Path),
    Primitive(Primitive),
}

impl HasSchemaPart for ItemTarget {
    fn schema_part(&self) -> TokenStream {
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

impl HasTypePart for ItemTarget {
    fn type_part(&self) -> TokenStream {
        match self {
            Self::Is(path) => quote!(#path),
            Self::Primitive(prim) => {
                let ty = prim.as_type();
                quote!(#ty)
            }
        }
    }

    fn view_type_part(&self) -> TokenStream {
        match self {
            Self::Is(path) => {
                quote!(<#path as ::mimic::core::traits::TypeView>::View)
            }
            Self::Primitive(prim) => {
                let ty = prim.as_type();
                quote!(<#ty as ::mimic::core::traits::TypeView>::View)
            }
        }
    }
}
