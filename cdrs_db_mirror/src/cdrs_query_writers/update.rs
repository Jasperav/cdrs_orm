use cdrs_query_writer::{primary_key, Inf, Update};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

/// Generates several update statements for the Rust struct
pub(crate) fn generate(inf: &Inf, fn_name: &Ident, update: Update) -> TokenStream {
    let pk_struct = &inf.pk_struct;
    let pk_parameter = &inf.pk_parameter;
    let db_name = inf.table_name;
    let name = inf.name;

    match update {
        Update::SingleColumn((ident, ty)) => {
            let const_name = format_ident!(
                "{}",
                (fn_name.to_string().replacen("_qv_", "_", 1) + "_query").to_uppercase()
            );

            let query = format!("update {} set {} = ?", db_name, ident.to_string());
            let idents = inf
                .pk_fields
                .iter()
                .map(|p| p.ident.clone().unwrap())
                .collect::<Vec<_>>();
            let where_clause_query = cdrs_query_writer::where_pk_query_from_idents(&idents);

            quote! {
                impl #name {
                    pub const #const_name: &'static str = concat!(#query, #where_clause_query);

                    pub fn #fn_name(#pk_parameter: &#pk_struct, #ident: #ty) -> (&'static str, cdrs::query::QueryValues) {
                        let mut values = primary_key.where_clause();

                        let value = cdrs::types::value::Value::new_normal(#ident);

                        let mut values = match values {
                            cdrs::query::QueryValues::SimpleValues(mut v) => {
                                v.insert(0, value);

                                v
                            },
                            _ => panic!()
                        };

                        (#name::#const_name, cdrs::query::QueryValues::SimpleValues(values))
                    }
                }
            }
        }
        Update::AllOptional((idents, types)) => {
            let idents_name = idents.iter().map(|i| i.to_string()).collect::<Vec<_>>();
            let pk = primary_key();

            quote! {
                impl #name {
                    pub fn #fn_name(#pk_parameter: &#pk_struct, #(#idents: #types),*) -> std::option::Option<(String, cdrs::query::QueryValues)> {
                        let mut to_update: Vec<String> = std::vec::Vec::new();
                        let mut qv: Vec<cdrs::types::value::Value> = std::vec::Vec::new();

                        #(
                            if let Some(s) = #idents {
                                to_update.push(format!("{} = ?", #idents_name));
                                qv.push(cdrs::types::value::Value::new_normal(s));
                            }
                        )*

                        if to_update.is_empty() {
                            return None
                        }

                        let to_update: String = to_update.join(", ");
                        let to_update = format!("set {}", to_update);
                        let values = primary_key.where_clause();

                        match values {
                            cdrs::query::QueryValues::SimpleValues(mut v) => {
                                qv.append(&mut v);
                            },
                            _ => panic!()
                        };

                        let string = format!("update {} {}{}", #db_name, to_update, #pk_struct::#pk);

                        Some((string, cdrs::query::QueryValues::SimpleValues(qv)))
                    }
                }
            }
        }
        Update::Dynamic(dynamic_update) => {
            let enum_ident = &inf.updateable_columns_enum;
            let enum_pk_param = &inf.updateable_columns_enum_parameter;
            let enum_method_names = dynamic_update.enum_method_names;
            let enum_cases = dynamic_update.enum_cases;
            let tys = dynamic_update.updateable_columns_types;

            // Still generate the enum cases even if there are no cases or it materialized view
            // because code generation will be easier
            let mut e = if inf.materialized_view.is_none() {
                quote! {
                    pub enum #enum_ident {
                        #(#enum_cases(#tys)),*
                    }
                }
            } else {
                quote! {
                    pub enum #enum_ident {}
                }
            };

            if !enum_cases.is_empty() && inf.materialized_view.is_none() {
                e.extend(quote! {
                    impl #name {
                        pub fn #fn_name(#pk_parameter: &#pk_struct, #enum_pk_param: #enum_ident) -> (&'static str, cdrs::query::QueryValues) {
                            match #enum_pk_param {
                                #(#enum_ident::#enum_cases(val) => #name::#enum_method_names(#pk_parameter, val)),*
                            }
                        }
                    }
                })
            }

            e
        }
        Update::DynamicVec(dynamic_multiple_updates) => {
            let enum_column_names = dynamic_multiple_updates.enum_column_names;
            let enum_cases = dynamic_multiple_updates.enum_cases;
            let enum_ident = &inf.updateable_columns_enum;
            let pk = primary_key();

            if enum_cases.is_empty() || inf.materialized_view.is_some() {
                return quote! {};
            }

            quote! {
                impl #name {
                    pub fn #fn_name(#pk_parameter: &#pk_struct, vec: std::vec::Vec<#enum_ident>) -> (String, cdrs::query::QueryValues) {
                        assert!(!vec.is_empty());
                        let mut query = vec![];
                        let mut values: std::vec::Vec<cdrs::types::value::Value> = vec![];

                        for ident in vec {
                            match ident {
                                #(#enum_ident::#enum_cases(val) => {
                                    query.push(concat!(stringify!(#enum_column_names), " = ?"));
                                    values.push(val.into());
                                },)*
                            }
                        }

                        let columns_to_update: String = query.join(", ");
                        let update_statement = format!("update {} set {}{}", stringify!(#name), columns_to_update, #pk_struct::#pk);
                        match primary_key.where_clause() {
                            cdrs::query::QueryValues::SimpleValues(v) => values.extend(v),
                            _ => panic!()
                        };



                        let query_values = cdrs::query::QueryValues::SimpleValues(values);

                        (update_statement, query_values)
                    }
                }
            }
        }
    }
}
