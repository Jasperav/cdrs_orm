use cdrs_query_writer::{Inf, Update, COLUMN_SEPARATOR, CRUD};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct ImplWriter;

const INSERT_UNIQUE: &str = "c_insert_unique";
const SELECT_UNIQUE: &str = "c_select_unique";
const SELECT_ALL: &str = "c_select_all";
const DELETE_UNIQUE: &str = "c_delete_unique";
const UPDATE_OPTIONALS: &str = "c_update_opt";
const UPDATE_COLUMN: &str = "c_update";

impl cdrs_query_writer::Writer for ImplWriter {
    fn write_pk(&self, inf: &Inf) -> TokenStream {
        assert_eq!("SomeStruct".to_string(), inf.name.to_string());
        assert_eq!("SomeStruct", inf.table_name);
        assert_eq!(
            "SomeStructPrimaryKey".to_string(),
            inf.pk_struct.to_string()
        );
        assert_eq!(1, inf.non_pk_fields.len());
        assert_eq!(5, inf.fields.len());
        assert_eq!(4, inf.pk_fields.len());
        assert_eq!(2, inf.clustering_fields.len());
        assert_eq!(2, inf.partition_fields.len());

        TokenStream::new()
    }

    fn write(
        &self,
        inf: &Inf,
        db_mirror_fn_name: &Ident,
        custom_fn_name: &Ident,
        crud: CRUD,
    ) -> TokenStream {
        let db_mirror_fn_name_str = db_mirror_fn_name.to_string();
        let custom_fn_name_str = custom_fn_name.to_string();

        match crud {
            CRUD::InsertUnique => {
                assert_eq!(cdrs_query_writer::INSERT, db_mirror_fn_name_str);
                assert_eq!(INSERT_UNIQUE, custom_fn_name_str);
            }
            CRUD::UpdateUnique(update) => {
                match update {
                    Update::SingleColumn((_, _)) => {
                        assert!(db_mirror_fn_name_str
                            .starts_with(cdrs_query_writer::UPDATE_SINGLE_COLUMN));
                        assert!(custom_fn_name_str.starts_with(UPDATE_COLUMN));
                    }
                    Update::AllOptional((_, _)) => {
                        assert_eq!(cdrs_query_writer::UPDATE_OPTIONALS, db_mirror_fn_name_str);
                        assert_eq!(UPDATE_OPTIONALS, custom_fn_name_str);
                    }
                    Update::Dynamic(_) | Update::DynamicVec(_) => {
                        // Do nothing
                        return TokenStream::new();
                    }
                }
            }
            CRUD::SelectUnique => {
                assert_eq!(cdrs_query_writer::SELECT_UNIQUE, db_mirror_fn_name_str);
                assert_eq!(SELECT_UNIQUE, custom_fn_name_str);
            }
            CRUD::DeleteUnique => {
                assert_eq!(cdrs_query_writer::DELETE_UNIQUE, db_mirror_fn_name_str);
                assert_eq!(DELETE_UNIQUE, custom_fn_name_str);
            }
            CRUD::SelectAll => {
                assert_eq!(cdrs_query_writer::SELECT_ALL, db_mirror_fn_name_str);
                assert_eq!(SELECT_ALL, custom_fn_name_str);
            }
        }

        let name = inf.name;

        quote! {
            impl #name {
                pub fn #custom_fn_name() {
                    // Empty method, just to check if the method is added correctly in the test
                    // Normally, you will custom logic here
                }
            }
        }
    }

    fn fn_name_insert(&self) -> &'static str {
        INSERT_UNIQUE
    }

    fn fn_name_select_unique(&self) -> &'static str {
        SELECT_UNIQUE
    }

    fn fn_name_delete_unique(&self) -> &'static str {
        DELETE_UNIQUE
    }

    fn fn_name_update_optionals(&self) -> &'static str {
        UPDATE_OPTIONALS
    }

    fn fn_name_update_column(&self, column: &str) -> String {
        format!("{}{}{}", UPDATE_COLUMN, COLUMN_SEPARATOR, column)
    }

    fn fn_name_select_all(&self) -> &'static str {
        SELECT_ALL
    }
}
