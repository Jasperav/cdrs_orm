use proc_macro2::TokenStream;
use quote::format_ident;

use crate::method_writer::{Crud, Inf, Writer};
use crate::{INSERT, INSERT_USING_TTL};

/// Writes the unique insert query
pub fn generate(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let mut ts = TokenStream::new();

    ts.extend(generate_query(
        inf,
        writer,
        INSERT,
        writer.fn_name_insert(),
        false,
    ));
    ts.extend(generate_query(
        inf,
        writer,
        INSERT_USING_TTL,
        writer.fn_name_insert_using_ttl(),
        true,
    ));

    ts
}

fn generate_query(
    inf: &Inf,
    writer: &impl Writer,
    fn_name: &str,
    custom_fn_name: &str,
    using_ttl: bool,
) -> TokenStream {
    writer.write(
        inf,
        &format_ident!("{}", &fn_name),
        &format_ident!("{}", &custom_fn_name),
        Crud::InsertUnique(using_ttl),
    )
}
