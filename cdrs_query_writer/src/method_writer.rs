use crate::{
    pk_parameter, pk_struct, read_attributes, updateable_columns_enum, COLUMN_SEPARATOR,
    DELETE_UNIQUE, INSERT, SELECT_ALL, SELECT_ALL_COUNT, SELECT_UNIQUE, UPDATE_MULTIPLE_COLUMNS,
    UPDATE_OPTIONALS, UPDATE_SINGLE_COLUMN, UPDATE_SINGLE_COLUMN_DYNAMIC,
};
use cdrs_con::capitalizing::struct_name_to_db_table_name;
use cdrs_con::create_test_db_session;
use cdrs_con::materialized_view::{query_materialized_view, MaterializedViewInf};
use proc_macro2::TokenStream;
use proc_macro2_helper::named_struct_fields_from_data;
use quote::format_ident;
use syn::{DeriveInput, Field, Ident, Type};

pub enum CRUD<'a> {
    InsertUnique,
    UpdateUnique(Update<'a>),
    SelectUnique,
    SelectAll,
    SelectAllCount,
    DeleteUnique,
}

pub struct DynamicUpdate<'a> {
    pub updateable_columns: &'a Vec<Ident>,
    pub updateable_columns_types: &'a Vec<Type>,
    pub enum_cases: &'a Vec<Ident>,
    pub enum_method_names: &'a Vec<Ident>,
}

pub struct DynamicMultipleUpdates<'a> {
    pub enum_cases: &'a Vec<Ident>,
    pub enum_column_names: &'a Vec<Ident>,
}

// No 'ReplaceRow' because that's the same as an insert
pub enum Update<'a> {
    Dynamic(DynamicUpdate<'a>),
    DynamicVec(DynamicMultipleUpdates<'a>),
    SingleColumn((&'a Ident, &'a Type)),
    AllOptional((&'a Vec<Ident>, &'a Vec<Type>)),
}

#[derive(Clone)]
pub struct Inf<'a> {
    pub name: &'a Ident,
    pub partition_fields: Vec<&'a Field>,
    pub clustering_fields: Vec<&'a Field>,
    /// partition_fields and clustering_fields combined
    pub primary_fields: Vec<&'a Field>,
    pub json_mapped_fields: Vec<&'a Field>,
    pub pk_fields: Vec<&'a Field>,
    pub non_pk_fields: Vec<&'a Field>,
    pub table_name: &'a str,
    pub pk_struct: Ident,
    pub pk_parameter: Ident,
    pub fields: Vec<Field>,
    pub updateable_columns_enum: Ident,
    pub updateable_columns_enum_parameter: Ident,
    // If filled this current table is a materialized view and it's filled with the base table name and struct name
    pub materialized_view: Option<MaterializedViewInf>,
}

pub struct BaseTableTokenstream {
    pub update: TokenStream,
    pub delete: TokenStream,
    pub insert: TokenStream,
}

pub trait Writer {
    fn write_pk(&self, inf: &Inf) -> TokenStream;
    fn write(
        &self,
        inf: &Inf,
        db_mirror_fn_name: &Ident,
        custom_fn_name: &Ident,
        crud: CRUD,
    ) -> TokenStream;

    fn post_process_ts(
        &self,
        mut select_queries: TokenStream,
        pk: TokenStream,
        base_table_tokenstream: Option<BaseTableTokenstream>,
    ) -> TokenStream {
        select_queries.extend(pk);

        if let Some(btt) = base_table_tokenstream {
            select_queries.extend(btt.insert);
            select_queries.extend(btt.update);
            select_queries.extend(btt.delete);
        }

        select_queries
    }

    // Override the 'fn_name...' methods to a custom fn name
    fn fn_name_insert(&self) -> &'static str {
        INSERT
    }
    fn fn_name_select_unique(&self) -> &'static str {
        SELECT_UNIQUE
    }
    fn fn_name_select_all(&self) -> &'static str {
        SELECT_ALL
    }
    fn fn_name_select_all_count(&self) -> &'static str {
        SELECT_ALL_COUNT
    }
    fn fn_name_delete_unique(&self) -> &'static str {
        DELETE_UNIQUE
    }
    fn fn_name_update_optionals(&self) -> &'static str {
        UPDATE_OPTIONALS
    }
    fn fn_name_update_column(&self, column: &str) -> String {
        format!("{}{}{}", UPDATE_SINGLE_COLUMN, COLUMN_SEPARATOR, column)
    }
    fn fn_name_update_column_dynamic(&self) -> &'static str {
        UPDATE_SINGLE_COLUMN_DYNAMIC
    }

    fn fn_name_update_multiple_columns(&self) -> &'static str {
        UPDATE_MULTIPLE_COLUMNS
    }
}

pub fn write(derive: DeriveInput, writer: impl Writer) -> TokenStream {
    let name = &derive.ident;
    let fields = named_struct_fields_from_data(derive.data);
    let (partition_fields, clustering_fields, json_mapped_fields) = read_attributes(&fields);

    let mut pk_fields = partition_fields.clone();

    pk_fields.append(&mut clustering_fields.clone());

    let table_name = struct_name_to_db_table_name(name.to_string().as_str());
    let non_pk_fields = fields.iter().filter(|f| !pk_fields.contains(f)).collect();
    let pk_struct = pk_struct(&name);
    let session = create_test_db_session();
    let updateable_columns_enum = updateable_columns_enum(&name.to_string());
    let mv = query_materialized_view(&session, &table_name);
    let inf = Inf {
        name,
        primary_fields: partition_fields
            .iter()
            .cloned()
            .chain(clustering_fields.iter().cloned())
            .collect(),
        partition_fields,
        clustering_fields,
        json_mapped_fields,
        pk_fields,
        non_pk_fields,
        table_name: &table_name,
        pk_struct,
        pk_parameter: pk_parameter(),
        fields: fields.clone(),
        updateable_columns_enum,
        updateable_columns_enum_parameter: format_ident!("dyn_column"),
        materialized_view: mv,
    };

    let pk_ts = writer.write_pk(&inf);

    let is_materialized_view = inf.materialized_view.is_some();

    let mut base_ts = BaseTableTokenstream {
        update: Default::default(),
        delete: Default::default(),
        insert: Default::default(),
    };

    if !is_materialized_view {
        // Materialized views rows can not be inserted, deleted or updated
        base_ts.insert = crate::crud::insert::generate(&inf, &writer);
        base_ts.delete = crate::crud::delete::generate(&inf, &writer);
        base_ts.update = crate::crud::update::generate(&inf, &writer);
    }

    let select_ts = crate::crud::select::generate(&inf, &writer);

    writer.post_process_ts(
        select_ts,
        pk_ts,
        if is_materialized_view {
            None
        } else {
            Some(base_ts)
        },
    )
}
