use crate::crud::crud_operation::CrudOperation;
use crate::crud::{ColumnValue, ExtractColumn, QueryType};

pub(crate) struct Select;

impl CrudOperation for Select {
    fn crud_query_start(&self) -> &'static str {
        "select"
    }

    fn table_name_after(&self) -> &'static str {
        "from "
    }

    fn column_clauses(&self, query: &str) -> ExtractColumn {
        let select = "select ";
        let from_index = query.find(" from").unwrap();
        let select_index = query.find(select).unwrap();
        let select_clause = &query[select_index + select.len()..from_index];

        if select_clause == "*" || select_clause.contains("count(") {
            return vec![];
        }

        select_clause
            .split(", ")
            .map(|c| {
                let alias_split = c.split(" as ").collect::<Vec<_>>();

                ColumnValue {
                    column_name: alias_split.first().unwrap().to_string(),
                    parameterized: false,
                    uses_in_value: false,
                    is_part_of_where_clause: false,
                }
            })
            .collect()
    }

    fn query_type(&self, query: &str, full_pk: bool) -> QueryType {
        if query.contains(" in (?)") {
            panic!("An in query should invoked by doing 'in ?', not 'in (?)', since this always select only 0/1 rows");
        }

        let query_is_limited_by_one = query.ends_with(" limit 1");
        let counts = query.contains("count(");

        if full_pk {
            assert!(!query_is_limited_by_one);
            assert!(
                !counts,
                "Counting a query which only returns 0 or 1 row, query: {}",
                query
            );
            QueryType::SelectUnique
        } else if query_is_limited_by_one {
            QueryType::SelectUniqueByLimit
        } else if counts {
            QueryType::SelectCount
        } else {
            QueryType::SelectMultiple
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_columns_select_clause() {
        let select = Select;
        let q = select.column_clauses("select * from table");

        assert_eq!(0, q.len());

        let q = select.column_clauses("select 1 from table");

        assert_eq!(1, q.len());
        assert_eq!("1", &q[0].column_name);

        let q = select.column_clauses("select 1, 2 as j, 3 from table");

        assert_eq!(3, q.len());
        assert_eq!("1", &q[0].column_name);
        assert_eq!("2", &q[1].column_name);
        assert_eq!("3", &q[2].column_name);
    }
}
