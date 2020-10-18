use cdrs_query_writer::Inf;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generates a truncate query
pub fn generate(inf: &Inf, fn_name: &Ident) -> TokenStream {
    let name = inf.name;

    quote! {
        impl #name {
            pub const TRUNCATE_QUERY: &'static str = concat!("truncate ", stringify!(#name));

            pub fn #fn_name(&self) -> (&'static str, cdrs::query::QueryValues) {
                (#name::TRUNCATE_QUERY, cdrs::query::QueryValues::SimpleValues(vec![]))
            }
        }
    }
}
