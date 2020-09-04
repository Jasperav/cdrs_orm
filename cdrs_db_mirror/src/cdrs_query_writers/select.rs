use crate::cdrs_query_writers::shared_writer::write_select_or_delete;

use cdrs_query_writer::Inf;
use proc_macro2::{Ident, TokenStream};

/// Generates a select method which selects a single row by primary key
pub fn generate_unique(inf: &Inf, fn_name: &Ident) -> TokenStream {
    write_select_or_delete(&format!("select * from {}", inf.table_name), fn_name, inf)
}

/// Generates the query to query all rows from a given table
pub fn generate_all(inf: &Inf, fn_name: &Ident) -> TokenStream {
    let name = inf.name;
    let db_name = inf.table_name;
    let select_all = format!("select * from {}", db_name);

    quote::quote! {
        impl #name {
            pub const SELECT_ALL_QUERY: &'static str = #select_all;

            pub fn #fn_name() -> (&'static str, cdrs::query::QueryValues) {
                (#name::SELECT_ALL_QUERY, cdrs::query::QueryValues::SimpleValues(vec![]))
            }
        }
    }
}
