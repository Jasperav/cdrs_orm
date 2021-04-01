use crate::db_mirror::replace_qv_by_query;
use cdrs_con::test_query;
use cdrs_query_writer::Inf;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generates a insert method which inserts a single row by primary key
pub fn generate(inf: &Inf, fn_name: &Ident, using_ttl: bool) -> TokenStream {
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
        + " ("
        + &idents_names
        + ") values ("
        + &question_marks
        + ")";
    let simple_values = quote::format_ident!("simple_values");
    let ident_len = idents.len();

    let mut ts = if using_ttl {
        quote! {
            impl #name {
                fn #simple_values(self) -> Vec<cdrs_tokio::types::value::Value> {
                    let mut values: Vec<cdrs_tokio::types::value::Value> = Vec::with_capacity(#ident_len);

                    # (
                        values.push(cdrs_tokio::types::value::Value::new_normal(self.#idents));
                    ) *

                    values
                }
            }
        }
    } else {
        quote! {}
    };

    ts.extend(if using_ttl {
        let const_name = quote::format_ident!("INSERT_QUERY_USING_TTL");
        let query = query + " using ttl ?";

        test_query(&query);

        quote! {
            impl #name {
                pub const #const_name: &'static str = #query;

                pub fn #fn_name(self, using_ttl: i32) -> (&'static str, cdrs_tokio::query::QueryValues) {
                    let mut values = self.#simple_values();

                    values.push(cdrs_tokio::types::value::Value::new_normal(using_ttl));

                    let qv = cdrs_tokio::query::QueryValues::SimpleValues(values);

                    (#name::#const_name, qv)
                }
            }
        }
    } else {
        test_query(&query);

        quote! {
            impl #name {
                pub const #const_name: &'static str = #query;

                pub fn query_values(self) -> cdrs_tokio::query::QueryValues {
                    cdrs_tokio::query::QueryValues::SimpleValues(self.#simple_values())
                }

                pub fn #fn_name(self) -> (&'static str, cdrs_tokio::query::QueryValues) {
                    (#name::#const_name, self.query_values())
                }
            }
        }
    });

    ts
}
