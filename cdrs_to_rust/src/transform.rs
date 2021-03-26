use crate::JSONMapping;
use proc_macro2::TokenStream;

/// Custom transformations can be done by implementing this trait
pub trait Transformer {
    /// The default implementation is that column names are snake cased named
    fn column_name_to_struct_property_name(&self, column_name: &str) -> String {
        column_name.to_string()
    }

    fn table_name_to_struct_name(&self, table_name: &str) -> String {
        cdrs_con::capitalizing::table_name_to_struct_name(table_name)
    }

    fn filename(&self, table_name: &str, _struct_name: &syn::Ident) -> String {
        table_name.to_string()
    }

    /// This crate will create a mod file in the generated folder
    /// The default implementation is that it will expose the struct
    fn mod_file(&self, _table_name: &str, _struct_name: &syn::Ident, filename: &str) -> String {
        format!("pub mod {};\n", &filename)
    }

    /// Adds a way to add the JSON mapping to a property
    fn json_mapping(&self, _table_name: &str, _column_name: &str) -> Option<JSONMapping> {
        None
    }

    /// Possibility to add attributes to a field
    fn attributes(&self, _table_name: &str, _column_name: &str) -> TokenStream {
        Default::default()
    }

    /// Add custom derives
    fn derive(&self, _struct_name: &str) -> Vec<&'static str> {
        vec![
            "cdrs_db_mirror::DBMirror",
            "cdrs_tokio_helpers_derive::TryFromRow",
        ]
    }

    /// Add annotations on top of the struct, like #[allow(dead_code)]
    fn metadata(&self, _struct_name: &str) -> Vec<String> {
        vec![]
    }
}

pub struct DefaultTransformer;

impl Transformer for DefaultTransformer {}
