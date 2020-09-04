use crate::capitalizing::table_name_to_struct_name;
use crate::crud::{ColumnValue, ExtractColumn, QueryType};
use crate::supported_data_types::CassandraDataType;
use crate::{create_test_db_session, keyspace, query_columns, use_keyspace, Columns};
use cdrs::query::QueryExecutor;
use cdrs::types::value::Value;
use std::collections::HashSet;

/// Meta data of a query
#[derive(PartialEq, Debug)]
pub struct QueryMetaData {
    /// The columns that are used in this query
    pub extracted_columns: ExtractColumn,
    /// Parameterized columns
    pub parameterized_columns_data_types: Vec<CassandraDataType>,
    pub query_type: QueryType,
    /// The corresponding Rust struct name of the query
    pub struct_name: String,
    /// Only true if the query is limited
    pub limited: bool,
}

/// Extract the query meta data from a query
pub fn extract_query_meta_data<S: AsRef<str> + std::fmt::Display>(query: &S) -> QueryMetaData {
    let as_str = query.as_ref();
    let crud = crate::crud::create_query_crud(as_str);
    let table_name = crate::crud::extract_table_name(&query, &*crud);

    // TODO: Maybe a singleton connection performs better
    let session = create_test_db_session();

    use_keyspace(&session, &keyspace());

    let columns = query_columns(&session, table_name);

    if columns.is_empty() {
        panic!("Table '{}' in keyspace '{}' does not exists (or does not have columns, which is useless)", table_name, keyspace());
    }

    let extracted_columns = crate::crud::extract_columns(query.as_ref(), &*crud);

    if query.as_ref().starts_with("insert") && extracted_columns.len() != columns.len() {
        panic!("Insert query is missing values")
    }

    let parameterized_columns =
        check_subset_and_keep_parameterized_columns(&columns, &extracted_columns);

    let mut parameterized_columns_data_types = parameterized_columns
        .iter()
        .map(|c| CassandraDataType::new(c.data_type.as_str()))
        .collect::<Vec<_>>();

    if as_str.ends_with(" limit ?") {
        // parameterized_columns_data_types does not contain the limit type, add it back
        parameterized_columns_data_types.push(CassandraDataType::Int);
    }

    // Columns can be reused in ranges, so filter duplicates
    let unique_columns_where_clause = extracted_columns
        .iter()
        .map(|r| r.column_name.clone())
        .collect::<HashSet<_>>();
    // For this variable a hashset is also used although it will not filter any elements
    // but is used for comparing later on
    let unique_columns = columns
        .iter()
        .filter(|r| r.kind().is_part_of_pk())
        .map(|r| r.column_name.clone())
        .collect::<HashSet<_>>();
    let columns_in_where_all_pk = unique_columns_where_clause.eq(&unique_columns);

    // Maybe a range is added to the last column, or an 'in' query
    let where_pattern = " where ";
    let is_selecting_unique = if let Some(start) = as_str.find(where_pattern) {
        let end = as_str.find(" limit ").unwrap_or(as_str.len() - 1);
        let slice = &as_str[start..end];

        slice.chars().filter(|i| i == &'=').count() == unique_columns.len()
    } else {
        as_str.ends_with("limit 1")
    };

    let is_full_pk = columns_in_where_all_pk && is_selecting_unique;

    if as_str.contains("count(") && as_str.contains(" limit ") {
        panic!("Both using count and limit is strange")
    }

    QueryMetaData {
        extracted_columns,
        parameterized_columns_data_types,
        query_type: crud.query_type(query.as_ref(), is_full_pk),
        struct_name: table_name_to_struct_name(table_name),
        limited: as_str.contains(" limit "),
    }
}

/// Tests is a query is correct
/// If not, it will panic
pub fn test_query<S: AsRef<str> + std::fmt::Display>(query: S) -> QueryMetaData {
    let qmd = extract_query_meta_data(&query);

    let values = qmd
        .parameterized_columns_data_types
        .iter()
        .map(|cdr| random_value_for_cs_type(cdr))
        .collect::<Vec<_>>();

    let session = create_test_db_session();

    use_keyspace(&session, &keyspace());

    // Execute the query with test values
    if let Err(e) = session.query_with_values(
        &query,
        cdrs::query::QueryValues::SimpleValues(values.clone()),
    ) {
        panic!(
            "Query failed: \nQuery: {}\nResult: {:#?}\nValues: {:#?}",
            query, e, values
        );
    }

    qmd
}

/// Generates a random value for a given data type
fn random_value_for_cs_type(cdt: &CassandraDataType) -> Value {
    match cdt {
        CassandraDataType::TinyInt => i8::MAX.into(),
        CassandraDataType::SmallInt => i16::MAX.into(),
        CassandraDataType::Int => i32::MAX.into(),
        CassandraDataType::BigInt
        | CassandraDataType::Time
        | CassandraDataType::Timestamp
        | CassandraDataType::Counter => i64::MAX.into(),
        CassandraDataType::Text | CassandraDataType::Ascii | CassandraDataType::Varchar => {
            "_VALUE_FOR_QUERY_VALUE_TESTING".into()
        }
        CassandraDataType::Boolean => true.into(),
        CassandraDataType::Float => f32::MAX.into(),
        CassandraDataType::Double => f64::MAX.into(),
        CassandraDataType::Uuid => uuid::Uuid::parse_str("3866a82f-f37c-446c-8838-fb6686c3acf2")
            .unwrap()
            .into(),
    }
}

