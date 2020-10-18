use crate::{Inf, Writer, CRUD, TRUNCATE};
use proc_macro2::TokenStream;
use quote::format_ident;

/// Writes the truncate query
pub fn generate(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let db_mirror_fn_name = TRUNCATE;
    let custom_fn_name = db_mirror_fn_name.replacen(TRUNCATE, writer.fn_name_truncate(), 1);

    writer.write(
        inf,
        &format_ident!("{}", &db_mirror_fn_name),
        &format_ident!("{}", &custom_fn_name),
        CRUD::Truncate,
    )
}
