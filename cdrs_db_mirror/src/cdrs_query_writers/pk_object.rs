use crate::DERIVE_CDRS_PK;
use cdrs_query_writer::{primary_key, Inf};
use proc_macro2::TokenStream;
use quote::quote;

/// Generates the primary key struct
pub fn generate(inf: &Inf) -> TokenStream {
    if inf.partition_fields.is_empty() {
        return TokenStream::new();
    }

    let name = inf.name;
    let pk_struct = &inf.pk_struct;
    let pk_parameter = &inf.pk_parameter;
    let idents = inf
        .pk_fields
        .iter()
        .map(|p| p.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let mut properties = TokenStream::new();
    let mut mapping = TokenStream::new();

    for pk in inf.pk_fields.iter() {
        // TODO: Can this be done without clone?
        let ident = pk.ident.clone().unwrap();
        let ty = &pk.ty;

        if proc_macro2_helper::attributes_contains(&pk.attrs, "json_mapped") {
            properties.extend(quote! {
                #[json_mapped]
            });
        }

        properties.extend(quote! {
            pub #ident: #ty,
        });

        mapping.extend(quote! {
           #ident: self.#ident.clone(),
        });
    }

    let default_derives = vec![
        "PartialEq",
        "Clone",
        "Debug",
        "serde::Serialize",
        "serde::Deserialize",
        "cdrs_db_mirror::DBJson",
        "cdrs_helpers_derive::TryFromRow",
    ];
    let mut default_derives = default_derives
        .into_iter()
        .map(|d| d.parse().unwrap())
        .collect();
    let mut derives = extract_custom_derives();

    derives.append(&mut default_derives);

    let where_clause_query = cdrs_query_writer::where_pk_query_from_idents(&idents);
    let where_clause_pk = primary_key();

    quote! {
        #[derive(#(#derives),*)]
        pub struct #pk_struct {
            #properties
        }

        impl #name {
            pub fn #pk_parameter(&self) -> #pk_struct {
                #pk_struct {
                    #mapping
                }
            }
        }

        impl #pk_struct {
            pub const #where_clause_pk: &'static str = #where_clause_query;

            pub fn where_clause(&self) -> cdrs::query::QueryValues {
                use std::iter::FromIterator;

                let mut query_values: Vec<cdrs::types::value::Value> = Vec::new();

                #(
                    query_values.push(cdrs::types::value::Value::new_normal(self.#idents.clone()));
                )*

                cdrs::query::QueryValues::SimpleValues(query_values)
            }
        }
    }
}

fn extract_custom_derives() -> Vec<TokenStream> {
    match std::env::var(DERIVE_CDRS_PK) {
        Ok(value_string) => {
            if value_string.is_empty() {
                vec![]
            } else {
                let values = value_string.split(',');
                let mut values_vec = Vec::new();

                for value in values {
                    values_vec.push(value.parse().unwrap());
                }

                values_vec
            }
        }
        _ => vec![],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_custom_derives() {
        let current_val = std::env::var(DERIVE_CDRS_PK);

        assert!(extract_custom_derives().is_empty());

        std::env::set_var(DERIVE_CDRS_PK, "Copy");
        let result = extract_custom_derives();

        assert_eq!(1, result.len());
        assert_eq!("Copy", result[0].to_string().as_str());

        std::env::set_var(DERIVE_CDRS_PK, "Copy,Clone,Something");
        let result = extract_custom_derives();

        assert_eq!(3, result.len());
        assert_eq!("Copy", result[0].to_string().as_str());
        assert_eq!("Clone", result[1].to_string().as_str());
        assert_eq!("Something", result[2].to_string().as_str());

        if let Ok(e) = current_val {
            std::env::set_var(DERIVE_CDRS_PK, e);
        }
    }
}
