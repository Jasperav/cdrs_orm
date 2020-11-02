use std::env;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::new as new_session;
use cdrs::cluster::session::Session;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};

use cdrs::frame::Frame;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::{QueryExecutor, QueryValues};
use cdrs::types::prelude::*;

pub type DbTestSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

/// Set this environment variable to true if table names are snake cased
pub const CAMEL_CASE_TO_DB_SNAKE_CASE_KEY: &str = "CAMEL_CASE_TO_DB_SNAKE_CASE";
/// Mandatory: the keyspace to use when validating queries and mapping table names to Rust structures.
/// Note: this crate can add rows! Make sure this is a test keyspace or development environment
pub const TEST_CDRS_DB_KEYSPACE_KEY: &str = "TEST_CDRS_DB_KEYSPACE_KEY";
/// Defaults to 127.0.0.1:9042
pub const CDRS_DATABASE_URL_KEY: &str = "TEST_CDRS_DB_URL";

/// Default keyspace for running tests
pub const TEST_CDRS_DB_KEYSPACE: &str = "test_keyspace_for_testing";
/// This is not an environment variable but the name of a table that is used for testing
pub const TEST_TABLE: &str = "test_table";

pub mod crud;
mod query_executor;
pub mod supported_data_types;

pub use crud::QueryType;
pub use query_executor::{test_query, QueryMetaData};

pub mod capitalizing;
pub mod materialized_view;

/// The type of the column
#[derive(PartialEq, Debug)]
pub enum ColumnKind {
    /// Column is part of the partition key
    PartitionKey,
    /// Column is part of the clustering key
    Clustering,
    /// Column is not part of the primary key
    Regular,
}

impl ColumnKind {
    pub fn is_part_of_pk(&self) -> bool {
        match self {
            ColumnKind::PartitionKey | ColumnKind::Clustering => true,
            ColumnKind::Regular => false,
        }
    }
}

impl ToString for ColumnKind {
    fn to_string(&self) -> String {
        let column_kind_to_string = match self {
            ColumnKind::PartitionKey => "partition_key",
            ColumnKind::Regular => "regular",
            ColumnKind::Clustering => "clustering",
        };

        column_kind_to_string.to_string()
    }
}

/// Meta data about a column for a table
#[derive(Debug, cdrs_helpers_derive::TryFromRow)]
pub struct Columns {
    /// Name of the column
    pub column_name: String,
    /// Either partition_key, regular or clustering
    pub kind: String,
    /// The position of the column, only relevant for primary key fields.
    /// Regular columns have the value -1, primary key fields >= 0.
    pub position: i32,
    /// The data type of the column
    pub data_type: String,
}

impl Columns {
    pub fn kind(&self) -> ColumnKind {
        match self.kind.as_str() {
            "partition_key" => ColumnKind::PartitionKey,
            "regular" => ColumnKind::Regular,
            "clustering" => ColumnKind::Clustering,
            _ => panic!("Invalid column type: {}", self.kind.as_str()),
        }
    }
}

/// Creates a quick and simple database session, ideally for testing purposes
pub fn create_test_db_session() -> DbTestSession {
    let database_url =
        env::var(CDRS_DATABASE_URL_KEY).unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    // Connect to the database
    let node = NodeTcpConfigBuilder::new(&database_url, NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);

    new_session(&cluster_config, RoundRobin::new()).expect("session should be created")
}

/// Sets a keyspace for a session
pub fn use_keyspace(session: &DbTestSession, keyspace: &str) {
    session.query(format!("use {}", keyspace)).unwrap();
}

/// Query the database
pub fn query(session: &DbTestSession, query: &str) -> Frame {
    session.query(query).unwrap()
}

/// Query the database with values
pub fn query_with_values(session: &DbTestSession, query: &str, values: QueryValues) -> Frame {
    session.query_with_values(query, values).unwrap()
}

/// Query columns for a given table
pub fn query_columns(session: &DbTestSession, table: &str) -> Vec<Columns> {
    rows(session.query(format!("select column_name, clustering_order, kind, position, type as data_type from system_schema.columns where keyspace_name = '{}' and table_name = '{}'", keyspace(), table.to_lowercase())))
}

/// Removes and recreates a keyspace.
/// Note: it does not check keyspace configuration, the new keyspace has default configuration.
pub fn recreate_keyspace(session: &DbTestSession, keyspace: &str) {
    session
        .query(format!("drop keyspace if exists {}", keyspace))
        .unwrap();
    session
        .query(format!(
            "create keyspace {} with replication = {} and durable_writes = false",
            keyspace, "{'class': 'SimpleStrategy', 'replication_factor': 1}"
        ))
        .unwrap();

    use_keyspace(session, keyspace);
}

/// Sets up a test keyspace
pub fn setup_test_keyspace() -> DbTestSession {
    let session = create_test_db_session();

    prepare_test_keyspace(&session);

    session
}

/// Can recreate or reuse a keyspace and will create a dummy table for testing
pub fn prepare_test_keyspace(session: &DbTestSession) {
    if keyspace_tables(session).is_empty() {
        recreate_keyspace(session, TEST_CDRS_DB_KEYSPACE);
    } else {
        use_keyspace(session, TEST_CDRS_DB_KEYSPACE);
    }

    create_dummy_table(session);
}

pub fn keyspace() -> String {
    env::var(TEST_CDRS_DB_KEYSPACE_KEY).unwrap_or_else(|_| {
        panic!(
            "Add env property {} which determines the schema to use",
            TEST_CDRS_DB_KEYSPACE_KEY
        )
    })
}

/// Transforms a result from a query into rows of the specified type
pub fn rows<T: TryFromRow>(result: cdrs::Result<Frame>) -> Vec<T> {
    result
        .expect("Failed to execute query")
        .get_body()
        .expect("Failed to get body")
        .into_rows()
        .expect("Failed to turn into rows")
        .into_iter()
        .map(|row| T::try_from_row(row).expect("Failed to turn query results into struct"))
        .collect()
}

/// Well this holds the table name, but a struct is needed for the TryFromRow trait
#[derive(cdrs_helpers_derive::TryFromRow)]
pub struct TableName {
    pub table_name: String,
}

/// Queries all the tablenames known in the database
fn keyspace_tables(session: &DbTestSession) -> Vec<TableName> {
    rows(Ok(query(
        session,
        &format!(
            "select table_name from system_schema.tables where keyspace_name = '{}'",
            TEST_CDRS_DB_KEYSPACE
        ),
    )))
}

/// Creates a dummy table
pub fn create_dummy_table(session: &DbTestSession) {
    query(session, &format!("create table if not exists {} (a int, b int, c int, d int, e int, primary key((b, c), d, a))", TEST_TABLE));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_connection() {
        let s = create_test_db_session();

        s.query("drop keyspace if exists sada".to_string()).unwrap();
    }
}
