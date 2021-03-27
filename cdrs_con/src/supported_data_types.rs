/// Supported data types
/// Feel free to add more
#[derive(Debug, PartialEq)]
pub enum CassandraDataType {
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    Text,
    Ascii,
    Varchar,
    Boolean,
    Time,
    Timestamp,
    Float,
    Double,
    Uuid,
    Counter,
}

impl CassandraDataType {
    pub fn new(from: &str) -> Self {
        match from {
            "tinyint" => CassandraDataType::TinyInt,
            "smallint" => CassandraDataType::SmallInt,
            "int" => CassandraDataType::Int,
            "bigint" => CassandraDataType::BigInt,
            "text" => CassandraDataType::Text,
            "ascii" => CassandraDataType::Ascii,
            "varchar" => CassandraDataType::Varchar,
            "boolean" => CassandraDataType::Boolean,
            "time" => CassandraDataType::Time,
            "timestamp" => CassandraDataType::Timestamp,
            "float" => CassandraDataType::Float,
            "double" => CassandraDataType::Double,
            "uuid" => CassandraDataType::Uuid,
            "counter" => CassandraDataType::Counter,
            _ => panic!("Unexpected type: {}, please add this type", from),
        }
    }

    pub fn to_ty(&self) -> &'static str {
        match self {
            CassandraDataType::TinyInt => "i8",
            CassandraDataType::SmallInt => "i16",
            CassandraDataType::Int => "i32",
            CassandraDataType::BigInt
            | CassandraDataType::Time
            | CassandraDataType::Timestamp
            | CassandraDataType::Counter => "i64",
            CassandraDataType::Text | CassandraDataType::Ascii | CassandraDataType::Varchar => {
                "String"
            }
            CassandraDataType::Boolean => "bool",
            CassandraDataType::Float => "f32",
            CassandraDataType::Double => "f64",
            CassandraDataType::Uuid => "uuid::Uuid",
        }
    }
}
