use crate::method_writer::{Inf, Writer, CRUD};
use crate::INSERT;
use proc_macro2::TokenStream;
use quote::format_ident;

/// Writes the unique insert query
pub fn generate(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let db_mirror_fn_name = INSERT;
    let custom_fn_name = db_mirror_fn_name.replacen(INSERT, writer.fn_name_insert(), 1);

    writer.write(
        inf,
        &format_ident!("{}", &db_mirror_fn_name),
        &format_ident!("{}", &custom_fn_name),
        CRUD::InsertUnique,
    )
}
