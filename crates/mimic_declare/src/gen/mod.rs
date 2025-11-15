pub mod implementor;

pub use implementor::*;

use crate::{prelude::*, view::*};

///
/// Common interface for all node generators.
///
/// Each generator emits both the base node and all its derived representations
/// (views, filters, create/update types, etc.).
///

pub trait NodeGen {
    /// Emit the code for this node and all derived forms.
    fn generate(&self) -> TokenStream;
}

///
/// EntityGen
/// Generates all derived code for an `Entity` node.
///

pub struct EntityGen<'a>(pub &'a Entity);

impl EntityGen<'_> {
    pub fn generate(&self) -> TokenStream {
        let node = self.0;

        let view = EntityView(node);
        let create = EntityCreate(node);
        let update = EntityUpdate(node);
        let filter = EntityFilter(node);

        quote! {
            #node

            #view
            #create
            #update
            #filter
        }
    }
}

impl ToTokens for EntityGen<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}

///
/// RecordGen
///

pub struct RecordGen<'a>(pub &'a Record);

impl RecordGen<'_> {
    pub fn generate(&self) -> TokenStream {
        let node = self.0;

        let view = RecordView(node);
        let update = RecordUpdate(node);
        let filter = RecordFilter(node);

        quote! {
            #node

            #view
            #update
            #filter
        }
    }
}

impl ToTokens for RecordGen<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}

// simple_gen
// for the simpler types that only have a view
macro_rules! simple_gen {
    ($name:ident, $node:ty, $view:path) => {
        pub struct $name<'a>(pub &'a $node);

        impl NodeGen for $name<'_> {
            fn generate(&self) -> TokenStream {
                let node = self.0;
                let view = $view(node);

                quote! {
                    #node
                    #view
                }
            }
        }

        impl ToTokens for $name<'_> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                tokens.extend(self.generate());
            }
        }
    };
}

simple_gen!(EnumGen, Enum, EnumView);
simple_gen!(ListGen, List, ListView);
simple_gen!(MapGen, Map, MapView);
simple_gen!(NewtypeGen, Newtype, NewtypeView);
simple_gen!(SetGen, Set, SetView);
simple_gen!(TupleGen, Tuple, TupleView);

// passthrough_gen
// for the simpler types that only have a view
macro_rules! passthrough_gen {
    ($name:ident, $node:ty) => {
        pub struct $name<'a>(pub &'a $node);

        impl NodeGen for $name<'_> {
            fn generate(&self) -> TokenStream {
                let node = self.0;

                quote! {
                    #node
                }
            }
        }

        impl ToTokens for $name<'_> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                tokens.extend(self.generate());
            }
        }
    };
}

passthrough_gen!(CanisterGen, Canister);
passthrough_gen!(SanitizerGen, Sanitizer);
passthrough_gen!(StoreGen, Store);
passthrough_gen!(ValidatorGen, Validator);
