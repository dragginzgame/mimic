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

pub trait ViewType: View
where
    Self::Node: HasDef,
{
    /// Generate the view's token stream.
    fn view_part(&self) -> TokenStream;

    /// List of traits this node participates in
    /// (either derived or implemented).
    fn traits(&self) -> TraitList {
        TraitList(vec![
            Trait::CandidType,
            Trait::Clone,
            Trait::Debug,
            Trait::Serialize,
            Trait::Deserialize,
        ])
    }
}
