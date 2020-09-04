use proc_macro::TokenStream;

use syn::DeriveInput;

/// Add a comma separates list to this environment key to add derives for primary key structs
const DERIVE_CDRS_PK: &str = "DERIVE_CDRS_PK";

mod cdrs_query_writers;
mod db_json;
mod db_mirror;

#[proc_macro_derive(DBJson, attributes(json_mapped))]
pub fn db_json(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();

    db_json::db_json(derive_input).into()
}

#[proc_macro_derive(DBMirror, attributes(partition_key, clustering_key, json_mapped))]
pub fn db_mirror(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();

    db_mirror::db_mirror(derive_input).into()
}
