use crate::method_writer::{Inf, Writer, CRUD};
use crate::{SELECT_ALL, SELECT_UNIQUE};
use proc_macro2::TokenStream;
use quote::format_ident;

/// Writes the select queries
pub fn generate(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let mut unique = write_unique(inf, writer);
    let ts = write_all(inf, writer);

    unique.extend(ts);

    unique
}

fn write_unique(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let db_mirror_fn_name = SELECT_UNIQUE;
    let custom_fn_name =
        db_mirror_fn_name.replacen(SELECT_UNIQUE, writer.fn_name_select_unique(), 1);

    writer.write(
        inf,
        &format_ident!("{}", &db_mirror_fn_name),
        &format_ident!("{}", &custom_fn_name),
        CRUD::SelectUnique,
    )
}

fn write_all(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let db_mirror_fn_name = SELECT_ALL;
    let custom_fn_name = db_mirror_fn_name.replacen(SELECT_ALL, writer.fn_name_select_all(), 1);

    writer.write(
        inf,
        &format_ident!("{}", &db_mirror_fn_name),
        &format_ident!("{}", &custom_fn_name),
        CRUD::SelectAll,
    )
}
