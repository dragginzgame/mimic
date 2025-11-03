use crate::prelude::*;

///
/// View
/// a helper wrapper around a Node
///

pub trait View {
    type Node;

    fn node(&self) -> &Self::Node;
}

///
/// ViewType
///
/// A node that emits additional derived view representations
/// (e.g., main View, Edit, Filter).
///

pub trait ViewType {
    /// Generate the view's token stream.
    fn generate(&self) -> TokenStream;

    /// List of traits this node participates in
    /// (either derived or implemented).
    fn traits(&self) -> TraitSet {
        TraitSet::from(vec![
            TraitKind::CandidType,
            TraitKind::Clone,
            TraitKind::Debug,
            TraitKind::Serialize,
            TraitKind::Deserialize,
        ])
    }
}
