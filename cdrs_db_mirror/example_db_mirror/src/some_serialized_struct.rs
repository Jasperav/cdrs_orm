#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    cdrs_db_mirror::DBJson,
    PartialEq,
    rand_derive2::RandGen,
)]
pub struct SomeSerializedStruct {
    pub id: i32,
}
