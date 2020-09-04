use crate::crud::crud_operation::CRUDOperation;
use crate::crud::{ExtractColumn, QueryType};

pub(crate) struct Delete;

impl CRUDOperation for Delete {
    fn crud_query_start(&self) -> &'static str {
        "delete"
    }

    fn table_name_after(&self) -> &'static str {
        "from "
    }

    fn column_clauses(&self, _query: &str) -> ExtractColumn {
        vec![]
    }

    fn query_type(&self, _query: &str, full_pk: bool) -> QueryType {
        if full_pk {
            QueryType::DeleteUnique
        } else {
            QueryType::DeleteMultiple
        }
    }
}
