mod crud_operation;
mod delete;
mod insert;
mod select;
mod truncate;
mod update;

use crate::crud::crud_operation::CRUDOperation;
use delete::*;
use insert::*;
use select::*;
use truncate::*;
use update::*;

/// Represents a column that is used in a query
#[derive(Debug, PartialEq)]
pub struct ColumnValue {
    pub column_name: String,
    /// If true, the column is assigned a question mark (... where a = ?)
    /// If false, the column has a fixed value (... where a = 1)
    pub parameterized: bool,
    /// Only true if the column uses an in value (... where a in ?)
    pub uses_in_value: bool,
    /// Only true if the column is used in the where clause
    /// False if e.g. the column is part of the select clause
    pub is_part_of_where_clause: bool,
}

pub type ExtractColumn = Vec<ColumnValue>;

/// Dynamically choose from a given query the correct CRUD type
pub(crate) fn create_query_crud(query: &str) -> Box<dyn CRUDOperation> {
    let cruds: Vec<Box<dyn CRUDOperation>> = vec![
        Box::new(Select),
        Box::new(Update),
        Box::new(Insert),
        Box::new(Delete),
        Box::new(Truncate),
    ];

    cruds
        .into_iter()
        .find(|c| query.starts_with(c.crud_query_start()))
        .expect("Queries should start with select, update, delete, insert or truncate")
}

/// The different types of a query
#[derive(PartialEq, Debug)]
pub enum QueryType {
    /// Select multiple rows
    SelectMultiple,
    /// Selects single row because it ends with limit 1
    SelectUniqueByLimit,
    /// Select a unique row
    SelectUnique,
    /// Selects a count
    SelectCount,
    /// Updates a row
    /// Note: this is always on full primary key
    UpdateUnique,
    /// Deletes multiple rows
    DeleteMultiple,
    /// Deletes a unique row
    DeleteUnique,
    /// Inserts a single row
    InsertUnique,
    /// Truncates a table
    Truncate,
}

/// Extracts a table name from a query
///
/// use cdrs_con::crud::extract_table_name;
/// let query = "select * from my_table where a = 1";
/// let boxed = Box::new(Select);
/// let table_name = extract_table_name(query, &boxed);
/// assert_eq!("my_table", table_name);
///
pub(crate) fn extract_table_name<'a, S: AsRef<str>>(
    query: &'a S,
    crud: &'a dyn CRUDOperation,
) -> &'a str {
    let index = query
        .as_ref()
        .find(crud.table_name_after())
        .expect("No table name found in query")
        + crud.table_name_after().len()
        - 1;
    let suffix = &query.as_ref()[index + 1..];
    let end = suffix.find(' ').unwrap_or_else(|| suffix.len());

    &suffix[..end]
}

/// Extracts the used columns in a query
/// Note: cant use a set because the same order needs to be remained
///
/// use cdrs_con::crud::extract_columns;
/// let query = "select b, c from my_table where a = 1";
/// let boxed = Box::new(Select);
/// let columns = extract_columns(query, &boxed);
///
/// assert_eq!(3, columns.len());
/// assert_eq!("b", &columns[0].column_name);
/// assert_eq!(false, &columns[0].is_part_of_where_clause);
/// assert_eq!("c", &columns[1].column_name);
/// assert_eq!(false, &columns[1].parameterized);
/// assert_eq!("a", &columns[2].column_name);
/// assert_eq!(true, &columns[2].is_part_of_where_clause);
///
pub(crate) fn extract_columns(query: &str, crud: &dyn CRUDOperation) -> ExtractColumn {
    let (query, query_without_where) = split_query(query);

    crud.column_clauses(query_without_where)
        .into_iter()
        .chain(columns_after_where(query))
        .collect()
}

/// Splits the query in a tuple
/// The first element is the query without the limit clause (if any)
/// The second element is the query without the limit and where clause
/// ```
/// use cdrs_con::crud::split_query;
/// let query = "select * from my_table where a = 1 limit 5";
/// let (q, p) = split_query(query);
///
/// assert_eq!("select * from my_table where a = 1", q);
/// assert_eq!("select * from my_table", p);
/// ```
pub fn split_query(q: &str) -> (&str, &str) {
    let mut query = q;

    // Remove the 'limit' if present
    if let Some(i) = query.rfind(" limit ") {
        query = &query[..i]
    }

    let where_clause = query.find(" where ").unwrap_or_else(|| query.len());

    (query, &query[..where_clause])
}

