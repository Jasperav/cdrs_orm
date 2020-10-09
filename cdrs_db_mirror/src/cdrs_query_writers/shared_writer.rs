use cdrs_con::test_query;
use cdrs_query_writer::{parameterized, Inf};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;

/// Writes the actual tokenstream for the current type for a unique delete or unique select
pub fn write_select_or_delete(
    // The query to write without the where clause
    query_without_where: &str,
    // The function name to use for acquiring the query
    fn_name: &Ident,
    // Information about the table
    inf: &Inf,
) -> TokenStream {
    let name = inf.name;
    let query_where_clause_only = parameterized(
        &inf.primary_fields
            .iter()
            .map(|p| p.ident.clone().unwrap())
            .collect::<Vec<_>>(),
    );
    let query = format!("{} where {}", query_without_where, query_where_clause_only);
    let const_name =
        cdrs_query_writer::add_query_and_uppercase(&fn_name.to_string().replacen("_qv", "", 1));

    test_query(&query);

    let pk_struct = &inf.pk_struct;
    let _ty: Type = syn::parse2(quote! {
        #pk_struct
    })
    .unwrap();
    let _p = &inf.pk_parameter;

    quote! {
        impl #name {
            pub const #const_name: &'static str = #query;
        }
        impl #pk_struct {
            pub fn #fn_name(&self) -> (&'static str, cdrs::query::QueryValues) {
                (#name::#const_name, self.where_clause())
            }
        }
    }
}
