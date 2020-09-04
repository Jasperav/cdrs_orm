use crate::cdrs_query_writers::shared_writer::write_select_or_delete;
use cdrs_query_writer::Inf;
use proc_macro2::{Ident, TokenStream};

/// Generates a delete method which deletes a single row by primary key
pub fn generate(inf: &Inf, fn_name: &Ident) -> TokenStream {
    write_select_or_delete(&format!("delete from {}", inf.table_name), fn_name, inf)
}
