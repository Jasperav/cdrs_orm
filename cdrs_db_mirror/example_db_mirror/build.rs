use cdrs_con::{create_test_db_session, query, recreate_keyspace, TEST_CDRS_DB_KEYSPACE};

fn main() {
    dotenv::dotenv().unwrap();

    let session = create_test_db_session();

    recreate_keyspace(&session, TEST_CDRS_DB_KEYSPACE);
    query(
        &session,
        "create table another_struct
(
    id   int,
    name text,
    primary key ( id )
);",
    );
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
    query(
        &session,
        "create table foo_struct
(
    id   int,
    cluster_key   int,
    name text,
    nickname text,
    primary key ((id), cluster_key)
);",
    );
    query(
        &session,
        "create table struct_json_mapping
(
    a   text,
    b   text,
    c   text,
    primary key ((a))
);",
    );
}