/// Checks if all the used columns in the query are present in the table itself
/// and after that, filter out only parameterized column values
fn check_subset_and_keep_parameterized_columns<'a>(
    columns: &'a [Columns],
    columns_used_in_query: &[ColumnValue],
) -> Vec<&'a Columns> {
    columns_used_in_query
        .iter()
        // First check if all the columns that are used are in the table definition
        .map(|cq| {
            (
                columns
                    .iter()
                    .find(|c| c.column_name.as_str() == cq.column_name.as_str())
                    .unwrap_or_else(|| panic!("Illegal column: {}", cq.column_name)),
                cq,
            )
        })
        // Only keep the parameterized values, since random values needs to be generated for that
        .filter(|(_, cq)| cq.parameterized)
        .map(|(c, _)| c)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::crud::ColumnValue;
    use crate::{query, setup_test_keyspace, TEST_TABLE};

    #[test]
    fn test_extract_query_meta_data() {
        let _ = setup_test_keyspace();

        let qmd =
            extract_query_meta_data(&"select * from test_table where a = ? and b = 1 limit ?");

        assert_eq!(
            QueryMetaData {
                extracted_columns: vec![
                    ColumnValue {
                        column_name: "a".to_string(),
                        parameterized: true,
                        uses_in_value: false,
                        is_part_of_where_clause: true,
                    },
                    ColumnValue {
                        column_name: "b".to_string(),
                        parameterized: false,
                        uses_in_value: false,
                        is_part_of_where_clause: true,
                    }
                ],
                parameterized_columns_data_types: vec![
                    CassandraDataType::Int,
                    CassandraDataType::Int
                ],
                query_type: QueryType::SelectMultiple,
                struct_name: "TestTable".to_string(),
                limited: true,
            },
            qmd
        );
    }

    #[test]
    #[should_panic]
    fn test_count_and_limit_single_query() {
        let _session = setup_test_keyspace();

        test_query(format!("select count(*) from {} limit 1", TEST_TABLE));
    }

    #[test]
    fn test_test_query() {
        let _session = setup_test_keyspace();

        test_query(format!(
            "select * from {} where b = 1 and c = ?",
            TEST_TABLE
        ));
        test_query(format!(
            "select * from {} where b = 1 and c = 1",
            TEST_TABLE
        ));

        wrap_failing_query(format!(
            "select * from {} where a = 1 and c = 1",
            TEST_TABLE
        ));
        wrap_failing_query(format!(
            "select * from {} where a = ? and c = 1",
            TEST_TABLE
        ));
    }

    #[test]
    fn test_uuid() {
        let session = setup_test_keyspace();

        query(
            &session,
            "create table if not exists UUIDTable(u uuid, primary key((u)))",
        );

        test_query("select * from UUIDTable where u = ?");
    }

    fn wrap_failing_query<S: AsRef<str> + std::fmt::Display + std::panic::UnwindSafe>(query: S) {
        assert!(std::panic::catch_unwind(|| { test_query(query) }).is_err());
    }

    #[test]
    fn test_query_to_table_name() {
        let table_name = "TABLE";
        let check = |t: String| {
            let crud = crate::crud::create_query_crud(&t);

            assert_eq!(table_name, crate::crud::extract_table_name(&t, &*crud));
        };

        check(format!("select * from {}", table_name));
        check(format!(
            "select test, test2 from {} where pk = 1",
            table_name
        ));

        check(format!("update {}", table_name));
        check(format!("update {} set a = 1", table_name));
        check(format!("update {} set a = 1 where a = 2", table_name));

        check(format!("insert into {} (a) values (1)", table_name));

        check(format!("delete from {} where 1 = 1", table_name));
    }

    fn check_subset_columns() -> Vec<Columns> {
        vec![
            Columns {
                column_name: "a".to_string(),
                kind: "".to_string(),
                position: 0,
                data_type: "".to_string(),
            },
            Columns {
                column_name: "b".to_string(),
                kind: "".to_string(),
                position: 0,
                data_type: "".to_string(),
            },
        ]
    }

    fn create_columns_used_in_query(value: &str) -> ExtractColumn {
        vec![ColumnValue {
            column_name: value.to_string(),
            parameterized: true,
            uses_in_value: false,
            is_part_of_where_clause: false,
        }]
    }

    #[test]
    fn test_check_subset() {
        let c = check_subset_columns();
        let r = check_subset_and_keep_parameterized_columns(&c, &create_columns_used_in_query("a"));

        assert_eq!(1, r.len());
        assert_eq!("a", r[0].column_name.as_str());
    }

    #[test]
    #[should_panic]
    fn test_check_subset_fail() {
        check_subset_and_keep_parameterized_columns(
            &check_subset_columns(),
            &create_columns_used_in_query("c"),
        );
    }
}
