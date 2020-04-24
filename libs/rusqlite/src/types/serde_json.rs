//! `ToSql` and `FromSql` implementation for JSON `Value`.
// made pub for testing
//use serde_json::Value;
pub use serde_json::Value;
use crate::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use crate::Result;

/// Serialize JSON `Value` to text.
impl ToSql for Value {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(serde_json::to_string(self).unwrap()))
    }
}

/// Deserialize text/blob to JSON `Value`.
impl FromSql for Value {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => serde_json::from_slice(s),
            ValueRef::Blob(b) => serde_json::from_slice(b),
            _ => return Err(FromSqlError::InvalidType),
        }
        .map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}
