use syn::parse_macro_input;

use cdrs_con::QueryType;
use cdrs_query::Query;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn control(input: TokenStream) -> TokenStream {
    let query = parse_macro_input!(input as Query);
    let qmd = query.qmd;

    let val = match qmd.query_type {
        QueryType::SelectMultiple => 0,
        QueryType::SelectUnique => 1,
        QueryType::SelectUniqueByLimit => 2,
        QueryType::SelectCount => 3,
        QueryType::UpdateUnique => 4,
        QueryType::DeleteMultiple => 5,
        QueryType::DeleteUnique => 6,
        QueryType::InsertUnique => 7,
    };
    let qv = query.qv;

    let q = quote! {
        (#val, #qv)
    };

    q.into()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_compile() {}
}
