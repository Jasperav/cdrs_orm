use crate::crud::crud_operation::CRUDOperation;
use crate::crud::{ColumnValue, ExtractColumn, QueryType};

pub(crate) struct Update;

impl CRUDOperation for Update {
    fn crud_query_start(&self) -> &'static str {
        "update"
    }

    fn table_name_after(&self) -> &'static str {
        "update "
    }

    fn column_clauses(&self, query: &str) -> ExtractColumn {
        let s = " set ";
        let set = query.find(s).unwrap();
        let update = &query[set + s.len()..];

        update
            .split(", ")
            .map(|u| {
                let split = u.split(" = ").collect::<Vec<_>>();

                ColumnValue {
                    column_name: split[0].to_string(),
                    parameterized: split[1] == "?",
                    uses_in_value: false,
                    is_part_of_where_clause: false,
                }
            })
            .collect()
    }

    fn query_type(&self, _query: &str, _full_pk: bool) -> QueryType {
        QueryType::UpdateUnique
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_columns_update_clause() {
        let update = Update;
        let q = update.column_clauses("update table set a = 1");

        assert_eq!(1, q.len());
        assert_eq!("a", &q[0].column_name);

        let q = update.column_clauses("update table set a = 1, b = ?, c = 3");

        assert_eq!(3, q.len());
        assert_eq!("a", &q[0].column_name);
        assert_eq!("b", &q[1].column_name);
        assert_eq!("c", &q[2].column_name);
        assert!(!q[0].parameterized);
        assert!(q[1].parameterized);
        assert!(!q[2].parameterized);

        let q = update.column_clauses("update table set a = ?, b = ?");

        assert!(q[0].parameterized);
        assert!(q[1].parameterized);
    }
}
