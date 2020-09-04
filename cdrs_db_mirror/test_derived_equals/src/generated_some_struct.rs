#[derive(cdrs_db_mirror::DBMirror)]
#[allow(dead_code)]
struct SomeStruct {
    #[partition_key]
    id: i32,
    #[partition_key]
    another_id: i32,
    #[clustering_key]
    cluster_key: i32,
    #[clustering_key]
    another_cluster_key: i32,
    // Just some column that is not part of the primary key
    name: String,
}