/// Extracts columns that are used in the where clause
/// ```
/// use cdrs_con::crud::columns_after_where;
/// let query = "select * from my_table where a = ? and c in ?";
/// let extracted = columns_after_where(query);
/// assert_eq!(2, extracted.len());
/// assert_eq!("a", &extracted[0].column_name);
/// assert_eq!("c", &extracted[1].column_name);
/// assert!(!extracted[0].uses_in_value);
/// assert!(extracted[1].uses_in_value);
/// ```
pub fn columns_after_where(query: &str) -> ExtractColumn {
    let w = " where ";
    let index = match query.find(w) {
        None => return vec![],
        Some(i) => i,
    };

    let mut suffix = query[index + w.len()..].to_string();
    let mut column_values = vec![];
    let operators_before_column_name = vec![" = ", " >= ", " > ", " <= ", " < ", " in "];

    loop {
        let mut operator_with_lowest_index = None;

        for operator in operators_before_column_name.iter() {
            if let Some(s) = suffix.find(operator) {
                match operator_with_lowest_index {
                    None => operator_with_lowest_index = Some((s, operator)),
                    Some((o, _)) => {
                        if s < o {
                            operator_with_lowest_index = Some((s, operator))
                        }
                    }
                }
            }
        }

        match operator_with_lowest_index {
            None => break,
            Some((index, operator)) => {
                let and = " and ";
                let val = &suffix[suffix.find(operator).unwrap() + operator.len()
                    ..suffix.find(and).unwrap_or_else(|| suffix.len())];

                let cv = ColumnValue {
                    column_name: suffix[..index].to_string(),
                    parameterized: val.contains('?'),
                    uses_in_value: operator == &" in ",
                    is_part_of_where_clause: true,
                };

                column_values.push(cv);

                if let Some(p) = suffix.find(and) {
                    suffix = suffix[p + and.len()..].to_string();
                } else {
                    break;
                }
            }
        }
    }

    column_values
}

#[test]
fn test_query_before_process() {
    let query = "select * from some_table where a = 1 and b = ?";
    let before_where = "select * from some_table";
    let (query_extracted, before_where_extracted) = split_query(query);

    assert_eq!(query, query_extracted);
    assert_eq!(before_where, before_where_extracted);

    let (query_extracted, before_where_extracted) =
        split_query("select * from some_table where a = 1 and b = ? limit 100");

    assert_eq!(
        "select * from some_table where a = 1 and b = ?",
        query_extracted
    );
    assert_eq!(before_where, before_where_extracted);

    let query = "insert into my_table(a, b) values (1, ?)";
    let (query_extracted, before_where_extracted) = split_query(query);

    assert_eq!(query, query_extracted);
    assert_eq!(query, before_where_extracted);
}

#[test]
fn test_columns_after_where() {
    let v = columns_after_where("");

    assert!(v.is_empty());

    let v = columns_after_where("select * from dummy");

    assert!(v.is_empty());

    let v = columns_after_where("select * from dummy where a = 1");

    assert_eq!("a", &v[0].column_name);
    assert!(!v[0].parameterized);
    assert!(!v[0].uses_in_value);

    let v = columns_after_where("select * from dummy where a = ?");

    assert_eq!("a", &v[0].column_name);
    assert!(v[0].parameterized);
    assert!(!v[0].uses_in_value);

    let v = columns_after_where("select * from dummy where a in ?");

    assert_eq!("a", &v[0].column_name);
    assert!(v[0].parameterized);
    assert!(v[0].uses_in_value);

    let v = columns_after_where("select * from dummy where a = 1 and b > 0 and c <= somethingrandom and d < ? and e in (hi) and f = 2");

    assert_eq!("a", &v[0].column_name);
    assert_eq!("b", &v[1].column_name);
    assert_eq!("c", &v[2].column_name);
    assert_eq!("d", &v[3].column_name);
    assert_eq!("e", &v[4].column_name);
    assert_eq!("f", &v[5].column_name);
    assert!(!v[0].parameterized);
    assert!(!v[1].parameterized);
    assert!(!v[2].parameterized);
    assert!(v[3].parameterized);
    assert!(!v[4].parameterized);
    assert!(!v[5].parameterized);

    let v =
        columns_after_where("select * from test_table where b = ? and c = 5 and d in ? limit 1");

    assert!(!v[0].uses_in_value);
    assert!(v[0].parameterized);
    assert!(!v[1].uses_in_value);
    assert!(!v[1].parameterized);
    assert!(v[2].uses_in_value);
    assert!(v[2].parameterized);
}

#[test]
fn test_extract_columns() {
    let select: Box<dyn CRUDOperation> = Box::new(Select);

    let c = extract_columns(
        "select a, b as c, d from table where a = 1 and b > 2 and e in ? limit ?",
        &*select,
    );

    assert_eq!(6, c.len());
    assert_eq!("a", &c[0].column_name);
    assert_eq!("b", &c[1].column_name);
    assert_eq!("d", &c[2].column_name);
    assert_eq!("a", &c[3].column_name);
    assert_eq!("b", &c[4].column_name);
    assert_eq!("e", &c[5].column_name);
}
