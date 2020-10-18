use syn::Field;

mod crud;
mod method_writer;

pub use method_writer::*;
use proc_macro2::Ident;
use proc_macro2_helper::filter_attributes_from_fields;
use quote::format_ident;

pub const COLUMN_SEPARATOR: &str = "_";

pub const INSERT: &str = "insert_qv";
pub const DELETE_UNIQUE: &str = "delete_unique_qv";
pub const SELECT_UNIQUE: &str = "select_unique_qv";
pub const SELECT_ALL: &str = "select_all_q";
pub const SELECT_ALL_COUNT: &str = "select_all_count_q";
pub const TRUNCATE: &str = "truncate_q";

/// Note: updates are always executed for a single row
pub const UPDATE_OPTIONALS: &str = "update_qv";
pub const UPDATE_SINGLE_COLUMN: &str = "update_qv";
pub const UPDATE_SINGLE_COLUMN_DYNAMIC: &str = "update_dyn_qv";
pub const UPDATE_MULTIPLE_COLUMNS: &str = "update_multiple_qv";

pub fn update_single_column(column_name: &str) -> String {
    format!(
        "{}{}{}",
        UPDATE_SINGLE_COLUMN, COLUMN_SEPARATOR, column_name
    )
}

pub fn parameterized(v: &[Ident]) -> String {
    v.iter()
        .map(|f| f.clone().to_string() + " = ?")
        .collect::<Vec<_>>()
        .join(" and ")
}

pub fn add_query_and_uppercase<T: AsRef<str>>(q: &T) -> Ident {
    let q = q.as_ref().to_string() + "_query";

    format_ident!("{}", q.to_uppercase())
}

pub fn primary_key() -> Ident {
    format_ident!("WHERE_CLAUSE_PK")
}

pub fn read_attributes(fields: &[Field]) -> (Vec<&Field>, Vec<&Field>, Vec<&Field>) {
    let partition_key_fields = filter_attributes_from_fields(fields, "partition_key");
    let cluster_key_fields = filter_attributes_from_fields(fields, "clustering_key");
    let json_mapped_fields = filter_attributes_from_fields(fields, "json_mapped");

    if partition_key_fields.is_empty() {
        assert!(cluster_key_fields.is_empty());
    }

    (partition_key_fields, cluster_key_fields, json_mapped_fields)
}

pub fn pk_struct(name: &Ident) -> Ident {
    format_ident!("{}", pk_struct_str(&name.to_string()))
}

pub fn pk_struct_str(name: &str) -> String {
    name.to_string() + "PrimaryKey"
}

pub fn pk_parameter() -> Ident {
    format_ident!("primary_key")
}

pub fn where_pk_query_from_fields(fields: &[Field]) -> String {
    let (partition_fields, mut clustering_fields, _) = read_attributes(fields);

    let mut pk_fields = partition_fields.clone();

    pk_fields.append(&mut clustering_fields);

    let idents = pk_fields
        .into_iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<_>>();

    where_pk_query_from_idents(&idents)
}

pub fn where_pk_query_from_idents(idents: &[Ident]) -> String {
    let idents = idents
        .iter()
        .map(|i| i.to_string())
        .map(|i| i + " = ?")
        .collect::<Vec<_>>();

    " where ".to_string() + &idents.join(" and ")
}

pub fn updatable_columns_enum(struct_name: &str) -> Ident {
    format_ident!("{}UpdatableColumns", struct_name)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_compile() {}
}
