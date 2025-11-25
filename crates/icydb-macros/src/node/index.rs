use crate::prelude::*;

///
/// Index
///

#[derive(Debug, FromMeta)]
pub struct Index {
    pub store: Path,

    #[darling(default, map = "split_idents")]
    pub fields: Vec<Ident>,

    #[darling(default)]
    pub unique: bool,
}

impl HasSchemaPart for Index {
    fn schema_part(&self) -> TokenStream {
        let store = quote_one(&self.store, to_path);
        let fields = quote_slice(&self.fields, to_str_lit);
        let unique = &self.unique;

        quote! {
            ::icydb::schema::node::Index {
                store: #store,
                fields: #fields,
                unique: #unique,
            }
        }
    }
}
