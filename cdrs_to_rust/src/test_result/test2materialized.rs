// @generated, do not edit
#[derive(cdrs_db_mirror :: DBMirror, cdrs_tokio_helpers_derive :: TryFromRow)]
pub struct test2materialized {
    #[partition_key]
    pub b: i32,
    #[clustering_key]
    pub a: i32,
}
