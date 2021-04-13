// @generated, do not edit
#[derive(cdrs_db_mirror :: DBMirror, cdrs_tokio_helpers_derive :: TryFromRow)]
pub struct test2 {
    #[partition_key]
    pub a: i32,
    #[clustering_key]
    pub b: i32,
}
