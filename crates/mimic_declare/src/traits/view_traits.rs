use crate::prelude::*;

///
/// View
///
/// A node that emits additional derived view representations
/// (e.g., main View, Edit, Filter).
///

pub trait View {
    type Node;

    fn node(&self) -> &Self::Node;
}

///
/// ViewType
/// a macro type that can emit the code for a view type
///

pub trait ViewType: View {
    /// Generate the view's token stream.
    fn view_part(&self) -> TokenStream;

    // Naming shortcuts
    fn view_ident(&self) -> Ident {
        format_ident!("{}View", self.node().def().ident())
    }

    fn edit_ident(&self) -> Ident {
        format_ident!("{}Edit", self.node().def().ident())
    }

    fn filter_ident(&self) -> Ident {
        format_ident!("{}Filter", self.node().def().ident())
    }

    /// List of traits this node participates in
    /// (either derived or implemented).
    fn traits(&self) -> TraitList {
        TraitList(vec![
            Trait::CandidType,
            Trait::Clone,
            Trait::Debug,
            Trait::Default,
            Trait::Serialize,
            Trait::Deserialize,
        ])
    }
}
