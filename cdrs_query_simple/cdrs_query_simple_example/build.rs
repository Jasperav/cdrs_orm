use cdrs_con::{
    create_dummy_table, create_test_db_session, query, recreate_keyspace, TEST_CDRS_DB_KEYSPACE,
};

fn main() {
    let session = create_test_db_session();

    recreate_keyspace(&session, TEST_CDRS_DB_KEYSPACE);
    create_dummy_table(&session);

    query(
        &session,
        "create table uuid_table (pk uuid, primary key((pk)))",
    );
}
