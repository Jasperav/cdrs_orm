use crate::DbTestSession;
use cdrs::frame::{AsByte, Frame};
use cdrs::query::{QueryExecutor, QueryValues};
use cdrs::types::value::{Value, ValueType};

pub fn query_values_tokio_to_sync(qv: cdrs_tokio::query::QueryValues) -> QueryValues {
    match qv {
        cdrs_tokio::query::QueryValues::SimpleValues(sv) => {
            QueryValues::SimpleValues(sv.into_iter().map(value_tokio_to_sync).collect())
        }
        cdrs_tokio::query::QueryValues::NamedValues(nv) => QueryValues::NamedValues(
            nv.into_iter()
                .map(|(s, v)| (s, value_tokio_to_sync(v)))
                .collect(),
        ),
    }
}

pub fn value_tokio_to_sync(value: cdrs_tokio::types::value::Value) -> Value {
    Value {
        body: value.body,
        value_type: match value.value_type {
            cdrs_tokio::types::value::ValueType::Normal(n) => ValueType::Normal(n),
            cdrs_tokio::types::value::ValueType::Null => ValueType::Null,
            cdrs_tokio::types::value::ValueType::NotSet => ValueType::NotSet,
        },
    }
}

/// Query the database with values from cdrs-tokio
pub fn query_with_values_tokio(
    session: &DbTestSession,
    query: &str,
    values: cdrs_tokio::query::QueryValues,
) -> Frame {
    session
        .query_with_values(query, query_values_tokio_to_sync(values))
        .unwrap()
}

pub fn frame_sync_to_tokio(frame: Frame) -> cdrs_tokio::frame::Frame {
    cdrs_tokio::frame::Frame {
        version: cdrs_tokio::frame::Version::from(vec![frame.version.as_byte()]),
        flags: frame
            .flags
            .into_iter()
            .map(|flag| cdrs_tokio::frame::Flag::from(flag.as_byte()))
            .collect(),
        opcode: cdrs_tokio::frame::Opcode::from(frame.opcode.as_byte()),
        stream: frame.stream as i16,
        body: frame.body,
        tracing_id: frame.tracing_id,
        warnings: frame.warnings,
    }
}

/// Transforms a result from a query into rows of the specified type
pub fn rows_tokio<T: cdrs_tokio::frame::TryFromRow>(result: cdrs::Result<Frame>) -> Vec<T> {
    let frame = frame_sync_to_tokio(result.expect("Failed to execute query"));

    frame
        .get_body()
        .expect("Failed to get body")
        .into_rows()
        .expect("Failed to turn into rows")
        .into_iter()
        .map(|row| T::try_from_row(row).expect("Failed to turn query results into struct"))
        .collect()
}
