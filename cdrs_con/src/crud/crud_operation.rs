use crate::crud::{ExtractColumn, QueryType};

/// Trait that is implemented for every CRUD operation
pub(crate) trait CrudOperation {
    /// Is either insert, select, update or delete
    /// Based on this value, the correct CRUD operation can be determined from a query
    fn crud_query_start(&self) -> &'static str;
    /// Is either from, into or update
    /// Based on this value, the table name can be determined to execute the CRUD operation for
    fn table_name_after(&self) -> &'static str;
    /// Determines all the columns that are used in the query
    fn column_clauses(&self, query: &str) -> ExtractColumn;
    /// Determines the query type for the query
    /// parameter full_pk means if the query parameter contains the full primary key
    fn query_type(&self, query: &str, full_pk: bool) -> QueryType;
}
