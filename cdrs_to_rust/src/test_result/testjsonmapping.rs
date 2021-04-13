"#![allow(unknown_lints)]\n#![allow(clippy::all)]\n#![rustfmt::skip]\n#![allow(unused_attributes)]\n// @generated, do not edit"
use crate::MyJsonType;
#[derive(cdrs_db_mirror :: DBMirror, cdrs_tokio_helpers_derive :: TryFromRow)]
pub struct testjsonmapping {
    #[partition_key]
    pub a: i32,
    #[clustering_key]
    #[json_mapped]
    pub json: MyJsonType,
    #[json_mapped]
    pub json_nullable: std::option::Option<MyJsonType>,
}
