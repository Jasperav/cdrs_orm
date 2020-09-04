use cdrs_query::Query;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Transforms a query to (&'static str, cdrs::query::QueryValues)
/// let (query, values) = cdrs_query_simple::str_qv!("select * from my_table where some_property = ?", my_property);
/// The query variable equals "select * from my_table where some_property = ?"
/// The values variable equals query_values!(my_property)
/// And the query is checked at compile time and type checked
#[proc_macro]
pub fn str_qv(input: TokenStream) -> TokenStream {
    let query = parse_macro_input!(input as Query);

    let query_raw = query.query_raw;
    let query_values = query.qv;
    let ts = quote::quote! {
        (#query_raw, #query_values)
    };

    ts.into()
}
