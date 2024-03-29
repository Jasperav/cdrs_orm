use crate::crud::crud_operation::CrudOperation;
use crate::crud::ExtractColumn;
use crate::QueryType;

pub(crate) struct Truncate;

impl CrudOperation for Truncate {
    fn crud_query_start(&self) -> &'static str {
        "truncate"
    }

    fn table_name_after(&self) -> &'static str {
        " "
    }

    fn column_clauses(&self, query: &str) -> ExtractColumn {
        // Check if a table is followed after 'truncate' and nothing more
        // A check on a single whitespace is enough
        let table = &query[query.find(' ').unwrap() + 1..];
        // If there is another whitespace, it is an invalid query
        if table.contains(' ') {
            panic!(
                "This truncate query contains multiple whitespaces, but only 1 is allowed: '{}'",
                query
            );
        }

        // No columns are present in a truncate query
        vec![]
    }

    fn query_type(&self, _query: &str, full_pk: bool) -> QueryType {
        assert!(!full_pk);

        QueryType::Truncate
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_columns_truncate() {
        let select = Truncate;
        let _ = select.column_clauses("truncate my_table");
    }

    #[test]
    #[should_panic]
    fn test_columns_truncate_invalid() {
        let select = Truncate;
        let _ = select.column_clauses("truncate my_table where a = 1");
    }
}
