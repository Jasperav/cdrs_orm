use cdrs_con::{create_test_db_session, query, recreate_keyspace, TEST_CDRS_DB_KEYSPACE};

fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let session = create_test_db_session();

    recreate_keyspace(&session, TEST_CDRS_DB_KEYSPACE);
    query(
        &session,
        "create table uuid_struct
(
    id   uuid,
    name text,
    primary key ( id )
);",
    );
    query(
        &session,
        "create table some_struct
(
    id   int,
    another_id   int,
    cluster_key   int,
    another_cluster_key   int,
    name text,
    primary key ((id, another_id), cluster_key, another_cluster_key)
);",
    );
}
