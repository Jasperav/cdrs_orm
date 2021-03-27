use cdrs_query_writer::{Crud, Inf, Writer};
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use syn::DeriveInput;

pub(crate) fn db_mirror(derive_input: DeriveInput) -> TokenStream {
    cdrs_query_writer::write(derive_input, ImplWriter)
}

pub(crate) fn replace_qv_by_query(ident: &Ident) -> Ident {
    replace_by_query(ident, 2)
}

pub(crate) fn replace_by_query(ident: &Ident, prefix_len: usize) -> Ident {
    let to_string = ident.to_string();
    let without_qv = to_string[0..to_string.len() - prefix_len].to_string() + "query";

    format_ident!("{}", without_qv.to_uppercase())
}

struct ImplWriter;

impl Writer for ImplWriter {
    fn write_pk(&self, inf: &Inf) -> proc_macro2::TokenStream {
        crate::cdrs_query_writers::pk_object::generate(&inf)
    }

    fn write(
        &self,
        inf: &Inf,
        db_mirror_fn_name: &Ident,
        _custom_fn_name: &Ident,
        crud: Crud,
    ) -> proc_macro2::TokenStream {
        match crud {
            Crud::InsertUnique(using_ttl) => {
                crate::cdrs_query_writers::insert::generate(inf, db_mirror_fn_name, using_ttl)
            }
            Crud::UpdateUnique(update) => {
                crate::cdrs_query_writers::update::generate(inf, db_mirror_fn_name, update)
            }
            Crud::DeleteUnique => {
                crate::cdrs_query_writers::delete::generate(inf, db_mirror_fn_name)
            }
            Crud::SelectUnique => {
                crate::cdrs_query_writers::select::generate_unique(inf, db_mirror_fn_name)
            }
            Crud::SelectAll => crate::cdrs_query_writers::select::generate_all(
                inf,
                db_mirror_fn_name,
                "*",
                "SELECT_ALL_QUERY",
            ),
            Crud::SelectAllCount => crate::cdrs_query_writers::select::generate_all(
                inf,
                db_mirror_fn_name,
                "count(*)",
                "SELECT_ALL_COUNT_QUERY",
            ),
            Crud::Truncate => crate::cdrs_query_writers::truncate::generate(inf, db_mirror_fn_name),
        }
    }
}
