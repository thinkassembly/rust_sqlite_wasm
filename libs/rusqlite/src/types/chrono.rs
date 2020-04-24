//! Convert most of the [Time Strings](http://sqlite.org/lang_datefunc.html) to chrono types.

use std::borrow::Cow;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use crate::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use crate::Result;

/// ISO 8601 calendar date without timezone => "YYYY-MM-DD"
impl ToSql for NaiveDate {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let date_str = self.format("%Y-%m-%d").to_string();
        Ok(ToSqlOutput::from(date_str))
    }
}

/// "YYYY-MM-DD" => ISO 8601 calendar date without timezone.
impl FromSql for NaiveDate {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                Ok(dt) => Ok(dt),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            })
    }
}

/// ISO 8601 time without timezone => "HH:MM:SS.SSS"
impl ToSql for NaiveTime {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let date_str = self.format("%H:%M:%S%.f").to_string();
        Ok(ToSqlOutput::from(date_str))
    }
}

/// "HH:MM"/"HH:MM:SS"/"HH:MM:SS.SSS" => ISO 8601 time without timezone.
impl FromSql for NaiveTime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            let fmt = match s.len() {
                5 => "%H:%M",
                8 => "%H:%M:%S",
                _ => "%H:%M:%S%.f",
            };
            match NaiveTime::parse_from_str(s, fmt) {
                Ok(dt) => Ok(dt),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            }
        })
    }
}

/// ISO 8601 combined date and time without timezone =>
/// "YYYY-MM-DDTHH:MM:SS.SSS"
impl ToSql for NaiveDateTime {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let date_str = self.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
        Ok(ToSqlOutput::from(date_str))
    }
}

/// "YYYY-MM-DD HH:MM:SS"/"YYYY-MM-DD HH:MM:SS.SSS" => ISO 8601 combined date
/// and time without timezone. ("YYYY-MM-DDTHH:MM:SS"/"YYYY-MM-DDTHH:MM:SS.SSS"
/// also supported)
impl FromSql for NaiveDateTime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            let fmt = if s.len() >= 11 && s.as_bytes()[10] == b'T' {
                "%Y-%m-%dT%H:%M:%S%.f"
            } else {
                "%Y-%m-%d %H:%M:%S%.f"
            };

            match NaiveDateTime::parse_from_str(s, fmt) {
                Ok(dt) => Ok(dt),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            }
        })
    }
}

/// Date and time with time zone => UTC RFC3339 timestamp
/// ("YYYY-MM-DDTHH:MM:SS.SSS+00:00").
impl<Tz: TimeZone> ToSql for DateTime<Tz> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.with_timezone(&Utc).to_rfc3339()))
    }
}

/// RFC3339 ("YYYY-MM-DDTHH:MM:SS.SSS[+-]HH:MM") into `DateTime<Utc>`.
impl FromSql for DateTime<Utc> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        {
            // Try to parse value as rfc3339 first.
            let s = value.as_str()?;

            // If timestamp looks space-separated, make a copy and replace it with 'T'.
            let s = if s.len() >= 11 && s.as_bytes()[10] == b' ' {
                let mut s = s.to_string();
                unsafe {
                    let sbytes = s.as_mut_vec();
                    sbytes[10] = b'T';
                }
                Cow::Owned(s)
            } else {
                Cow::Borrowed(s)
            };

            if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
                return Ok(dt.with_timezone(&Utc));
            }
        }

        // Couldn't parse as rfc3339 - fall back to NaiveDateTime.
        NaiveDateTime::column_result(value).map(|dt| Utc.from_utc_datetime(&dt))
    }
}

/// RFC3339 ("YYYY-MM-DDTHH:MM:SS.SSS[+-]HH:MM") into `DateTime<Local>`.
impl FromSql for DateTime<Local> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let utc_dt = DateTime::<Utc>::column_result(value)?;
        Ok(utc_dt.with_timezone(&Local))
    }
}
