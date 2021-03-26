use crate::method_writer::{Crud, Inf, Writer};
use crate::DELETE_UNIQUE;
use proc_macro2::TokenStream;
use quote::format_ident;

/// Writes the unique delete query
pub fn generate(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let db_mirror_fn_name = DELETE_UNIQUE;
    let custom_fn_name =
        db_mirror_fn_name.replacen(DELETE_UNIQUE, writer.fn_name_delete_unique(), 1);

    writer.write(
        inf,
        &format_ident!("{}", &db_mirror_fn_name),
        &format_ident!("{}", &custom_fn_name),
        Crud::DeleteUnique,
    )
}
