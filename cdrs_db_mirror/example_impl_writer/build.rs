use cdrs_con::{create_test_db_session, query, recreate_keyspace, TEST_CDRS_DB_KEYSPACE};
use cdrs_to_rust::Transformer;

fn main() {
    let session = create_test_db_session();

    recreate_keyspace(&session, TEST_CDRS_DB_KEYSPACE);
    query(
        &session,
        "create table SomeStruct
(
    id   int,
    another_id   int,
    cluster_key   int,
    another_cluster_key   int,
    name uuid,
    primary key ((id, another_id), cluster_key, another_cluster_key)
);",
    );

    let current_dir = std::env::current_dir().unwrap();

    struct Trans;

    impl Transformer for Trans {
        fn table_name_to_struct_name(&self, _table_name: &str) -> String {
            "SomeStruct".to_string()
        }

        fn derive(&self, _struct_name: &str) -> Vec<&'static str> {
            vec!["cdrs_db_mirror::DBMirror", "custom_derive::Entity"]
        }

        fn metadata(&self, _struct_name: &str) -> Vec<String> {
            vec!["#[allow(dead_code)]".to_string()]
        }
    }

    cdrs_to_rust::generate(
        &current_dir.join("src").join("generated"),
        Trans,
        quote::quote! {},
    )
}
