//! Traits dealing with SQLite data types.
//!
//! SQLite uses a [dynamic type system](https://www.sqlite.org/datatype3.html). Implementations of
//! the `ToSql` and `FromSql` traits are provided for the basic types that
//! SQLite provides methods for:
//!
//! * Integers (`i32` and `i64`; SQLite uses `i64` internally, so getting an
//! `i32` will truncate   if the value is too large or too small).
//! * Reals (`f64`)
//! * Strings (`String` and `&str`)
//! * Blobs (`Vec<u8>` and `&[u8]`)
//!
//! Additionally, because it is such a common data type, implementations are
//! provided for `time::Timespec` that use the RFC 3339 date/time format,
//! `"%Y-%m-%dT%H:%M:%S.%fZ"`, to store time values as strings.  These values
//! can be parsed by SQLite's builtin
//! [datetime](https://www.sqlite.org/lang_datefunc.html) functions.  If you
//! want different storage for timespecs, you can use a newtype. For example, to
//! store timespecs as `f64`s:
//!
//! ```rust
//! use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
//! use rusqlite::Result;
//!
//! pub struct TimespecSql(pub time::Timespec);
//!
//! impl FromSql for TimespecSql {
//!     fn column_result(value: ValueRef) -> FromSqlResult<Self> {
//!         f64::column_result(value).map(|as_f64| {
//!             TimespecSql(time::Timespec {
//!                 sec: as_f64.trunc() as i64,
//!                 nsec: (as_f64.fract() * 1.0e9) as i32,
//!             })
//!         })
//!     }
//! }
//!
//! impl ToSql for TimespecSql {
//!     fn to_sql(&self) -> Result<ToSqlOutput> {
//!         let TimespecSql(ts) = *self;
//!         let as_f64 = ts.sec as f64 + (ts.nsec as f64) / 1.0e9;
//!         Ok(as_f64.into())
//!     }
//! }
//! ```
//!
//! `ToSql` and `FromSql` are also implemented for `Option<T>` where `T`
//! implements `ToSql` or `FromSql` for the cases where you want to know if a
//! value was NULL (which gets translated to `None`).

pub use self::from_sql::{FromSql, FromSqlError, FromSqlResult};
pub use self::to_sql::{ToSql, ToSqlOutput};
pub use self::value::Value;
pub use self::value_ref::ValueRef;

use std::fmt;

#[cfg(feature = "chrono")]
mod chrono;
mod from_sql;
#[cfg(feature = "serde_json")]
pub mod serde_json;
mod time;
mod to_sql;
#[cfg(feature = "url")]
mod url;
mod value;
mod value_ref;

/// Empty struct that can be used to fill in a query parameter as `NULL`.
///
/// ## Example
///
/// ```rust,no_run
/// # use rusqlite::{Connection, Result};
/// # use rusqlite::types::{Null};
///
/// fn insert_null(conn: &Connection) -> Result<usize> {
///     conn.execute("INSERT INTO people (name) VALUES (?)", &[Null])
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Null;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Type::Null => write!(f, "Null"),
            Type::Integer => write!(f, "Integer"),
            Type::Real => write!(f, "Real"),
            Type::Text => write!(f, "Text"),
            Type::Blob => write!(f, "Blob"),
        }
    }
}
