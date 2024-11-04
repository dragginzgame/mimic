pub use crate::orm::{base::types, prelude::*};

///
/// Duration
///
/// Duration in seconds
///

#[cfg(not(target_arch = "wasm32"))]
#[::mimic::export::ctor::ctor]
fn ctor_3822332542() {
    ::mimic::orm::schema::build::schema_write().add_node(
        ::mimic::orm::schema::node::SchemaNode::Newtype(::mimic::orm::schema::node::Newtype {
            def: ::mimic::orm::schema::node::Def {
                module_path: module_path!().to_string(),
                comments: "Duration\nDuration in seconds".to_string(),
                ident: "Duration".to_string(),
            },
            value: ::mimic::orm::schema::node::Value {
                cardinality: ::mimic::orm::schema::types::Cardinality::One,
                item: ::mimic::orm::schema::node::Item {
                    indirect: false,
                    is: Some(<types::U64 as ::mimic::orm::traits::Path>::path().to_string()),
                    relation: None,
                },
                default: None,
            },
            primitive: Some(::mimic::orm::schema::types::PrimitiveType::U64),
            sanitizers: vec![],
            validators: vec![],
        }),
    );
}
#[derive(
    :: derive_more :: Add,
    :: derive_more :: AddAssign,
    :: derive_more ::
AsRef,
    Clone,
    Copy,
    Debug,
    Default,
    :: derive_more :: Deref,
    :: derive_more ::
DerefMut,
    :: serde :: Deserialize,
    Eq,
    :: derive_more :: FromStr,
    ::
derive_more :: Mul,
    :: derive_more :: MulAssign,
    Ord,
    PartialEq,
    PartialOrd,
    :: serde :: Serialize,
    :: derive_more :: Sub,
    :: derive_more :: SubAssign,
    ::
candid :: CandidType,
)]
pub struct Duration(types::U64);
impl ::mimic::orm::traits::Display for Duration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T> ::mimic::orm::traits::From<T> for Duration
where
    T: Into<types::U64>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}
impl ::mimic::orm::traits::NumCast for Duration {
    fn from<T: ::mimic::orm::traits::NumToPrimitive>(n: T) -> Option<Self> {
        let num = n.to_u64()?;
        <Self as ::mimic::orm::traits::NumFromPrimitive>::from_u64(num)
    }
}
impl ::mimic::orm::traits::Orderable for Duration {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        Ord::cmp(self, other)
    }
}
impl ::mimic::orm::traits::Filterable for Duration {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}
impl ::mimic::orm::traits::Inner<u64> for Duration {
    fn inner(&self) -> &u64 {
        self.0.inner()
    }
}
impl ::mimic::orm::traits::NodeDyn for Duration {
    fn path_dyn(&self) -> String {
        <Self as ::mimic::orm::traits::Path>::path()
    }
}
impl ::mimic::orm::traits::NumFromPrimitive for Duration {
    fn from_i64(n: i64) -> Option<Self> {
        type Ty = types::U64;
        Ty::from_i64(n).map(Self)
    }
    fn from_u64(n: u64) -> Option<Self> {
        type Ty = types::U64;
        Ty::from_u64(n).map(Self)
    }
}
impl ::mimic::orm::traits::NumToPrimitive for Duration {
    fn to_i64(&self) -> Option<i64> {
        ::mimic::export::num_traits::NumCast::from(self.0)
    }
    fn to_u64(&self) -> Option<u64> {
        ::mimic::export::num_traits::NumCast::from(self.0)
    }
}
impl ::mimic::orm::traits::Path for Duration {
    const IDENT: &'static str = "Duration";
    const PATH: &'static str = concat!(module_path!(), "::", "Duration");
}
impl ::mimic::orm::traits::SanitizeManual for Duration {}
impl ::mimic::orm::traits::SanitizeAuto for Duration {
    fn sanitize_auto(&mut self) {}
}
impl ::mimic::orm::traits::ValidateManual for Duration {}
impl ::mimic::orm::traits::ValidateAuto for Duration {
    fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::orm::types::ErrorVec> {
        Ok(())
    }
}
impl ::mimic::orm::traits::Visitable for Duration {
    fn drive(&self, visitor: &mut dyn ::mimic::orm::visit::Visitor) {
        ::mimic::orm::visit::perform_visit(visitor, &self.0, "");
    }
    fn drive_mut(&mut self, visitor: &mut dyn ::mimic::orm::visit::Visitor) {
        ::mimic::orm::visit::perform_visit_mut(visitor, &mut self.0, "");
    }
}

///
/// DurationMs
///
/// Duration in milliseconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct DurationMs {}

impl DurationMs {
    #[must_use]
    pub const fn hour(n: usize) -> Self {
        Self((n * 3_600_000) as u64)
    }
}

///
/// Timestamp
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Timestamp {}

impl Timestamp {
    #[must_use]
    pub fn now() -> Self {
        Self(crate::utils::time::now_secs())
    }
}

///
/// Created
///
/// A Timestamp that defaults to the current now() time when it is created
///

#[newtype(
    primitive = "U64",
    value(
        item(is = "types::time::Timestamp"),
        default = "types::time::Timestamp::now"
    )
)]
pub struct Created {}
