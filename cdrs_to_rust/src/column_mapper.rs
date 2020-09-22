use crate::{Columns, Transformer};
use cdrs_con::supported_data_types::CassandraDataType;
use cdrs_con::ColumnKind;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

/// Transforms a db column to a Rust property
pub(crate) fn column_to_property(
    table_name: &str,
    columns: Vec<Columns>,
    transformer: &impl Transformer,
    file: &mut File,
) -> Vec<TokenStream> {
    let mut added_imports = HashSet::new();

    columns
        .into_iter()
        .map(|c| {
            let mut att = match c.kind() {
                ColumnKind::PartitionKey => quote! {
                    #[partition_key]
                },
                ColumnKind::Regular => quote! {},
                ColumnKind::Clustering => quote! {
                    #[clustering_key]
                },
            };

            let ty = if let Some(mapping) = transformer.json_mapping(table_name, &c.column_name) {
                assert_eq!("text", &c.data_type);

                if !mapping.import.is_empty() {
                    let import = format!("use {};", mapping.import);

                    // Don't add imports twice
                    if added_imports.insert(import.clone()) {
                        write!(file, "{}", &import).unwrap();
                    }
                }

                let attrs = mapping.attributes;

                att.extend(quote! {
                    #attrs
                    #[json_mapped]
                });

                let ts = mapping.raw_type;

                if mapping.nullable {
                    quote! {
                        std::option::Option<#ts>
                    }
                } else {
                    ts
                }
            } else {
                let ty = CassandraDataType::new(c.data_type.as_str()).to_ty();
                // This can be turned into a ident, but the json_mapping can't
                // and if this is not turned into an ident, ty is just a string

                ty.parse().unwrap()
            };

            let ident = format_ident!(
                "{}",
                &transformer.column_name_to_struct_property_name(&c.column_name)
            );

            att.extend(quote! {
                pub #ident: #ty,
            });

            att
        })
        .collect()
}

pub(crate) fn properties_to_struct(
    struct_name: &syn::Ident,
    properties: Vec<TokenStream>,
    transformer: &impl Transformer,
    imports: &TokenStream,
) -> TokenStream {
    let custom_derives = transformer
        .derive(struct_name.to_string().as_ref())
        .into_iter()
        .map(|d| d.parse().unwrap())
        .collect::<Vec<TokenStream>>();
    let metadata = transformer
        .metadata(struct_name.to_string().as_ref())
        .into_iter()
        .map(|d| d.parse().unwrap())
        .collect::<Vec<TokenStream>>();

    quote! {
        #imports

        #(#metadata)*
        #[derive(#(#custom_derives),*)]
        pub struct #struct_name {
            #(#properties)*
        }
    }
}
