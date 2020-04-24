use crate::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use crate::Result;

const CURRENT_TIMESTAMP_FMT: &str = "%Y-%m-%d %H:%M:%S";
const SQLITE_DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S.%fZ";
const SQLITE_DATETIME_FMT_LEGACY: &str = "%Y-%m-%d %H:%M:%S:%f %Z";

impl ToSql for time::Timespec {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let time_string = time::at_utc(*self)
            .strftime(SQLITE_DATETIME_FMT)
            .unwrap()
            .to_string();
        Ok(ToSqlOutput::from(time_string))
    }
}

impl FromSql for time::Timespec {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| {
                match s.len() {
                    19 => time::strptime(s, CURRENT_TIMESTAMP_FMT),
                    _ => time::strptime(s, SQLITE_DATETIME_FMT).or_else(|err| {
                        time::strptime(s, SQLITE_DATETIME_FMT_LEGACY).or_else(|_| Err(err))
                    }),
                }
                .or_else(|err| Err(FromSqlError::Other(Box::new(err))))
            })
            .map(|tm| tm.to_timespec())
    }
}
