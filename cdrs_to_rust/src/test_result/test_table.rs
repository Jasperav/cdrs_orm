// Generated file
#[derive(cdrs_db_mirror :: DBMirror, cdrs_tokio_helpers_derive :: TryFromRow)]
pub struct test_table {
    #[partition_key]
    pub b: i32,
    #[partition_key]
    pub c: i32,
    #[clustering_key]
    pub d: i32,
    #[clustering_key]
    pub a: i32,
    pub e: i32,
}
