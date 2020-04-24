//! `ToSql` and `FromSql` implementation for [`url::Url`].
use crate::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use crate::Result;
use url::Url;

/// Serialize `Url` to text.
impl ToSql for Url {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

/// Deserialize text to `Url`.
impl FromSql for Url {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => {
                let s = std::str::from_utf8(s).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                Url::parse(s).map_err(|e| FromSqlError::Other(Box::new(e)))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
