use crate::node::Trait;
use proc_macro2::TokenStream;

///
/// TraitStrategy
///

#[derive(Debug, Default)]
pub struct TraitStrategy {
    pub derive: Option<Trait>,
    pub imp: Option<TokenStream>,
}

impl TraitStrategy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_derive(tr: Trait) -> Self {
        Self::new().with_derive(tr)
    }

    pub fn from_impl(tokens: TokenStream) -> Self {
        Self::new().with_impl(tokens)
    }

    pub const fn with_derive(mut self, tr: Trait) -> Self {
        self.derive = Some(tr);
        self
    }

    pub fn with_impl(mut self, tokens: TokenStream) -> Self {
        self.imp = Some(tokens);
        self
    }
}
