use cdrs_query_writer::Inf;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generates a truncate query
pub fn generate(inf: &Inf, fn_name: &Ident) -> TokenStream {
    let name = inf.name;
    let table_name = inf.table_name;
    let query = format!("truncate {}", table_name);

    quote! {
        impl #name {
            pub const TRUNCATE_QUERY: &'static str = #query;

            pub fn #fn_name() -> (&'static str, cdrs_tokio::query::QueryValues) {
                (#name::TRUNCATE_QUERY, cdrs_tokio::query::QueryValues::SimpleValues(vec![]))
            }
        }
    }
}
