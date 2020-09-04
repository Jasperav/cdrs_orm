use crate::capitalizing::table_name_to_struct_name;
use crate::{keyspace, query, query_columns, rows, DbTestSession};
use cdrs::query::QueryExecutor;
use std::collections::HashSet;

/// Information about a materialized view
#[derive(cdrs_helpers_derive::TryFromRow)]
pub struct MaterializedView {
    /// The table name of the materialized view
    pub table_name: String,
    /// The table name where the materialized view is based on
    pub base_table_name: String,
}

/// Creates query that can be used to query all the materialized views from the database
pub fn query_for_materialized_view() -> String {
    format!(
        "select view_name as table_name, base_table_name from system_schema.views where keyspace_name = '{}'",
        keyspace()
    )
}

/// Queries all the materialized views from the database
pub fn query_materialized_views(session: &DbTestSession) -> Vec<MaterializedView> {
    rows(session.query(query_for_materialized_view()))
}

/// Detailed information about a materialized view
#[derive(Clone, PartialEq, Debug)]
pub struct MaterializedViewInf {
    /// The corresponding Rust struct name where this materialized view should belong to
    pub struct_name: String,
    /// The table name where the materialized view is based on
    pub base_table_name: String,
    /// Only true if the materialized view has exactly the same columns as the base table
    pub same_columns: bool,
}

/// Queries a specific materialized view, and gives back detailted information about the materialized view
pub fn query_materialized_view(
    session: &DbTestSession,
    table_name: &str,
) -> Option<MaterializedViewInf> {
    let rows: Vec<MaterializedView> = rows(Ok(query(
        &session,
        (query_for_materialized_view() + format!(" and view_name = '{}'", table_name).as_str())
            .as_str(),
    )));

    if rows.is_empty() {
        return None;
    }

    assert_eq!(1, rows.len());

    let mv = rows.first().unwrap();
    let query_column_names = |table_name| {
        query_columns(&session, table_name)
            .into_iter()
            .map(|r| r.column_name)
            .collect::<HashSet<_>>()
    };
    // Check if the base table has the same columns
    let columns_base = query_column_names(&mv.base_table_name);
    let columns_own = query_column_names(&table_name);

    Some(MaterializedViewInf {
        base_table_name: mv.base_table_name.clone(),
        struct_name: table_name_to_struct_name(&mv.base_table_name),
        same_columns: columns_base.eq(&columns_own),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{setup_test_keyspace, TEST_TABLE};

    #[test]
    fn materialized_view() {
        let s = setup_test_keyspace();
        let test_table_mv = format!("{}_mv", TEST_TABLE);
        let drop_mv = || {
            query(
                &s,
                &format!("drop materialized view if exists {}", test_table_mv),
            );
        };

        drop_mv();

        let assert = |val| assert_eq!(val, query_materialized_view(&s, &test_table_mv));

        assert(None);

        let add_materialized_view = |select| {
            query(&s,
                  &format!("create materialized view {} as
                                select {}
                                from {}
                                where b is not null and c is not null and a is not null and d is not null
                                primary key ((b), c, d, a)", &test_table_mv, select, TEST_TABLE));
        };

        add_materialized_view("*");

        let mv_inf = |same_columns| MaterializedViewInf {
            base_table_name: TEST_TABLE.to_string(),
            struct_name: table_name_to_struct_name(TEST_TABLE),
            same_columns,
        };

        assert(Some(mv_inf(true)));

        drop_mv();

        add_materialized_view("a, b, c, d");

        assert(Some(mv_inf(false)));
    }
}
