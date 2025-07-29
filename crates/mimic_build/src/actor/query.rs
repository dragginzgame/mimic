use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    let mut tokens = quote!();

    tokens.extend(generate_query("mimic_query_load", builder, QueryKind::Load));
    tokens.extend(generate_query("mimic_query_save", builder, QueryKind::Save));
    tokens.extend(generate_query(
        "mimic_query_delete",
        builder,
        QueryKind::Delete,
    ));

    tokens
}

enum QueryKind {
    Load,
    Save,
    Delete,
}

// generate_query
fn generate_query(name: &str, builder: &ActorBuilder, kind: QueryKind) -> TokenStream {
    let entities = builder.get_entities();

    let match_arms = if entities.is_empty() {
        quote! {
            Err(::mimic::interface::query::QueryError::EntityNotFound(path))
                .map_err(::mimic::interface::InterfaceError::from)?
        }
    } else {
        let arms = entities.iter().map(|(entity_path, _)| {
            let ty: Path =
                parse_str(entity_path).unwrap_or_else(|_| panic!("Invalid path: {entity_path}"));

            match kind {
                QueryKind::Load => quote! {
                    #entity_path => db!().load().response::<#ty>(query)
                },
                QueryKind::Delete => quote! {
                    #entity_path => db!().delete().response::<#ty>(query)
                },
                QueryKind::Save => quote! {
                    #entity_path =>  db!().save().execute::<#ty>(query)
                },
            }
        });

        quote! {
            let res = match path.as_str() {
                #(#arms,)*
                _ => Err(::mimic::interface::query::QueryError::EntityNotFound(path))
                    .map_err(::mimic::interface::InterfaceError::from)?,
            }?;

            Ok(res)
        }
    };

    // generate the fn
    let fn_name = quote::format_ident!("{name}");
    let fn_sig = match kind {
        QueryKind::Load => quote! {
            #[::mimic::ic::query]
            pub fn #fn_name(
                path: String,
                query: ::mimic::db::query::LoadQuery,
            ) -> Result<::mimic::db::response::LoadResponse, ::mimic::Error>
        },

        QueryKind::Save => quote! {
            #[::mimic::ic::update]
            pub fn #fn_name(
                path: String,
                query: ::mimic::db::query::SaveQuery,
            ) -> Result<::mimic::core::Key, ::mimic::Error>
        },

        QueryKind::Delete => quote! {
            #[::mimic::ic::update]
            pub fn #fn_name(
                path: String,
                query: ::mimic::db::query::DeleteQuery,
            ) -> Result<::mimic::db::response::DeleteResponse, ::mimic::Error>
        },
    };

    quote! {
        #[allow(unused_variables)]
        #fn_sig {
            #match_arms
        }
    }
}
