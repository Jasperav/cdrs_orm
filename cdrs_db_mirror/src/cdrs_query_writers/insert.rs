use crate::db_mirror::replace_qv_by_query;
use cdrs_query_writer::Inf;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generates a insert method which inserts a single row by primary key
pub fn generate(inf: &Inf, fn_name: &Ident) -> TokenStream {
    let name = inf.name;
    let idents = inf
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<_>>();
    let idents_names: String = idents
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let question_marks: String = idents
        .iter()
        .map(|_| "?".to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let table_name = inf.table_name;
    let const_name = replace_qv_by_query(fn_name);
    let query = "insert into ".to_string()
        + table_name
        + "("
        + &idents_names
        + ") values ("
        + &question_marks
        + ")";

    quote! {
        impl #name {
            pub const #const_name: &'static str = #query;

            pub fn query_values(&self) -> cdrs::query::QueryValues {
                use std::collections::HashMap;
                let mut values: HashMap<String, cdrs::types::value::Value> = HashMap::new();

                # (
                    values.insert(stringify!(#idents).to_string(), cdrs::types::value::Value::new_normal(self.#idents.clone()));
                ) *

                cdrs::query::QueryValues::NamedValues(values)
            }

            pub fn #fn_name(&self) -> (&'static str, cdrs::query::QueryValues) {
                (#name::#const_name, self.query_values())
            }
        }
    }
}
