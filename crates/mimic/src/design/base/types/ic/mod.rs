pub mod icrc1;
pub mod icrc3;

use crate::design::prelude::*;

///
/// Memo
///

const MEMO_CONST: ::mimic::schema::node::Newtype = ::mimic::schema::node::Newtype {
    def: ::mimic::schema::node::Def {
        module_path: module_path!(),
        comments: Some("Memo"),
        ident: "Memo",
    },
    item: ::mimic::schema::node::Item {
        target: ::mimic::schema::node::ItemTarget::Primitive(
            ::mimic::schema::types::Primitive::Blob,
        ),
        relation: None,
        selector: None,
        validators: &[],
        sanitizers: &[],
        indirect: false,
    },
    default: None,
    ty: ::mimic::schema::node::Type {
        sanitizers: &[],
        validators: &[],
    },
};
#[cfg(not(target_arch = "wasm32"))]
#[:: mimic :: export :: ctor ::
ctor(anonymous, crate_path = :: mimic :: export :: ctor)]
fn __ctor() {
    ::mimic::schema::build::schema_write()
        .insert_node(::mimic::schema::node::SchemaNode::Newtype(MEMO_CONST));
}
#[derive(
    Eq,
    PartialEq,
    :: serde :: Serialize,
    Clone,
    :: serde :: Deserialize,
    :: mimic :: export :: derive_more :: DerefMut,
    :: mimic :: export ::
derive_more :: Deref,
    Debug,
    Default,
)]
#[repr(transparent)]
pub struct Memo(pub ::mimic::core::types::Blob);
pub type MemoView = <::mimic::core::types::Blob as ::mimic::core::traits::TypeView>::View;
impl PartialEq<::mimic::core::types::Blob> for Memo {
    fn eq(&self, other: &::mimic::core::types::Blob) -> bool {
        self.0 == *other
    }
}
impl PartialEq<Memo> for ::mimic::core::types::Blob {
    fn eq(&self, other: &Memo) -> bool {
        *self == other.0
    }
}
impl PartialEq<::mimic::core::types::Blob> for &Memo {
    fn eq(&self, other: &::mimic::core::types::Blob) -> bool {
        <Memo as PartialEq<::mimic::core::types::Blob>>::eq(*self, other)
    }
}
impl PartialEq<&Memo> for ::mimic::core::types::Blob {
    fn eq(&self, other: &&Memo) -> bool {
        <Self as PartialEq<Memo>>::eq(self, *other)
    }
}
impl ::mimic::core::traits::Path for Memo {
    const PATH: &'static str = concat!(module_path!(), "::", stringify!(Memo));
}
impl<T> ::mimic::core::traits::From<T> for Memo
where
    T: Into<::mimic::core::types::Blob>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}
impl ::mimic::core::traits::SanitizeAuto for Memo {}
impl ::mimic::core::traits::Visitable for Memo {
    fn drive(&self, visitor: &mut dyn ::mimic::core::visit::Visitor) {
        use ::mimic::core::visit::perform_visit;
        perform_visit(visitor, &self.0, None);
    }
    fn drive_mut(&mut self, visitor: &mut dyn ::mimic::core::visit::VisitorMut) {
        use ::mimic::core::visit::perform_visit_mut;
        perform_visit_mut(visitor, &mut self.0, None);
    }
}
impl ::mimic::core::traits::FieldValue for Memo {
    fn to_value(&self) -> ::mimic::core::value::Value {
        self.0.to_value()
    }
}
impl ::mimic::core::traits::ValidateCustom for Memo {}
impl ::mimic::core::traits::Inner<::mimic::core::types::Blob> for Memo {
    fn inner(&self) -> &::mimic::core::types::Blob {
        self.0.inner()
    }
    fn into_inner(self) -> ::mimic::core::types::Blob {
        self.0.into_inner()
    }
}
impl ::mimic::core::traits::SanitizeCustom for Memo {}
impl ::mimic::core::traits::TypeView for Memo {
    type View = MemoView;
    fn to_view(&self) -> Self::View {
        <::mimic::core::types::Blob as ::mimic::core::traits::TypeView>::to_view(&self.0)
    }
    fn from_view(view: Self::View) -> Self {
        Self(<::mimic::core::types::Blob as ::mimic::core::traits::TypeView>::from_view(view))
    }
}
impl ::mimic::core::traits::ValidateAuto for Memo {}

///
/// Payment
///

#[record(fields(
    field(ident = "recipient", value(item(prim = "Principal"))),
    field(ident = "tokens", value(item(is = "Tokens")))
))]
pub struct Payment {}

///
/// Tokens
/// always denominated in e8s
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Tokens {}
