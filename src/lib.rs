extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate rusqlite;

use rusqlite::{params, Connection};

extern crate libc_sys;


extern crate js_sys;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
struct Person {
    id: i32,
    name: String,
    time_created: f64,
    data: Option<Vec<u8>>,
}

#[wasm_bindgen]
pub fn start() {
    wasm_println::hook();
    println!("Sqlite Version {:?}", rusqlite::version());
    println!();

    match Connection::open_in_memory() {
        Ok(conn) => {
            println!("Creating TABLE: person
            CREATE TABLE person (
                          id              INTEGER PRIMARY KEY,
                          name            TEXT NOT NULL,
                          time_created    INTEGER NOT NULL,
                          data            BLOB
                          )");
            println!();
            conn.execute(
                "CREATE TABLE person (
                          id              INTEGER PRIMARY KEY,
                          name            TEXT NOT NULL,
                          time_created    INTEGER NOT NULL,
                           data            BLOB
                           )",
                params![],
            ).expect("CREATE TABLE statement did not execute.");


            let me = Person {
                id: 0,
                name: "Person ".to_string(),
                time_created: js_sys::Date::new_0().value_of(),
                data: None,
            };
            println!("Inserting into TABLE: \
            INSERT INTO person(name,time_created,data) VALUES (?1, ?2, ?3,?4)");

            for i in 0..10 {
                conn.execute(
                    "INSERT INTO person (id,name, time_created, data)
                                   VALUES (?1, ?2, ?3,?4)",
                    params![
                        i,
                        me.name.clone() + &i.to_string().clone(),
                       js_sys::Date::new_0().value_of(),
                        me.data
                    ],
                ).expect("Error inserting record.");
            }
            println!("Querying person table : \
            SELECT id,name,time_created,data FROM person");
            let mut stmt = conn
                .prepare("SELECT id, name, time_created, data FROM person")
                .expect("Error preparing statement.");

            let person_iter = stmt
                .query_map(params![], |row| {
                    Ok(Person {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        time_created: row.get(2)?,
                        data: row.get(3)?,
                    })
                })
                .expect("Select query failed");

            for p in person_iter {
                let person = p.expect("Could not unwrap person").clone();
                println!("{:?}", person);
            }
            println!("Done");
        }
        Err(e) => println!("{:?}", e),
    }
}

use rusqlite::ffi;

extern crate wasm_bindgen_test;
extern crate libsqlite3_sys;
extern crate fallible_streaming_iterator;
extern crate unicase;
extern crate chrono;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_error_when_singlethread_mode() {
    // put SQLite into single-threaded mode
    unsafe {
        if ffi::sqlite3_config(ffi::SQLITE_CONFIG_SINGLETHREAD) != ffi::SQLITE_OK {
            return;
        }
        if ffi::sqlite3_initialize() != ffi::SQLITE_OK {
            return;
        }
    }

    let _ = Connection::open_in_memory().unwrap();
}


#[wasm_bindgen_test]
fn test_dummy_module() {
    use rusqlite::types::ToSql;
    use rusqlite::vtab::{
        eponymous_only_module, sqlite3_vtab, sqlite3_vtab_cursor, Context, IndexInfo, VTab,
        VTabConnection, VTabCursor, Values,
    };
    use rusqlite::{version_number, Connection, Result};
    use std::os::raw::c_int;

    let module = eponymous_only_module::<DummyTab>(1);

    #[repr(C)]
    struct DummyTab {
        /// Base class. Must be first
        base: sqlite3_vtab,
    }

    impl VTab for DummyTab {
        type Aux = ();
        type Cursor = DummyTabCursor;

        fn connect(
            _: &mut VTabConnection,
            _aux: Option<&()>,
            _args: &[&[u8]],
        ) -> Result<(String, DummyTab)> {
            let vtab = DummyTab {
                base: sqlite3_vtab::default(),
            };
            Ok(("CREATE TABLE x(value)".to_owned(), vtab))
        }

        fn best_index(&self, info: &mut IndexInfo) -> Result<()> {
            info.set_estimated_cost(1.);
            Ok(())
        }

        fn open(&self) -> Result<DummyTabCursor> {
            Ok(DummyTabCursor::default())
        }
    }

    #[derive(Default)]
    #[repr(C)]
    struct DummyTabCursor {
        /// Base class. Must be first
        base: sqlite3_vtab_cursor,
        /// The rowid
        row_id: i64,
    }

    impl VTabCursor for DummyTabCursor {
        fn filter(
            &mut self,
            _idx_num: c_int,
            _idx_str: Option<&str>,
            _args: &Values<'_>,
        ) -> Result<()> {
            self.row_id = 1;
            Ok(())
        }

        fn next(&mut self) -> Result<()> {
            self.row_id += 1;
            Ok(())
        }

        fn eof(&self) -> bool {
            self.row_id > 1
        }

        fn column(&self, ctx: &mut Context, _: c_int) -> Result<()> {
            ctx.set_result(&self.row_id)
        }

        fn rowid(&self) -> Result<i64> {
            Ok(self.row_id)
        }
    }

    let db = Connection::open_in_memory().unwrap();

    db.create_module::<DummyTab>("dummy", &module, None)
        .unwrap();

    let version = version_number();
    if version < 3_008_012 {
        return;
    }

    let mut s = db.prepare("SELECT * FROM dummy()").unwrap();

    let dummy = s
        .query_row(&[] as &[&dyn ToSql], |row| row.get::<_, i32>(0))
        .unwrap();
    assert_eq!(1, dummy);
}


#[cfg(test)]
mod test {
    use rusqlite::*;
    use rusqlite::ffi;

    extern crate fallible_iterator;

    use self::fallible_iterator::FallibleIterator;
    use std::error::Error as StdError;
    use std::fmt;
    use wasm_bindgen_test::*;

    // this function is never called, but is still type checked; in
    // particular, calls with specific instantiations will require
    // that those types are `Send`.
    #[allow(dead_code, unconditional_recursion)]
    fn ensure_send<T: Send>() {
        ensure_send::<Connection>();
        ensure_send::<InterruptHandle>();
    }

    #[allow(dead_code, unconditional_recursion)]
    fn ensure_sync<T: Sync>() {
        ensure_sync::<InterruptHandle>();
    }

    pub fn checked_memory_handle() -> Connection {
        Connection::open_in_memory().unwrap()
    }
/*
    #[wasm_bindgen_test]
    fn test_concurrent_transactions_busy_commit() {
        use std::time::Duration;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("transactions.db3");

        Connection::open(&path)
            .expect("create temp db")
            .execute_batch(
                "
            BEGIN; CREATE TABLE foo(x INTEGER);
            INSERT INTO foo VALUES(42); END;",
            )
            .expect("create temp db");

        let mut db1 =
            Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
        let mut db2 = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

        db1.busy_timeout(Duration::from_millis(0)).unwrap();
        db2.busy_timeout(Duration::from_millis(0)).unwrap();

        {
            let tx1 = db1.transaction().unwrap();
            let tx2 = db2.transaction().unwrap();

            // SELECT first makes sqlite lock with a shared lock
            tx1.query_row("SELECT x FROM foo LIMIT 1", NO_PARAMS, |_| Ok(()))
                .unwrap();
            tx2.query_row("SELECT x FROM foo LIMIT 1", NO_PARAMS, |_| Ok(()))
                .unwrap();

            tx1.execute("INSERT INTO foo VALUES(?1)", &[1]).unwrap();
            let _ = tx2.execute("INSERT INTO foo VALUES(?1)", &[2]);

            let _ = tx1.commit();
            let _ = tx2.commit();
        }

        let _ = db1
            .transaction()
            .expect("commit should have closed transaction");
        let _ = db2
            .transaction()
            .expect("commit should have closed transaction");
    }
*/
    /*
    #[wasm_bindgen_test]
    fn test_persistence() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.db3");

        {
            let db = Connection::open(&path).unwrap();
            let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER);
                   INSERT INTO foo VALUES(42);
                   END;";
            db.execute_batch(sql).unwrap();
        }

        let path_string = path.to_str().unwrap();
        let db = Connection::open(&path_string).unwrap();
        let the_answer: Result<i64> = db.query_row("SELECT x FROM foo", NO_PARAMS, |r| r.get(0));

        assert_eq!(42i64, the_answer.unwrap());
    }
*/
    #[wasm_bindgen_test]
    fn test_open() {
        assert!(Connection::open_in_memory().is_ok());

        let db = checked_memory_handle();
        assert!(db.close().is_ok());
    }

    #[wasm_bindgen_test]
    fn test_open_failure() {
        let filename = "no_such_file.db";
        let result = Connection::open_with_flags(filename, OpenFlags::SQLITE_OPEN_READ_ONLY);
        assert!(!result.is_ok());
        let err = result.err().unwrap();
        if let Error::SqliteFailure(e, Some(msg)) = err {
            assert_eq!(ErrorCode::CannotOpen, e.code);
            assert_eq!(ffi::SQLITE_CANTOPEN, e.extended_code);
            assert!(
                msg.contains(filename),
                "error message '{}' does not contain '{}'",
                msg,
                filename
            );
        } else {
            panic!("SqliteFailure expected");
        }
    }

    #[wasm_bindgen_test]
    fn test_close_retry() {
        let db = checked_memory_handle();

        // force the DB to be busy by preparing a statement; this must be done at the
        // FFI level to allow us to call .close() without dropping the prepared
        // statement first.
        let raw_stmt = {
            use rusqlite::str_to_cstring;
            use std::mem::MaybeUninit;
            use std::os::raw::c_int;
            use std::ptr;

            let raw_db = db.db.borrow_mut().db;
            let sql = "SELECT 1";
            let mut raw_stmt = MaybeUninit::uninit();
            let cstring = str_to_cstring(sql).unwrap();
            let rc = unsafe {
                ffi::sqlite3_prepare_v2(
                    raw_db,
                    cstring.as_ptr(),
                    (sql.len() + 1) as c_int,
                    raw_stmt.as_mut_ptr(),
                    ptr::null_mut(),
                )
            };
            assert_eq!(rc, ffi::SQLITE_OK);
            let raw_stmt: *mut ffi::sqlite3_stmt = unsafe { raw_stmt.assume_init() };
            raw_stmt
        };

        // now that we have an open statement, trying (and retrying) to close should
        // fail.
        let (db, _) = db.close().unwrap_err();
        let (db, _) = db.close().unwrap_err();
        let (db, _) = db.close().unwrap_err();

        // finalize the open statement so a final close will succeed
        assert_eq!(ffi::SQLITE_OK, unsafe { ffi::sqlite3_finalize(raw_stmt) });

        db.close().unwrap();
    }

    #[wasm_bindgen_test]
    fn test_open_with_flags() {
        for bad_flags in &[
            OpenFlags::empty(),
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_READ_WRITE,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_CREATE,
        ] {
            assert!(Connection::open_in_memory_with_flags(*bad_flags).is_err());
        }
    }

    #[wasm_bindgen_test]
    fn test_execute_batch() {
        let db = checked_memory_handle();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER);
                   INSERT INTO foo VALUES(1);
                   INSERT INTO foo VALUES(2);
                   INSERT INTO foo VALUES(3);
                   INSERT INTO foo VALUES(4);
                   END;";
        db.execute_batch(sql).unwrap();

        db.execute_batch("UPDATE foo SET x = 3 WHERE x < 3")
            .unwrap();

        assert!(db.execute_batch("INVALID SQL").is_err());
    }

    #[wasm_bindgen_test]
    fn test_execute() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x INTEGER)").unwrap();

        assert_eq!(
            1,
            db.execute("INSERT INTO foo(x) VALUES (?)", &[1i32])
                .unwrap()
        );
        assert_eq!(
            1,
            db.execute("INSERT INTO foo(x) VALUES (?)", &[2i32])
                .unwrap()
        );

        assert_eq!(
            3i32,
            db.query_row::<i32, _, _>("SELECT SUM(x) FROM foo", NO_PARAMS, |r| r.get(0))
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "extra_check")]
    fn test_execute_select() {
        let db = checked_memory_handle();
        let err = db.execute("SELECT 1 WHERE 1 < ?", &[1i32]).unwrap_err();
        if err != Error::ExecuteReturnedResults {
            panic!("Unexpected error: {}", err);
        }
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "extra_check")]
    fn test_execute_multiple() {
        let db = checked_memory_handle();
        let err = db
            .execute(
                "CREATE TABLE foo(x INTEGER); CREATE TABLE foo(x INTEGER)",
                NO_PARAMS,
            )
            .unwrap_err();
        match err {
            Error::MultipleStatement => (),
            _ => panic!("Unexpected error: {}", err),
        }
    }

    #[wasm_bindgen_test]
    fn test_prepare_column_names() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

        let stmt = db.prepare("SELECT * FROM foo").unwrap();
        assert_eq!(stmt.column_count(), 1);
        assert_eq!(stmt.column_names(), vec!["x"]);

        let stmt = db.prepare("SELECT x AS a, x AS b FROM foo").unwrap();
        assert_eq!(stmt.column_count(), 2);
        assert_eq!(stmt.column_names(), vec!["a", "b"]);
    }

    #[wasm_bindgen_test]
    fn test_prepare_execute() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

        let mut insert_stmt = db.prepare("INSERT INTO foo(x) VALUES(?)").unwrap();
        assert_eq!(insert_stmt.execute(&[1i32]).unwrap(), 1);
        assert_eq!(insert_stmt.execute(&[2i32]).unwrap(), 1);
        assert_eq!(insert_stmt.execute(&[3i32]).unwrap(), 1);

        assert_eq!(insert_stmt.execute(&["hello".to_string()]).unwrap(), 1);
        assert_eq!(insert_stmt.execute(&["goodbye".to_string()]).unwrap(), 1);
        assert_eq!(insert_stmt.execute(&[types::Null]).unwrap(), 1);

        let mut update_stmt = db.prepare("UPDATE foo SET x=? WHERE x<?").unwrap();
        assert_eq!(update_stmt.execute(&[3i32, 3i32]).unwrap(), 2);
        assert_eq!(update_stmt.execute(&[3i32, 3i32]).unwrap(), 0);
        assert_eq!(update_stmt.execute(&[8i32, 8i32]).unwrap(), 3);
    }

    #[wasm_bindgen_test]
    fn test_prepare_query() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

        let mut insert_stmt = db.prepare("INSERT INTO foo(x) VALUES(?)").unwrap();
        assert_eq!(insert_stmt.execute(&[1i32]).unwrap(), 1);
        assert_eq!(insert_stmt.execute(&[2i32]).unwrap(), 1);
        assert_eq!(insert_stmt.execute(&[3i32]).unwrap(), 1);

        let mut query = db
            .prepare("SELECT x FROM foo WHERE x < ? ORDER BY x DESC")
            .unwrap();
        {
            let mut rows = query.query(&[4i32]).unwrap();
            let mut v = Vec::<i32>::new();

            while let Some(row) = rows.next().unwrap() {
                v.push(row.get(0).unwrap());
            }

            assert_eq!(v, [3i32, 2, 1]);
        }

        {
            let mut rows = query.query(&[3i32]).unwrap();
            let mut v = Vec::<i32>::new();

            while let Some(row) = rows.next().unwrap() {
                v.push(row.get(0).unwrap());
            }

            assert_eq!(v, [2i32, 1]);
        }
    }

    #[wasm_bindgen_test]
    fn test_query_map() {
        let db = checked_memory_handle();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER, y TEXT);
                   INSERT INTO foo VALUES(4, \"hello\");
                   INSERT INTO foo VALUES(3, \", \");
                   INSERT INTO foo VALUES(2, \"world\");
                   INSERT INTO foo VALUES(1, \"!\");
                   END;";
        db.execute_batch(sql).unwrap();

        let mut query = db.prepare("SELECT x, y FROM foo ORDER BY x DESC").unwrap();
        let results: Result<Vec<String>> = query
            .query(NO_PARAMS)
            .unwrap()
            .map(|row| row.get(1))
            .collect();

        assert_eq!(results.unwrap().concat(), "hello, world!");
    }

    #[wasm_bindgen_test]
    fn test_query_row() {
        let db = checked_memory_handle();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER);
                   INSERT INTO foo VALUES(1);
                   INSERT INTO foo VALUES(2);
                   INSERT INTO foo VALUES(3);
                   INSERT INTO foo VALUES(4);
                   END;";
        db.execute_batch(sql).unwrap();

        assert_eq!(
            10i64,
            db.query_row::<i64, _, _>("SELECT SUM(x) FROM foo", NO_PARAMS, |r| r.get(0))
                .unwrap()
        );

        let result: Result<i64> =
            db.query_row("SELECT x FROM foo WHERE x > 5", NO_PARAMS, |r| r.get(0));
        match result.unwrap_err() {
            Error::QueryReturnedNoRows => (),
            err => panic!("Unexpected error {}", err),
        }

        let bad_query_result = db.query_row("NOT A PROPER QUERY; test123", NO_PARAMS, |_| Ok(()));

        assert!(bad_query_result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_optional() {
        let db = checked_memory_handle();

        let result: Result<i64> = db.query_row("SELECT 1 WHERE 0 <> 0", NO_PARAMS, |r| r.get(0));
        let result = result.optional();
        match result.unwrap() {
            None => (),
            _ => panic!("Unexpected result"),
        }

        let result: Result<i64> = db.query_row("SELECT 1 WHERE 0 == 0", NO_PARAMS, |r| r.get(0));
        let result = result.optional();
        match result.unwrap() {
            Some(1) => (),
            _ => panic!("Unexpected result"),
        }

        let bad_query_result: Result<i64> =
            db.query_row("NOT A PROPER QUERY", NO_PARAMS, |r| r.get(0));
        let bad_query_result = bad_query_result.optional();
        assert!(bad_query_result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_pragma_query_row() {
        let db = checked_memory_handle();

        assert_eq!(
            "memory",
            db.query_row::<String, _, _>("PRAGMA journal_mode", NO_PARAMS, |r| r.get(0))
                .unwrap()
        );
        assert_eq!(
            "off",
            db.query_row::<String, _, _>("PRAGMA journal_mode=off", NO_PARAMS, |r| r.get(0))
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_prepare_failures() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

        let err = db.prepare("SELECT * FROM does_not_exist").unwrap_err();
        assert!(format!("{}", err).contains("does_not_exist"));
    }

    #[wasm_bindgen_test]
    fn test_last_insert_rowid() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x INTEGER PRIMARY KEY)")
            .unwrap();
        db.execute_batch("INSERT INTO foo DEFAULT VALUES").unwrap();

        assert_eq!(db.last_insert_rowid(), 1);

        let mut stmt = db.prepare("INSERT INTO foo DEFAULT VALUES").unwrap();
        for _ in 0i32..9 {
            stmt.execute(NO_PARAMS).unwrap();
        }
        assert_eq!(db.last_insert_rowid(), 10);
    }

    #[wasm_bindgen_test]
    fn test_is_autocommit() {
        let db = checked_memory_handle();
        assert!(
            db.is_autocommit(),
            "autocommit expected to be active by default"
        );
    }

    #[wasm_bindgen_test]
    fn test_is_busy() {
        let db = checked_memory_handle();
        assert!(!db.is_busy());
        let mut stmt = db.prepare("PRAGMA schema_version").unwrap();
        assert!(!db.is_busy());
        {
            let mut rows = stmt.query(NO_PARAMS).unwrap();
            assert!(!db.is_busy());
            let row = rows.next().unwrap();
            assert!(db.is_busy());
            assert!(row.is_some());
        }
        assert!(!db.is_busy());
    }

    #[wasm_bindgen_test]
    fn test_statement_debugging() {
        let db = checked_memory_handle();
        let query = "SELECT 12345";
        let stmt = db.prepare(query).unwrap();

        assert!(format!("{:?}", stmt).contains(query));
    }

    #[wasm_bindgen_test]
    fn test_notnull_constraint_error() {
        use std::os::raw::c_int;
        // extended error codes for constraints were added in SQLite 3.7.16; if we're
        // running on our bundled version, we know the extended error code exists.
        fn check_extended_code(extended_code: c_int) {
            assert_eq!(extended_code, ffi::SQLITE_CONSTRAINT_NOTNULL);
        }

        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(x NOT NULL)").unwrap();

        let result = db.execute("INSERT INTO foo (x) VALUES (NULL)", NO_PARAMS);
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::SqliteFailure(err, _) => {
                assert_eq!(err.code, ErrorCode::ConstraintViolation);
                check_extended_code(err.extended_code);
            }
            err => panic!("Unexpected error {}", err),
        }
    }

    #[wasm_bindgen_test]
    fn test_version_string() {
        let n = version_number();
        let major = n / 1_000_000;
        let minor = (n % 1_000_000) / 1_000;
        let patch = n % 1_000;

        assert!(version().contains(&format!("{}.{}.{}", major, minor, patch)));
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "functions")]
    fn test_interrupt() {
        let db = checked_memory_handle();

        let interrupt_handle = db.get_interrupt_handle();

        db.create_scalar_function(
            "interrupt",
            0,
            crate::functions::FunctionFlags::default(),
            move |_| {
                interrupt_handle.interrupt();
                Ok(0)
            },
        )
            .unwrap();

        let mut stmt = db
            .prepare("SELECT interrupt() FROM (SELECT 1 UNION SELECT 2 UNION SELECT 3)")
            .unwrap();

        let result: Result<Vec<i32>> = stmt.query(NO_PARAMS).unwrap().map(|r| r.get(0)).collect();

        match result.unwrap_err() {
            Error::SqliteFailure(err, _) => {
                assert_eq!(err.code, ErrorCode::OperationInterrupted);
            }
            err => {
                panic!("Unexpected error {}", err);
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_interrupt_close() {
        let db = checked_memory_handle();
        let handle = db.get_interrupt_handle();
        handle.interrupt();
        db.close().unwrap();
        handle.interrupt();

        // Look at it's internals to see if we cleared it out properly.
        let db_guard = handle.db_lock.lock().unwrap();
        assert!(db_guard.is_null());
        // It would be nice to test that we properly handle close/interrupt
        // running at the same time, but it seems impossible to do with any
        // degree of reliability.
    }

    #[wasm_bindgen_test]
    fn test_get_raw() {
        let db = checked_memory_handle();
        db.execute_batch("CREATE TABLE foo(i, x);").unwrap();
        let vals = ["foobar", "1234", "qwerty"];
        let mut insert_stmt = db.prepare("INSERT INTO foo(i, x) VALUES(?, ?)").unwrap();
        for (i, v) in vals.iter().enumerate() {
            let i_to_insert = i as i64;
            assert_eq!(insert_stmt.execute(params![i_to_insert, v]).unwrap(), 1);
        }

        let mut query = db.prepare("SELECT i, x FROM foo").unwrap();
        let mut rows = query.query(NO_PARAMS).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let i = row.get_raw(0).as_i64().unwrap();
            let expect = vals[i as usize];
            let x = row.get_raw("x").as_str().unwrap();
            assert_eq!(x, expect);
        }
    }

    #[wasm_bindgen_test]
    fn test_from_handle() {
        let db = checked_memory_handle();
        let handle = unsafe { db.handle() };
        {
            let db = unsafe { Connection::from_handle(handle) }.unwrap();
            db.execute_batch("PRAGMA VACUUM").unwrap();
        }
        db.close().unwrap();
    }

    mod query_and_then_tests {
        use super::*;

        #[derive(Debug)]
        enum CustomError {
            SomeError,
            Sqlite(Error),
        }

        impl fmt::Display for CustomError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
                match *self {
                    CustomError::SomeError => write!(f, "my custom error"),
                    CustomError::Sqlite(ref se) => write!(f, "my custom error: {}", se),
                }
            }
        }

        impl StdError for CustomError {
            fn description(&self) -> &str {
                "my custom error"
            }

            fn cause(&self) -> Option<&dyn StdError> {
                match *self {
                    CustomError::SomeError => None,
                    CustomError::Sqlite(ref se) => Some(se),
                }
            }
        }

        impl From<Error> for CustomError {
            fn from(se: Error) -> CustomError {
                CustomError::Sqlite(se)
            }
        }

        type CustomResult<T> = ::std::result::Result<T, CustomError>;

        #[wasm_bindgen_test]
        fn test_query_and_then() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       INSERT INTO foo VALUES(3, \", \");
                       INSERT INTO foo VALUES(2, \"world\");
                       INSERT INTO foo VALUES(1, \"!\");
                       END;";
            db.execute_batch(sql).unwrap();

            let mut query = db.prepare("SELECT x, y FROM foo ORDER BY x DESC").unwrap();
            let results: Result<Vec<String>> = query
                .query_and_then(NO_PARAMS, |row| row.get(1))
                .unwrap()
                .collect();

            assert_eq!(results.unwrap().concat(), "hello, world!");
        }

        #[wasm_bindgen_test]
        fn test_query_and_then_fails() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       INSERT INTO foo VALUES(3, \", \");
                       INSERT INTO foo VALUES(2, \"world\");
                       INSERT INTO foo VALUES(1, \"!\");
                       END;";
            db.execute_batch(sql).unwrap();

            let mut query = db.prepare("SELECT x, y FROM foo ORDER BY x DESC").unwrap();
            let bad_type: Result<Vec<f64>> = query
                .query_and_then(NO_PARAMS, |row| row.get(1))
                .unwrap()
                .collect();

            match bad_type.unwrap_err() {
                Error::InvalidColumnType(..) => (),
                err => panic!("Unexpected error {}", err),
            }

            let bad_idx: Result<Vec<String>> = query
                .query_and_then(NO_PARAMS, |row| row.get(3))
                .unwrap()
                .collect();

            match bad_idx.unwrap_err() {
                Error::InvalidColumnIndex(_) => (),
                err => panic!("Unexpected error {}", err),
            }
        }

        #[wasm_bindgen_test]
        fn test_query_and_then_custom_error() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       INSERT INTO foo VALUES(3, \", \");
                       INSERT INTO foo VALUES(2, \"world\");
                       INSERT INTO foo VALUES(1, \"!\");
                       END;";
            db.execute_batch(sql).unwrap();

            let mut query = db.prepare("SELECT x, y FROM foo ORDER BY x DESC").unwrap();
            let results: CustomResult<Vec<String>> = query
                .query_and_then(NO_PARAMS, |row| row.get(1).map_err(CustomError::Sqlite))
                .unwrap()
                .collect();

            assert_eq!(results.unwrap().concat(), "hello, world!");
        }

        #[wasm_bindgen_test]
        fn test_query_and_then_custom_error_fails() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       INSERT INTO foo VALUES(3, \", \");
                       INSERT INTO foo VALUES(2, \"world\");
                       INSERT INTO foo VALUES(1, \"!\");
                       END;";
            db.execute_batch(sql).unwrap();

            let mut query = db.prepare("SELECT x, y FROM foo ORDER BY x DESC").unwrap();
            let bad_type: CustomResult<Vec<f64>> = query
                .query_and_then(NO_PARAMS, |row| row.get(1).map_err(CustomError::Sqlite))
                .unwrap()
                .collect();

            match bad_type.unwrap_err() {
                CustomError::Sqlite(Error::InvalidColumnType(..)) => (),
                err => panic!("Unexpected error {}", err),
            }

            let bad_idx: CustomResult<Vec<String>> = query
                .query_and_then(NO_PARAMS, |row| row.get(3).map_err(CustomError::Sqlite))
                .unwrap()
                .collect();

            match bad_idx.unwrap_err() {
                CustomError::Sqlite(Error::InvalidColumnIndex(_)) => (),
                err => panic!("Unexpected error {}", err),
            }

            let non_sqlite_err: CustomResult<Vec<String>> = query
                .query_and_then(NO_PARAMS, |_| Err(CustomError::SomeError))
                .unwrap()
                .collect();

            match non_sqlite_err.unwrap_err() {
                CustomError::SomeError => (),
                err => panic!("Unexpected error {}", err),
            }
        }

        #[wasm_bindgen_test]
        fn test_query_row_and_then_custom_error() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       END;";
            db.execute_batch(sql).unwrap();

            let query = "SELECT x, y FROM foo ORDER BY x DESC";
            let results: CustomResult<String> = db.query_row_and_then(query, NO_PARAMS, |row| {
                row.get(1).map_err(CustomError::Sqlite)
            });

            assert_eq!(results.unwrap(), "hello");
        }

        #[wasm_bindgen_test]
        fn test_query_row_and_then_custom_error_fails() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       END;";
            db.execute_batch(sql).unwrap();

            let query = "SELECT x, y FROM foo ORDER BY x DESC";
            let bad_type: CustomResult<f64> = db.query_row_and_then(query, NO_PARAMS, |row| {
                row.get(1).map_err(CustomError::Sqlite)
            });

            match bad_type.unwrap_err() {
                CustomError::Sqlite(Error::InvalidColumnType(..)) => (),
                err => panic!("Unexpected error {}", err),
            }

            let bad_idx: CustomResult<String> = db.query_row_and_then(query, NO_PARAMS, |row| {
                row.get(3).map_err(CustomError::Sqlite)
            });

            match bad_idx.unwrap_err() {
                CustomError::Sqlite(Error::InvalidColumnIndex(_)) => (),
                err => panic!("Unexpected error {}", err),
            }

            let non_sqlite_err: CustomResult<String> =
                db.query_row_and_then(query, NO_PARAMS, |_| Err(CustomError::SomeError));

            match non_sqlite_err.unwrap_err() {
                CustomError::SomeError => (),
                err => panic!("Unexpected error {}", err),
            }
        }

        #[wasm_bindgen_test]
        fn test_dynamic() {
            let db = checked_memory_handle();
            let sql = "BEGIN;
                       CREATE TABLE foo(x INTEGER, y TEXT);
                       INSERT INTO foo VALUES(4, \"hello\");
                       END;";
            db.execute_batch(sql).unwrap();

            db.query_row("SELECT * FROM foo", params![], |r| {
                assert_eq!(2, r.column_count());
                Ok(())
            })
                .unwrap();
        }

        #[wasm_bindgen_test]
        fn test_dyn_box() {
            let db = checked_memory_handle();
            db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();
            let b: Box<dyn ToSql> = Box::new(5);
            db.execute("INSERT INTO foo VALUES(?)", &[b]).unwrap();
            db.query_row("SELECT x FROM foo", params![], |r| {
                assert_eq!(5, r.get_unwrap::<_, i32>(0));
                Ok(())
            })
                .unwrap();
        }

        #[wasm_bindgen_test]
        fn test_params() {
            let db = checked_memory_handle();
            db.query_row(
                "SELECT
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?;",
                params![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1,
                ],
                |r| {
                    assert_eq!(1, r.get_unwrap::<_, i32>(0));
                    Ok(())
                },
            )
                .unwrap();
        }

        #[wasm_bindgen_test]
        #[cfg(not(feature = "extra_check"))]
        fn test_alter_table() {
            let db = checked_memory_handle();
            db.execute_batch("CREATE TABLE x(t);").unwrap();
            // `execute_batch` should be used but `execute` should also work
            db.execute("ALTER TABLE x RENAME TO y;", params![]).unwrap();
        }
    }

    use rusqlite::{Connection, DatabaseName, Result};
    use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

    fn db_with_test_blob() -> Result<(Connection, i64)> {
        let db = Connection::open_in_memory()?;
        let sql = "BEGIN;
                   CREATE TABLE test (content BLOB);
                   INSERT INTO test VALUES (ZEROBLOB(10));
                   END;";
        db.execute_batch(sql)?;
        let rowid = db.last_insert_rowid();
        Ok((db, rowid))
    }

    #[wasm_bindgen_test]
    fn test_blob() {
        let (db, rowid) = db_with_test_blob().unwrap();

        let mut blob = db
            .blob_open(DatabaseName::Main, "test", "content", rowid, false)
            .unwrap();
        assert_eq!(4, blob.write(b"Clob").unwrap());
        assert_eq!(6, blob.write(b"567890xxxxxx").unwrap()); // cannot write past 10
        assert_eq!(0, blob.write(b"5678").unwrap()); // still cannot write past 10

        blob.reopen(rowid).unwrap();
        blob.close().unwrap();

        blob = db
            .blob_open(DatabaseName::Main, "test", "content", rowid, true)
            .unwrap();
        let mut bytes = [0u8; 5];
        assert_eq!(5, blob.read(&mut bytes[..]).unwrap());
        assert_eq!(&bytes, b"Clob5");
        assert_eq!(5, blob.read(&mut bytes[..]).unwrap());
        assert_eq!(&bytes, b"67890");
        assert_eq!(0, blob.read(&mut bytes[..]).unwrap());

        blob.seek(SeekFrom::Start(2)).unwrap();
        assert_eq!(5, blob.read(&mut bytes[..]).unwrap());
        assert_eq!(&bytes, b"ob567");

        // only first 4 bytes of `bytes` should be read into
        blob.seek(SeekFrom::Current(-1)).unwrap();
        assert_eq!(4, blob.read(&mut bytes[..]).unwrap());
        assert_eq!(&bytes, b"78907");

        blob.seek(SeekFrom::End(-6)).unwrap();
        assert_eq!(5, blob.read(&mut bytes[..]).unwrap());
        assert_eq!(&bytes, b"56789");

        blob.reopen(rowid).unwrap();
        assert_eq!(5, blob.read(&mut bytes[..]).unwrap());
        assert_eq!(&bytes, b"Clob5");

        // should not be able to seek negative or past end
        assert!(blob.seek(SeekFrom::Current(-20)).is_err());
        assert!(blob.seek(SeekFrom::End(0)).is_ok());
        assert!(blob.seek(SeekFrom::Current(1)).is_err());

        // write_all should detect when we return Ok(0) because there is no space left,
        // and return a write error
        blob.reopen(rowid).unwrap();
        assert!(blob.write_all(b"0123456789x").is_err());
    }

    #[wasm_bindgen_test]
    fn test_blob_in_bufreader() {
        let (db, rowid) = db_with_test_blob().unwrap();

        let mut blob = db
            .blob_open(DatabaseName::Main, "test", "content", rowid, false)
            .unwrap();
        assert_eq!(8, blob.write(b"one\ntwo\n").unwrap());

        blob.reopen(rowid).unwrap();
        let mut reader = BufReader::new(blob);

        let mut line = String::new();
        assert_eq!(4, reader.read_line(&mut line).unwrap());
        assert_eq!("one\n", line);

        line.truncate(0);
        assert_eq!(4, reader.read_line(&mut line).unwrap());
        assert_eq!("two\n", line);

        line.truncate(0);
        assert_eq!(2, reader.read_line(&mut line).unwrap());
        assert_eq!("\0\0", line);
    }

    #[wasm_bindgen_test]
    fn test_blob_in_bufwriter() {
        let (db, rowid) = db_with_test_blob().unwrap();

        {
            let blob = db
                .blob_open(DatabaseName::Main, "test", "content", rowid, false)
                .unwrap();
            let mut writer = BufWriter::new(blob);

            // trying to write too much and then flush should fail
            assert_eq!(8, writer.write(b"01234567").unwrap());
            assert_eq!(8, writer.write(b"01234567").unwrap());
            assert!(writer.flush().is_err());
        }

        {
            // ... but it should've written the first 10 bytes
            let mut blob = db
                .blob_open(DatabaseName::Main, "test", "content", rowid, false)
                .unwrap();
            let mut bytes = [0u8; 10];
            assert_eq!(10, blob.read(&mut bytes[..]).unwrap());
            assert_eq!(b"0123456701", &bytes);
        }

        {
            let blob = db
                .blob_open(DatabaseName::Main, "test", "content", rowid, false)
                .unwrap();
            let mut writer = BufWriter::new(blob);

            // trying to write_all too much should fail
            writer.write_all(b"aaaaaaaaaabbbbb").unwrap();
            assert!(writer.flush().is_err());
        }

        {
            // ... but it should've written the first 10 bytes
            let mut blob = db
                .blob_open(DatabaseName::Main, "test", "content", rowid, false)
                .unwrap();
            let mut bytes = [0u8; 10];
            assert_eq!(10, blob.read(&mut bytes[..]).unwrap());
            assert_eq!(b"aaaaaaaaaa", &bytes);
        }
    }

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::sync_channel;
    use std::thread;
    use std::time::Duration;

    use rusqlite::{Error, ErrorCode, TransactionBehavior, NO_PARAMS};
/*
    #[wasm_bindgen_test]
    fn test_default_busy() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.db3");

        let mut db1 = Connection::open(&path).unwrap();
        let tx1 = db1
            .transaction_with_behavior(TransactionBehavior::Exclusive)
            .unwrap();
        let db2 = Connection::open(&path).unwrap();
        let r: Result<()> = db2.query_row("PRAGMA schema_version", NO_PARAMS, |_| unreachable!());
        match r.unwrap_err() {
            Error::SqliteFailure(err, _) => {
                assert_eq!(err.code, ErrorCode::DatabaseBusy);
            }
            err => panic!("Unexpected error {}", err),
        }
        tx1.rollback().unwrap();
    }
*/

    use rusqlite::StatementCache;

    trait StatementCacheEX {
        fn clear(&self);

        fn len(&self) -> usize;

        fn capacity(&self) -> usize;
    }

    impl StatementCacheEX for StatementCache {
        fn clear(&self) {
            self.0.borrow_mut().clear();
        }

        fn len(&self) -> usize {
            self.0.borrow().len()
        }

        fn capacity(&self) -> usize {
            self.0.borrow().capacity()
        }
    }

    #[wasm_bindgen_test]
    fn test_cache() {
        let db = Connection::open_in_memory().unwrap();
        let cache = &db.cache;
        let initial_capacity = cache.capacity();
        assert_eq!(0, cache.len());
        assert!(initial_capacity > 0);

        let sql = "PRAGMA schema_version";
        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(1, cache.len());

        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(1, cache.len());

        cache.clear();
        assert_eq!(0, cache.len());
        assert_eq!(initial_capacity, cache.capacity());
    }

    #[wasm_bindgen_test]
    fn test_set_capacity() {
        let db = Connection::open_in_memory().unwrap();
        let cache = &db.cache;

        let sql = "PRAGMA schema_version";
        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(1, cache.len());

        db.set_prepared_statement_cache_capacity(0);
        assert_eq!(0, cache.len());

        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(0, cache.len());

        db.set_prepared_statement_cache_capacity(8);
        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(1, cache.len());
    }

    #[wasm_bindgen_test]
    fn test_discard() {
        let db = Connection::open_in_memory().unwrap();
        let cache = &db.cache;

        let sql = "PRAGMA schema_version";
        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
            stmt.discard();
        }
        assert_eq!(0, cache.len());
    }

    #[wasm_bindgen_test]
    fn test_ddl() {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            r#"
            CREATE TABLE foo (x INT);
            INSERT INTO foo VALUES (1);
        "#,
        )
            .unwrap();

        let sql = "SELECT * FROM foo";

        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(
                Ok(Some(1i32)),
                stmt.query(NO_PARAMS).unwrap().map(|r| r.get(0)).next()
            );
        }

        db.execute_batch(
            r#"
            ALTER TABLE foo ADD COLUMN y INT;
            UPDATE foo SET y = 2;
        "#,
        )
            .unwrap();

        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(
                Ok(Some((1i32, 2i32))),
                stmt.query(NO_PARAMS)
                    .unwrap()
                    .map(|r| Ok((r.get(0)?, r.get(1)?)))
                    .next()
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_connection_close() {
        let conn = Connection::open_in_memory().unwrap();
        conn.prepare_cached("SELECT * FROM sqlite_master;").unwrap();

        conn.close().expect("connection not closed");
    }

    #[wasm_bindgen_test]
    fn test_cache_key() {
        let db = Connection::open_in_memory().unwrap();
        let cache = &db.cache;
        assert_eq!(0, cache.len());

        //let sql = " PRAGMA schema_version; -- comment";
        let sql = "PRAGMA schema_version; ";
        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(1, cache.len());

        {
            let mut stmt = db.prepare_cached(sql).unwrap();
            assert_eq!(0, cache.len());
            assert_eq!(
                0,
                stmt.query_row(NO_PARAMS, |r| r.get::<_, i64>(0)).unwrap()
            );
        }
        assert_eq!(1, cache.len());
    }

    #[wasm_bindgen_test]
    fn test_empty_stmt() {
        let conn = Connection::open_in_memory().unwrap();
        conn.prepare_cached("").unwrap();
    }

    use fallible_streaming_iterator::FallibleStreamingIterator;
    use unicase::UniCase;


    fn unicase_compare(s1: &str, s2: &str) -> std::cmp::Ordering {
        UniCase::new(s1).cmp(&UniCase::new(s2))
    }

    #[wasm_bindgen_test]
    fn test_unicase() {
        let db = Connection::open_in_memory().unwrap();

        db.create_collation("unicase", unicase_compare).unwrap();

        collate(db);
    }

    fn collate(db: Connection) {
        db.execute_batch(
            "CREATE TABLE foo (bar);
             INSERT INTO foo (bar) VALUES ('Maße');
             INSERT INTO foo (bar) VALUES ('MASSE');",
        )
            .unwrap();
        let mut stmt = db
            .prepare("SELECT DISTINCT bar COLLATE unicase FROM foo ORDER BY 1")
            .unwrap();
        let rows = stmt.query(NO_PARAMS).unwrap();
        assert_eq!(rows.count().unwrap(), 1);
    }

    fn collation_needed(db: &Connection, collation_name: &str) -> Result<()> {
        if "unicase" == collation_name {
            db.create_collation(collation_name, unicase_compare)
        } else {
            Ok(())
        }
    }

    #[wasm_bindgen_test]
    fn test_collation_needed() {
        let db = Connection::open_in_memory().unwrap();
        db.collation_needed(collation_needed).unwrap();
        collate(db);
    }

    use rusqlite::Column;


    #[wasm_bindgen_test]
    fn test_columns() {
        let db = Connection::open_in_memory().unwrap();
        let query = db.prepare("SELECT * FROM sqlite_master").unwrap();
        let columns = query.columns();
        let column_names: Vec<&str> = columns.iter().map(Column::name).collect();
        assert_eq!(
            column_names.as_slice(),
            &["type", "name", "tbl_name", "rootpage", "sql"]
        );
        let column_types: Vec<Option<&str>> = columns.iter().map(Column::decl_type).collect();
        assert_eq!(
            &column_types[..3],
            &[Some("text"), Some("text"), Some("text"), ]
        );
    }

    #[wasm_bindgen_test]
    fn test_column_name_in_error() {
        use rusqlite::{types::Type, Error};
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "BEGIN;
             CREATE TABLE foo(x INTEGER, y TEXT);
             INSERT INTO foo VALUES(4, NULL);
             END;",
        )
            .unwrap();
        let mut stmt = db.prepare("SELECT x as renamed, y FROM foo").unwrap();
        let mut rows = stmt.query(rusqlite::NO_PARAMS).unwrap();
        let row = rows.next().unwrap().unwrap();
        match row.get::<_, String>(0).unwrap_err() {
            Error::InvalidColumnType(idx, name, ty) => {
                assert_eq!(idx, 0);
                assert_eq!(name, "renamed");
                assert_eq!(ty, Type::Integer);
            }
            e => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
        match row.get::<_, String>("y").unwrap_err() {
            Error::InvalidColumnType(idx, name, ty) => {
                assert_eq!(idx, 1);
                assert_eq!(name, "y");
                assert_eq!(ty, Type::Null);
            }
            e => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
    }

    use wasm_bindgen_test::*;

    use rusqlite::config::DbConfig;


    #[wasm_bindgen_test]
    fn test_db_config() {
        let db = Connection::open_in_memory().unwrap();

        let opposite = !db.db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY).unwrap();
        assert_eq!(
            db.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, opposite),
            Ok(opposite)
        );
        assert_eq!(
            db.db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY),
            Ok(opposite)
        );

        let opposite = !db
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER)
            .unwrap();
        assert_eq!(
            db.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER, opposite),
            Ok(opposite)
        );
        assert_eq!(
            db.db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER),
            Ok(opposite)
        );
    }

    extern crate regex;

    use std::f64::EPSILON;
    use std::os::raw::{c_double, c_int};

    #[cfg(feature = "window")]
    use rusqlite::functions::WindowAggregate;
    use rusqlite::functions::{Aggregate, Context, FunctionFlags};
    use self::regex::Regex;

    fn half(ctx: &Context<'_>) -> Result<c_double> {
        assert_eq!(ctx.len(), 1, "called with unexpected number of arguments");
        let value = ctx.get::<c_double>(0)?;
        Ok(value / 2f64)
    }

    #[wasm_bindgen_test]
    fn test_function_half() {
        let db = Connection::open_in_memory().unwrap();
        db.create_scalar_function(
            "half",
            1,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            half,
        )
            .unwrap();
        let result: Result<f64> = db.query_row("SELECT half(6)", NO_PARAMS, |r| r.get(0));

        assert!((3f64 - result.unwrap()).abs() < EPSILON);
    }

    #[wasm_bindgen_test]
    fn test_remove_function() {
        let db = Connection::open_in_memory().unwrap();
        db.create_scalar_function(
            "half",
            1,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            half,
        )
            .unwrap();
        let result: Result<f64> = db.query_row("SELECT half(6)", NO_PARAMS, |r| r.get(0));
        assert!((3f64 - result.unwrap()).abs() < EPSILON);

        db.remove_function("half", 1).unwrap();
        let result: Result<f64> = db.query_row("SELECT half(6)", NO_PARAMS, |r| r.get(0));
        assert!(result.is_err());
    }

    // This implementation of a regexp scalar function uses SQLite's auxilliary data
    // (https://www.sqlite.org/c3ref/get_auxdata.html) to avoid recompiling the regular
    // expression multiple times within one query.
    fn regexp_with_auxilliary(ctx: &Context<'_>) -> Result<bool> {
        assert_eq!(ctx.len(), 2, "called with unexpected number of arguments");

        let saved_re: Option<&Regex> = ctx.get_aux(0)?;
        let new_re = match saved_re {
            None => {
                let s = ctx.get::<String>(0)?;
                match Regex::new(&s) {
                    Ok(r) => Some(r),
                    Err(err) => return Err(Error::UserFunctionError(Box::new(err))),
                }
            }
            Some(_) => None,
        };

        let is_match = {
            let re = saved_re.unwrap_or_else(|| new_re.as_ref().unwrap());

            let text = ctx
                .get_raw(1)
                .as_str()
                .map_err(|e| Error::UserFunctionError(e.into()))?;

            re.is_match(text)
        };

        if let Some(re) = new_re {
            ctx.set_aux(0, re);
        }

        Ok(is_match)
    }

    #[wasm_bindgen_test]
    fn test_function_regexp_with_auxilliary() {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "BEGIN;
             CREATE TABLE foo (x string);
             INSERT INTO foo VALUES ('lisa');
             INSERT INTO foo VALUES ('lXsi');
             INSERT INTO foo VALUES ('lisX');
             END;",
        )
            .unwrap();
        db.create_scalar_function(
            "regexp",
            2,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            regexp_with_auxilliary,
        )
            .unwrap();

        let result: Result<bool> =
            db.query_row("SELECT regexp('l.s[aeiouy]', 'lisa')", NO_PARAMS, |r| {
                r.get(0)
            });

        assert_eq!(true, result.unwrap());

        let result: Result<i64> = db.query_row(
            "SELECT COUNT(*) FROM foo WHERE regexp('l.s[aeiouy]', x) == 1",
            NO_PARAMS,
            |r| r.get(0),
        );

        assert_eq!(2, result.unwrap());
    }

    #[wasm_bindgen_test]
    fn test_varargs_function() {
        let db = Connection::open_in_memory().unwrap();
        db.create_scalar_function(
            "my_concat",
            -1,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |ctx| {
                let mut ret = String::new();

                for idx in 0..ctx.len() {
                    let s = ctx.get::<String>(idx)?;
                    ret.push_str(&s);
                }

                Ok(ret)
            },
        )
            .unwrap();

        for &(expected, query) in &[
            ("", "SELECT my_concat()"),
            ("onetwo", "SELECT my_concat('one', 'two')"),
            ("abc", "SELECT my_concat('a', 'b', 'c')"),
        ] {
            let result: String = db.query_row(query, NO_PARAMS, |r| r.get(0)).unwrap();
            assert_eq!(expected, result);
        }
    }

    #[wasm_bindgen_test]
    fn test_get_aux_type_checking() {
        let db = Connection::open_in_memory().unwrap();
        db.create_scalar_function("example", 2, FunctionFlags::default(), |ctx| {
            if !ctx.get::<bool>(1)? {
                ctx.set_aux::<i64>(0, 100);
            } else {
                assert_eq!(ctx.get_aux::<String>(0), Err(Error::GetAuxWrongType));
                assert_eq!(ctx.get_aux::<i64>(0), Ok(Some(&100)));
            }
            Ok(true)
        })
            .unwrap();

        let res: bool = db
            .query_row(
                "SELECT example(0, i) FROM (SELECT 0 as i UNION SELECT 1)",
                NO_PARAMS,
                |r| r.get(0),
            )
            .unwrap();
        // Doesn't actually matter, we'll assert in the function if there's a problem.
        assert!(res);
    }

    struct Sum;

    struct Count;

    impl Aggregate<i64, Option<i64>> for Sum {
        fn init(&self) -> i64 {
            0
        }

        fn step(&self, ctx: &mut Context<'_>, sum: &mut i64) -> Result<()> {
            *sum += ctx.get::<i64>(0)?;
            Ok(())
        }

        fn finalize(&self, sum: Option<i64>) -> Result<Option<i64>> {
            Ok(sum)
        }
    }

    impl Aggregate<i64, i64> for Count {
        fn init(&self) -> i64 {
            0
        }

        fn step(&self, _ctx: &mut Context<'_>, sum: &mut i64) -> Result<()> {
            *sum += 1;
            Ok(())
        }

        fn finalize(&self, sum: Option<i64>) -> Result<i64> {
            Ok(sum.unwrap_or(0))
        }
    }

    #[wasm_bindgen_test]
    fn test_sum() {
        let db = Connection::open_in_memory().unwrap();
        db.create_aggregate_function(
            "my_sum",
            1,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            Sum,
        )
            .unwrap();

        // sum should return NULL when given no columns (contrast with count below)
        let no_result = "SELECT my_sum(i) FROM (SELECT 2 AS i WHERE 1 <> 1)";
        let result: Option<i64> = db.query_row(no_result, NO_PARAMS, |r| r.get(0)).unwrap();
        assert!(result.is_none());

        let single_sum = "SELECT my_sum(i) FROM (SELECT 2 AS i UNION ALL SELECT 2)";
        let result: i64 = db.query_row(single_sum, NO_PARAMS, |r| r.get(0)).unwrap();
        assert_eq!(4, result);

        let dual_sum = "SELECT my_sum(i), my_sum(j) FROM (SELECT 2 AS i, 1 AS j UNION ALL SELECT \
                        2, 1)";
        let result: (i64, i64) = db
            .query_row(dual_sum, NO_PARAMS, |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap();
        assert_eq!((4, 2), result);
    }

    #[wasm_bindgen_test]
    fn test_count() {
        let db = Connection::open_in_memory().unwrap();
        db.create_aggregate_function(
            "my_count",
            -1,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            Count,
        )
            .unwrap();

        // count should return 0 when given no columns (contrast with sum above)
        let no_result = "SELECT my_count(i) FROM (SELECT 2 AS i WHERE 1 <> 1)";
        let result: i64 = db.query_row(no_result, NO_PARAMS, |r| r.get(0)).unwrap();
        assert_eq!(result, 0);

        let single_sum = "SELECT my_count(i) FROM (SELECT 2 AS i UNION ALL SELECT 2)";
        let result: i64 = db.query_row(single_sum, NO_PARAMS, |r| r.get(0)).unwrap();
        assert_eq!(2, result);
    }

    #[cfg(feature = "window")]
    impl WindowAggregate<i64, Option<i64>> for Sum {
        fn inverse(&self, ctx: &mut Context<'_>, sum: &mut i64) -> Result<()> {
            *sum -= ctx.get::<i64>(0)?;
            Ok(())
        }

        fn value(&self, sum: Option<&i64>) -> Result<Option<i64>> {
            Ok(sum.copied())
        }
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "window")]
    fn test_window() {
        use fallible_iterator::FallibleIterator;

        let db = Connection::open_in_memory().unwrap();
        db.create_window_function(
            "sumint",
            1,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            Sum,
        )
            .unwrap();
        db.execute_batch(
            "CREATE TABLE t3(x, y);
             INSERT INTO t3 VALUES('a', 4),
                     ('b', 5),
                     ('c', 3),
                     ('d', 8),
                     ('e', 1);",
        )
            .unwrap();

        let mut stmt = db
            .prepare(
                "SELECT x, sumint(y) OVER (
                   ORDER BY x ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING
                 ) AS sum_y
                 FROM t3 ORDER BY x;",
            )
            .unwrap();

        let results: Vec<(String, i64)> = stmt
            .query(NO_PARAMS)
            .unwrap()
            .map(|row| Ok((row.get("x")?, row.get("sum_y")?)))
            .collect()
            .unwrap();
        let expected = vec![
            ("a".to_owned(), 9),
            ("b".to_owned(), 12),
            ("c".to_owned(), 16),
            ("d".to_owned(), 12),
            ("e".to_owned(), 9),
        ];
        assert_eq!(expected, results);
    }

    use rusqlite::Action;


    #[wasm_bindgen_test]
    fn test_commit_hook() {
        let db = Connection::open_in_memory().unwrap();

        lazy_static! {
            static ref CALLED: AtomicBool = AtomicBool::new(false);
        }
        db.commit_hook(Some(|| {
            CALLED.store(true, Ordering::Relaxed);
            false
        }));
        db.execute_batch("BEGIN; CREATE TABLE foo (t TEXT); COMMIT;")
            .unwrap();
        assert!(CALLED.load(Ordering::Relaxed));
    }

    #[wasm_bindgen_test]
    fn test_fn_commit_hook() {
        let db = Connection::open_in_memory().unwrap();

        fn hook() -> bool {
            true
        }

        db.commit_hook(Some(hook));
        db.execute_batch("BEGIN; CREATE TABLE foo (t TEXT); COMMIT;")
            .unwrap_err();
    }

    #[wasm_bindgen_test]
    fn test_rollback_hook() {
        let db = Connection::open_in_memory().unwrap();

        lazy_static! {
            static ref CALLED: AtomicBool = AtomicBool::new(false);
        }
        db.rollback_hook(Some(|| {
            CALLED.store(true, Ordering::Relaxed);
        }));
        db.execute_batch("BEGIN; CREATE TABLE foo (t TEXT); ROLLBACK;")
            .unwrap();
        assert!(CALLED.load(Ordering::Relaxed));
    }

    #[wasm_bindgen_test]
    fn test_update_hook() {
        let db = Connection::open_in_memory().unwrap();

        lazy_static! {
            static ref CALLED: AtomicBool = AtomicBool::new(false);
        }
        db.update_hook(Some(|action, db: &str, tbl: &str, row_id| {
            assert_eq!(Action::SQLITE_INSERT, action);
            assert_eq!("main", db);
            assert_eq!("foo", tbl);
            assert_eq!(1, row_id);
            CALLED.store(true, Ordering::Relaxed);
        }));
        db.execute_batch("CREATE TABLE foo (t TEXT)").unwrap();
        db.execute_batch("INSERT INTO foo VALUES ('lisa')").unwrap();
        assert!(CALLED.load(Ordering::Relaxed));
    }

    use rusqlite::ffi::Limit;


    #[wasm_bindgen_test]
    fn test_limit() {
        let db = Connection::open_in_memory().unwrap();
        db.set_limit(Limit::SQLITE_LIMIT_LENGTH, 1024);
        assert_eq!(1024, db.limit(Limit::SQLITE_LIMIT_LENGTH));

        db.set_limit(Limit::SQLITE_LIMIT_SQL_LENGTH, 1024);
        assert_eq!(1024, db.limit(Limit::SQLITE_LIMIT_SQL_LENGTH));

        db.set_limit(Limit::SQLITE_LIMIT_COLUMN, 64);
        assert_eq!(64, db.limit(Limit::SQLITE_LIMIT_COLUMN));

        db.set_limit(Limit::SQLITE_LIMIT_EXPR_DEPTH, 256);
        assert_eq!(256, db.limit(Limit::SQLITE_LIMIT_EXPR_DEPTH));

        db.set_limit(Limit::SQLITE_LIMIT_COMPOUND_SELECT, 32);
        assert_eq!(32, db.limit(Limit::SQLITE_LIMIT_COMPOUND_SELECT));

        db.set_limit(Limit::SQLITE_LIMIT_FUNCTION_ARG, 32);
        assert_eq!(32, db.limit(Limit::SQLITE_LIMIT_FUNCTION_ARG));

        db.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 2);
        assert_eq!(2, db.limit(Limit::SQLITE_LIMIT_ATTACHED));

        db.set_limit(Limit::SQLITE_LIMIT_LIKE_PATTERN_LENGTH, 128);
        assert_eq!(128, db.limit(Limit::SQLITE_LIMIT_LIKE_PATTERN_LENGTH));

        db.set_limit(Limit::SQLITE_LIMIT_VARIABLE_NUMBER, 99);
        assert_eq!(99, db.limit(Limit::SQLITE_LIMIT_VARIABLE_NUMBER));

        // SQLITE_LIMIT_TRIGGER_DEPTH was added in SQLite 3.6.18.
        if rusqlite::version_number() >= 3_006_018 {
            db.set_limit(Limit::SQLITE_LIMIT_TRIGGER_DEPTH, 32);
            assert_eq!(32, db.limit(Limit::SQLITE_LIMIT_TRIGGER_DEPTH));
        }
/*
        // SQLITE_LIMIT_WORKER_THREADS was added in SQLite 3.8.7.
        if rusqlite::version_number() >= 3_008_007 {
            db.set_limit(Limit::SQLITE_LIMIT_WORKER_THREADS, 2);
            assert_eq!(2, db.limit(Limit::SQLITE_LIMIT_WORKER_THREADS));
        }
        */
    }

    use rusqlite::pragma::Sql;
    use rusqlite::pragma;


    #[wasm_bindgen_test]
    fn pragma_query_value() {
        let db = Connection::open_in_memory().unwrap();
        let user_version: i32 = db
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(0, user_version);
    }

    #[wasm_bindgen_test]
    fn pragma_func_query_value() {
        use rusqlite::NO_PARAMS;

        let db = Connection::open_in_memory().unwrap();
        let user_version: i32 = db
            .query_row(
                "SELECT user_version FROM pragma_user_version",
                NO_PARAMS,
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(0, user_version);
    }

    #[wasm_bindgen_test]
    fn pragma_query_no_schema() {
        let db = Connection::open_in_memory().unwrap();
        let mut user_version = -1;
        db.pragma_query(None, "user_version", |row| {
            user_version = row.get(0)?;
            Ok(())
        })
            .unwrap();
        assert_eq!(0, user_version);
    }

    #[wasm_bindgen_test]
    fn pragma_query_with_schema() {
        let db = Connection::open_in_memory().unwrap();
        let mut user_version = -1;
        db.pragma_query(Some(DatabaseName::Main), "user_version", |row| {
            user_version = row.get(0)?;
            Ok(())
        })
            .unwrap();
        assert_eq!(0, user_version);
    }

    #[wasm_bindgen_test]
    fn pragma() {
        let db = Connection::open_in_memory().unwrap();
        let mut columns = Vec::new();
        db.pragma(None, "table_info", &"sqlite_master", |row| {
            let column: String = row.get(1)?;
            columns.push(column);
            Ok(())
        })
            .unwrap();
        assert_eq!(5, columns.len());
    }

    #[wasm_bindgen_test]
    fn pragma_func() {
        let db = Connection::open_in_memory().unwrap();
        let mut table_info = db.prepare("SELECT * FROM pragma_table_info(?)").unwrap();
        let mut columns = Vec::new();
        let mut rows = table_info.query(&["sqlite_master"]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let row = row;
            let column: String = row.get(1).unwrap();
            columns.push(column);
        }
        assert_eq!(5, columns.len());
    }

    #[wasm_bindgen_test]
    fn pragma_update() {
        let db = Connection::open_in_memory().unwrap();
        db.pragma_update(None, "user_version", &1).unwrap();
    }

    #[wasm_bindgen_test]
    fn pragma_update_and_check() {
        let db = Connection::open_in_memory().unwrap();
        let journal_mode: String = db
            .pragma_update_and_check(None, "journal_mode", &"OFF", |row| row.get(0))
            .unwrap();
        assert_eq!("off", &journal_mode);
    }

    #[wasm_bindgen_test]
    fn is_identifier() {
        assert!(pragma::is_identifier("full"));
        assert!(pragma::is_identifier("r2d2"));
        assert!(!pragma::is_identifier("sp ce"));
        assert!(!pragma::is_identifier("semi;colon"));
    }

    #[wasm_bindgen_test]
    fn double_quote() {
        let mut sql = Sql::new();
        sql.push_schema_name(DatabaseName::Attached(r#"schema";--"#));
        assert_eq!(r#""schema"";--""#, sql.as_str());
    }

    #[wasm_bindgen_test]
    fn wrap_and_escape() {
        let mut sql = Sql::new();
        sql.push_string_literal("value'; --");
        assert_eq!("'value''; --'", sql.as_str());
    }

    use rusqlite::types::{ToSql, Value, FromSql};

    #[wasm_bindgen_test]
    fn test_execute_named() {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo(x INTEGER)").unwrap();

        assert_eq!(
            db.execute_named("INSERT INTO foo(x) VALUES (:x)", &[(":x", &1i32)])
                .unwrap(),
            1
        );
        assert_eq!(
            db.execute_named("INSERT INTO foo(x) VALUES (:x)", &[(":x", &2i32)])
                .unwrap(),
            1
        );

        assert_eq!(
            3i32,
            db.query_row_named::<i32, _>(
                "SELECT SUM(x) FROM foo WHERE x > :x",
                &[(":x", &0i32)],
                |r| r.get(0),
            )
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_stmt_execute_named() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag \
                   INTEGER)";
        db.execute_batch(sql).unwrap();

        let mut stmt = db
            .prepare("INSERT INTO test (name) VALUES (:name)")
            .unwrap();
        stmt.execute_named(&[(":name", &"one")]).unwrap();

        let mut stmt = db
            .prepare("SELECT COUNT(*) FROM test WHERE name = :name")
            .unwrap();
        assert_eq!(
            1i32,
            stmt.query_row_named::<i32, _>(&[(":name", &"one")], |r| r.get(0))
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_query_named() {
        let db = Connection::open_in_memory().unwrap();
        let sql = r#"
        CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag INTEGER);
        INSERT INTO test(id, name) VALUES (1, "one");
        "#;
        db.execute_batch(sql).unwrap();

        let mut stmt = db
            .prepare("SELECT id FROM test where name = :name")
            .unwrap();
        let mut rows = stmt.query_named(&[(":name", &"one")]).unwrap();

        let id: Result<i32> = rows.next().unwrap().unwrap().get(0);
        assert_eq!(Ok(1), id);
    }

    #[wasm_bindgen_test]
    fn test_query_map_named() {
        let db = Connection::open_in_memory().unwrap();
        let sql = r#"
        CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag INTEGER);
        INSERT INTO test(id, name) VALUES (1, "one");
        "#;
        db.execute_batch(sql).unwrap();

        let mut stmt = db
            .prepare("SELECT id FROM test where name = :name")
            .unwrap();
        let mut rows = stmt
            .query_map_named(&[(":name", &"one")], |row| {
                let id: Result<i32> = row.get(0);
                id.map(|i| 2 * i)
            })
            .unwrap();

        let doubled_id: i32 = rows.next().unwrap().unwrap();
        assert_eq!(2, doubled_id);
    }

    #[wasm_bindgen_test]
    fn test_query_and_then_named() {
        let db = Connection::open_in_memory().unwrap();
        let sql = r#"
        CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag INTEGER);
        INSERT INTO test(id, name) VALUES (1, "one");
        INSERT INTO test(id, name) VALUES (2, "one");
        "#;
        db.execute_batch(sql).unwrap();

        let mut stmt = db
            .prepare("SELECT id FROM test where name = :name ORDER BY id ASC")
            .unwrap();
        let mut rows = stmt
            .query_and_then_named(&[(":name", &"one")], |row| {
                let id: i32 = row.get(0)?;
                if id == 1 {
                    Ok(id)
                } else {
                    Err(Error::SqliteSingleThreadedMode)
                }
            })
            .unwrap();

        // first row should be Ok
        let doubled_id: i32 = rows.next().unwrap().unwrap();
        assert_eq!(1, doubled_id);

        // second row should be Err
        #[allow(clippy::match_wild_err_arm)]
        match rows.next().unwrap() {
            Ok(_) => panic!("invalid Ok"),
            Err(Error::SqliteSingleThreadedMode) => (),
            Err(_) => panic!("invalid Err"),
        }
    }

    #[wasm_bindgen_test]
    fn test_unbound_parameters_are_null() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "CREATE TABLE test (x TEXT, y TEXT)";
        db.execute_batch(sql).unwrap();

        let mut stmt = db
            .prepare("INSERT INTO test (x, y) VALUES (:x, :y)")
            .unwrap();
        stmt.execute_named(&[(":x", &"one")]).unwrap();

        let result: Option<String> = db
            .query_row("SELECT y FROM test WHERE x = 'one'", NO_PARAMS, |row| {
                row.get(0)
            })
            .unwrap();
        assert!(result.is_none());
    }

    #[wasm_bindgen_test]
    fn test_unbound_parameters_are_reused() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "CREATE TABLE test (x TEXT, y TEXT)";
        db.execute_batch(sql).unwrap();

        let mut stmt = db
            .prepare("INSERT INTO test (x, y) VALUES (:x, :y)")
            .unwrap();
        stmt.execute_named(&[(":x", &"one")]).unwrap();
        stmt.execute_named(&[(":y", &"two")]).unwrap();

        let result: String = db
            .query_row("SELECT x FROM test WHERE y = 'two'", NO_PARAMS, |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "one");
    }

    #[wasm_bindgen_test]
    fn test_insert() {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo(x INTEGER UNIQUE)")
            .unwrap();
        let mut stmt = db
            .prepare("INSERT OR IGNORE INTO foo (x) VALUES (?)")
            .unwrap();
        assert_eq!(stmt.insert(&[1i32]).unwrap(), 1);
        assert_eq!(stmt.insert(&[2i32]).unwrap(), 2);
        match stmt.insert(&[1i32]).unwrap_err() {
            Error::StatementChangedRows(0) => (),
            err => panic!("Unexpected error {}", err),
        }
        let mut multi = db
            .prepare("INSERT INTO foo (x) SELECT 3 UNION ALL SELECT 4")
            .unwrap();
        match multi.insert(NO_PARAMS).unwrap_err() {
            Error::StatementChangedRows(2) => (),
            err => panic!("Unexpected error {}", err),
        }
    }

    #[wasm_bindgen_test]
    fn test_insert_different_tables() {
        // Test for https://github.com/jgallagher/rusqlite/issues/171
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            r"
            CREATE TABLE foo(x INTEGER);
            CREATE TABLE bar(x INTEGER);
        ",
        )
            .unwrap();

        assert_eq!(
            db.prepare("INSERT INTO foo VALUES (10)")
                .unwrap()
                .insert(NO_PARAMS)
                .unwrap(),
            1
        );
        assert_eq!(
            db.prepare("INSERT INTO bar VALUES (10)")
                .unwrap()
                .insert(NO_PARAMS)
                .unwrap(),
            1
        );
    }

    #[wasm_bindgen_test]
    fn test_exists() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER);
                   INSERT INTO foo VALUES(1);
                   INSERT INTO foo VALUES(2);
                   END;";
        db.execute_batch(sql).unwrap();
        let mut stmt = db.prepare("SELECT 1 FROM foo WHERE x = ?").unwrap();
        assert!(stmt.exists(&[1i32]).unwrap());
        assert!(stmt.exists(&[2i32]).unwrap());
        assert!(!stmt.exists(&[0i32]).unwrap());
    }

    #[wasm_bindgen_test]
    fn test_query_rowx() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER, y INTEGER);
                   INSERT INTO foo VALUES(1, 3);
                   INSERT INTO foo VALUES(2, 4);
                   END;";
        db.execute_batch(sql).unwrap();
        let mut stmt = db.prepare("SELECT y FROM foo WHERE x = ?").unwrap();
        let y: Result<i64> = stmt.query_row(&[1i32], |r| r.get(0));
        assert_eq!(3i64, y.unwrap());
    }

    #[wasm_bindgen_test]
    fn test_query_by_column_name() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER, y INTEGER);
                   INSERT INTO foo VALUES(1, 3);
                   END;";
        db.execute_batch(sql).unwrap();
        let mut stmt = db.prepare("SELECT y FROM foo").unwrap();
        let y: Result<i64> = stmt.query_row(NO_PARAMS, |r| r.get("y"));
        assert_eq!(3i64, y.unwrap());
    }

    #[wasm_bindgen_test]
    fn test_query_by_column_name_ignore_case() {
        let db = Connection::open_in_memory().unwrap();
        let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER, y INTEGER);
                   INSERT INTO foo VALUES(1, 3);
                   END;";
        db.execute_batch(sql).unwrap();
        let mut stmt = db.prepare("SELECT y as Y FROM foo").unwrap();
        let y: Result<i64> = stmt.query_row(NO_PARAMS, |r| r.get("y"));
        assert_eq!(3i64, y.unwrap());
    }

    #[wasm_bindgen_test]
    fn test_expanded_sql() {
        let db = Connection::open_in_memory().unwrap();
        let stmt = db.prepare("SELECT ?").unwrap();
        stmt.bind_parameter(&1, 1).unwrap();
        assert_eq!(Some("SELECT 1".to_owned()), stmt.expanded_sql());
    }

    #[wasm_bindgen_test]
    fn test_bind_parameters() {
        let db = Connection::open_in_memory().unwrap();
        // dynamic slice:
        db.query_row(
            "SELECT ?1, ?2, ?3",
            &[&1u8 as &dyn ToSql, &"one", &Some("one")],
            |row| row.get::<_, u8>(0),
        )
            .unwrap();
        // existing collection:
        let data = vec![1, 2, 3];
        db.query_row("SELECT ?1, ?2, ?3", &data, |row| row.get::<_, u8>(0))
            .unwrap();
        db.query_row("SELECT ?1, ?2, ?3", data.as_slice(), |row| {
            row.get::<_, u8>(0)
        })
            .unwrap();
        db.query_row("SELECT ?1, ?2, ?3", data, |row| row.get::<_, u8>(0))
            .unwrap();

        use std::collections::BTreeSet;
        let data: BTreeSet<String> = ["one", "two", "three"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        db.query_row("SELECT ?1, ?2, ?3", &data, |row| row.get::<_, String>(0))
            .unwrap();

        let data = [0; 3];
        db.query_row("SELECT ?1, ?2, ?3", &data, |row| row.get::<_, u8>(0))
            .unwrap();
        db.query_row("SELECT ?1, ?2, ?3", data.iter(), |row| row.get::<_, u8>(0))
            .unwrap();
    }

    #[wasm_bindgen_test]
    fn test_empty_stmtx() {
        let conn = Connection::open_in_memory().unwrap();
        let mut stmt = conn.prepare("").unwrap();
        assert_eq!(0, stmt.column_count());
        assert!(stmt.parameter_index("test").is_ok());
        assert!(stmt.step().is_err());
        stmt.reset();
        assert!(stmt.execute(NO_PARAMS).is_err());
    }

    #[wasm_bindgen_test]
    fn test_comment_stmt() {
        let conn = Connection::open_in_memory().unwrap();
        conn.prepare("/*SELECT 1;*/").unwrap();
    }

    #[wasm_bindgen_test]
    fn test_comment_and_sql_stmt() {
        let conn = Connection::open_in_memory().unwrap();
        let stmt = conn.prepare("/*...*/ SELECT 1;").unwrap();
        assert_eq!(1, stmt.column_count());
    }

    #[wasm_bindgen_test]
    fn test_semi_colon_stmt() {
        let conn = Connection::open_in_memory().unwrap();
        let stmt = conn.prepare(";").unwrap();
        assert_eq!(0, stmt.column_count());
    }

    use std::sync::Mutex;


    #[wasm_bindgen_test]
    fn test_trace() {
        lazy_static! {
            static ref TRACED_STMTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
        }
        fn tracer(s: &str) {
            let mut traced_stmts = TRACED_STMTS.lock().unwrap();
            traced_stmts.push(s.to_owned());
        }

        let mut db = Connection::open_in_memory().unwrap();
        db.trace(Some(tracer));
        {
            let _ = db.query_row("SELECT ?", &[1i32], |_| Ok(()));
            let _ = db.query_row("SELECT ?", &["hello"], |_| Ok(()));
        }
        db.trace(None);
        {
            let _ = db.query_row("SELECT ?", &[2i32], |_| Ok(()));
            let _ = db.query_row("SELECT ?", &["goodbye"], |_| Ok(()));
        }

        let traced_stmts = TRACED_STMTS.lock().unwrap();
        assert_eq!(traced_stmts.len(), 2);
        assert_eq!(traced_stmts[0], "SELECT 1");
        assert_eq!(traced_stmts[1], "SELECT 'hello'");
    }

    #[wasm_bindgen_test]
    fn test_profile() {
        lazy_static! {
            static ref PROFILED: Mutex<Vec<(String, Duration)>> = Mutex::new(Vec::new());
        }
        fn profiler(s: &str, d: Duration) {
            let mut profiled = PROFILED.lock().unwrap();
            profiled.push((s.to_owned(), d));
        }

        let mut db = Connection::open_in_memory().unwrap();
        db.profile(Some(profiler));
        db.execute_batch("PRAGMA application_id = 1").unwrap();
        db.profile(None);
        db.execute_batch("PRAGMA application_id = 2").unwrap();

        let profiled = PROFILED.lock().unwrap();
        assert_eq!(profiled.len(), 1);
        assert_eq!(profiled[0].0, "PRAGMA application_id = 1");
    }

    use rusqlite::DropBehavior;

    fn checked_memory_handlex() -> Connection {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo (x INTEGER)").unwrap();
        db
    }

    #[wasm_bindgen_test]
    fn test_drop() {
        let mut db = checked_memory_handlex();
        {
            let tx = db.transaction().unwrap();
            tx.execute_batch("INSERT INTO foo VALUES(1)").unwrap();
            // default: rollback
        }
        {
            let mut tx = db.transaction().unwrap();
            tx.execute_batch("INSERT INTO foo VALUES(2)").unwrap();
            tx.set_drop_behavior(DropBehavior::Commit)
        }
        {
            let tx = db.transaction().unwrap();
            assert_eq!(
                2i32,
                tx.query_row::<i32, _, _>("SELECT SUM(x) FROM foo", NO_PARAMS, |r| r.get(0))
                    .unwrap()
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_explicit_rollback_commit() {
        let mut db = checked_memory_handlex();
        {
            let mut tx = db.transaction().unwrap();
            {
                let mut sp = tx.savepoint().unwrap();
                sp.execute_batch("INSERT INTO foo VALUES(1)").unwrap();
                sp.rollback().unwrap();
                sp.execute_batch("INSERT INTO foo VALUES(2)").unwrap();
                sp.commit().unwrap();
            }
            tx.commit().unwrap();
        }
        {
            let tx = db.transaction().unwrap();
            tx.execute_batch("INSERT INTO foo VALUES(4)").unwrap();
            tx.commit().unwrap();
        }
        {
            let tx = db.transaction().unwrap();
            assert_eq!(
                6i32,
                tx.query_row::<i32, _, _>("SELECT SUM(x) FROM foo", NO_PARAMS, |r| r.get(0))
                    .unwrap()
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_savepoint() {
        let mut db = checked_memory_handlex();
        {
            let mut tx = db.transaction().unwrap();
            tx.execute_batch("INSERT INTO foo VALUES(1)").unwrap();
            assert_current_sum(1, &tx);
            tx.set_drop_behavior(DropBehavior::Commit);
            {
                let mut sp1 = tx.savepoint().unwrap();
                sp1.execute_batch("INSERT INTO foo VALUES(2)").unwrap();
                assert_current_sum(3, &sp1);
                // will rollback sp1
                {
                    let mut sp2 = sp1.savepoint().unwrap();
                    sp2.execute_batch("INSERT INTO foo VALUES(4)").unwrap();
                    assert_current_sum(7, &sp2);
                    // will rollback sp2
                    {
                        let sp3 = sp2.savepoint().unwrap();
                        sp3.execute_batch("INSERT INTO foo VALUES(8)").unwrap();
                        assert_current_sum(15, &sp3);
                        sp3.commit().unwrap();
                        // committed sp3, but will be erased by sp2 rollback
                    }
                    assert_current_sum(15, &sp2);
                }
                assert_current_sum(3, &sp1);
            }
            assert_current_sum(1, &tx);
        }
        assert_current_sum(1, &db);
    }

    #[wasm_bindgen_test]
    fn test_ignore_drop_behavior() {
        let mut db = checked_memory_handlex();

        let mut tx = db.transaction().unwrap();
        {
            let mut sp1 = tx.savepoint().unwrap();
            insert(1, &sp1);
            sp1.rollback().unwrap();
            insert(2, &sp1);
            {
                let mut sp2 = sp1.savepoint().unwrap();
                sp2.set_drop_behavior(DropBehavior::Ignore);
                insert(4, &sp2);
            }
            assert_current_sum(6, &sp1);
            sp1.commit().unwrap();
        }
        assert_current_sum(6, &tx);
    }

    #[wasm_bindgen_test]
    fn test_savepoint_names() {
        let mut db = checked_memory_handlex();

        {
            let mut sp1 = db.savepoint_with_name("my_sp").unwrap();
            insert(1, &sp1);
            assert_current_sum(1, &sp1);
            {
                let mut sp2 = sp1.savepoint_with_name("my_sp").unwrap();
                sp2.set_drop_behavior(DropBehavior::Commit);
                insert(2, &sp2);
                assert_current_sum(3, &sp2);
                sp2.rollback().unwrap();
                assert_current_sum(1, &sp2);
                insert(4, &sp2);
            }
            assert_current_sum(5, &sp1);
            sp1.rollback().unwrap();
            {
                let mut sp2 = sp1.savepoint_with_name("my_sp").unwrap();
                sp2.set_drop_behavior(DropBehavior::Ignore);
                insert(8, &sp2);
            }
            assert_current_sum(8, &sp1);
            sp1.commit().unwrap();
        }
        assert_current_sum(8, &db);
    }

    #[wasm_bindgen_test]
    fn test_rc() {
        use std::rc::Rc;
        let mut conn = Connection::open_in_memory().unwrap();
        let rc_txn = Rc::new(conn.transaction().unwrap());

        // This will compile only if Transaction is Debug
        Rc::try_unwrap(rc_txn).unwrap();
    }

    fn insert(x: i32, conn: &Connection) {
        conn.execute("INSERT INTO foo VALUES(?)", &[x]).unwrap();
    }

    fn assert_current_sum(x: i32, conn: &Connection) {
        let i = conn
            .query_row::<i32, _, _>("SELECT SUM(x) FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(x, i);
    }


    fn checked_memory_handlexx() -> Connection {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo (b BLOB, t TEXT, i INTEGER, f FLOAT, n)")
            .unwrap();
        db
    }

    #[wasm_bindgen_test]
    fn test_blobx() {
        let db = checked_memory_handlexx();

        let v1234 = vec![1u8, 2, 3, 4];
        db.execute("INSERT INTO foo(b) VALUES (?)", &[&v1234])
            .unwrap();

        let v: Vec<u8> = db
            .query_row("SELECT b FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(v, v1234);
    }

    #[wasm_bindgen_test]
    fn test_empty_blob() {
        let db = checked_memory_handlexx();

        let empty = vec![];
        db.execute("INSERT INTO foo(b) VALUES (?)", &[&empty])
            .unwrap();

        let v: Vec<u8> = db
            .query_row("SELECT b FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(v, empty);
    }

    #[wasm_bindgen_test]
    fn test_str() {
        let db = checked_memory_handlexx();

        let s = "hello, world!";
        db.execute("INSERT INTO foo(t) VALUES (?)", &[&s]).unwrap();

        let from: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(from, s);
    }

    #[wasm_bindgen_test]
    fn test_string() {
        let db = checked_memory_handlexx();

        let s = "hello, world!";
        db.execute("INSERT INTO foo(t) VALUES (?)", &[s.to_owned()])
            .unwrap();

        let from: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(from, s);
    }

    #[wasm_bindgen_test]
    fn test_value() {
        let db = checked_memory_handlexx();

        db.execute("INSERT INTO foo(i) VALUES (?)", &[Value::Integer(10)])
            .unwrap();

        assert_eq!(
            10i64,
            db.query_row::<i64, _, _>("SELECT i FROM foo", NO_PARAMS, |r| r.get(0))
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_option() {
        let db = checked_memory_handlexx();

        let s = Some("hello, world!");
        let b = Some(vec![1u8, 2, 3, 4]);

        db.execute("INSERT INTO foo(t) VALUES (?)", &[&s]).unwrap();
        db.execute("INSERT INTO foo(b) VALUES (?)", &[&b]).unwrap();

        let mut stmt = db
            .prepare("SELECT t, b FROM foo ORDER BY ROWID ASC")
            .unwrap();
        let mut rows = stmt.query(NO_PARAMS).unwrap();

        {
            let row1 = rows.next().unwrap().unwrap();
            let s1: Option<String> = row1.get_unwrap(0);
            let b1: Option<Vec<u8>> = row1.get_unwrap(1);
            assert_eq!(s.unwrap(), s1.unwrap());
            assert!(b1.is_none());
        }

        {
            let row2 = rows.next().unwrap().unwrap();
            let s2: Option<String> = row2.get_unwrap(0);
            let b2: Option<Vec<u8>> = row2.get_unwrap(1);
            assert!(s2.is_none());
            assert_eq!(b, b2);
        }
    }

    #[wasm_bindgen_test]
    #[allow(clippy::cognitive_complexity)]
    fn test_mismatched_types() {
        fn is_invalid_column_type(err: Error) -> bool {
            match err {
                Error::InvalidColumnType(..) => true,
                _ => false,
            }
        }

        let db = checked_memory_handlexx();

        db.execute(
            "INSERT INTO foo(b, t, i, f) VALUES (X'0102', 'text', 1, 1.5)",
            NO_PARAMS,
        )
            .unwrap();

        let mut stmt = db.prepare("SELECT b, t, i, f, n FROM foo").unwrap();
        let mut rows = stmt.query(NO_PARAMS).unwrap();

        let row = rows.next().unwrap().unwrap();

        // check the correct types come back as expected
        assert_eq!(vec![1, 2], row.get::<_, Vec<u8>>(0).unwrap());
        assert_eq!("text", row.get::<_, String>(1).unwrap());
        assert_eq!(1, row.get::<_, c_int>(2).unwrap());
        assert!((1.5 - row.get::<_, c_double>(3).unwrap()).abs() < EPSILON);
        assert!(row.get::<_, Option<c_int>>(4).unwrap().is_none());
        assert!(row.get::<_, Option<c_double>>(4).unwrap().is_none());
        assert!(row.get::<_, Option<String>>(4).unwrap().is_none());

        // check some invalid types

        // 0 is actually a blob (Vec<u8>)
        assert!(is_invalid_column_type(
            row.get::<_, c_int>(0).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, c_int>(0).err().unwrap()
        ));
        assert!(is_invalid_column_type(row.get::<_, i64>(0).err().unwrap()));
        assert!(is_invalid_column_type(
            row.get::<_, c_double>(0).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, String>(0).err().unwrap()
        ));
        // need to sort out time lib
        /*assert!(is_invalid_column_type(
    row.get::<_, time::Timespec>(0).err().unwrap()
));*/
        assert!(is_invalid_column_type(
            row.get::<_, Option<c_int>>(0).err().unwrap()
        ));

        // 1 is actually a text (String)
        assert!(is_invalid_column_type(
            row.get::<_, c_int>(1).err().unwrap()
        ));
        assert!(is_invalid_column_type(row.get::<_, i64>(1).err().unwrap()));
        assert!(is_invalid_column_type(
            row.get::<_, c_double>(1).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Vec<u8>>(1).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Option<c_int>>(1).err().unwrap()
        ));

        // 2 is actually an integer
        assert!(is_invalid_column_type(
            row.get::<_, String>(2).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Vec<u8>>(2).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Option<String>>(2).err().unwrap()
        ));

        // 3 is actually a float (c_double)
        assert!(is_invalid_column_type(
            row.get::<_, c_int>(3).err().unwrap()
        ));
        assert!(is_invalid_column_type(row.get::<_, i64>(3).err().unwrap()));
        assert!(is_invalid_column_type(
            row.get::<_, String>(3).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Vec<u8>>(3).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Option<c_int>>(3).err().unwrap()
        ));

        // 4 is actually NULL
        assert!(is_invalid_column_type(
            row.get::<_, c_int>(4).err().unwrap()
        ));
        assert!(is_invalid_column_type(row.get::<_, i64>(4).err().unwrap()));
        assert!(is_invalid_column_type(
            row.get::<_, c_double>(4).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, String>(4).err().unwrap()
        ));
        assert!(is_invalid_column_type(
            row.get::<_, Vec<u8>>(4).err().unwrap()
        ));
        // need to sort out time lib
        /*        assert!(is_invalid_column_type(
            row.get::<_, time::Timespec>(4).err().unwrap()
        ));*/
    }

    #[wasm_bindgen_test]
    fn test_dynamic_type() {
        use rusqlite::types::Value;
        let db = checked_memory_handlexx();

        db.execute(
            "INSERT INTO foo(b, t, i, f) VALUES (X'0102', 'text', 1, 1.5)",
            NO_PARAMS,
        )
            .unwrap();

        let mut stmt = db.prepare("SELECT b, t, i, f, n FROM foo").unwrap();
        let mut rows = stmt.query(NO_PARAMS).unwrap();

        let row = rows.next().unwrap().unwrap();
        assert_eq!(Value::Blob(vec![1, 2]), row.get::<_, Value>(0).unwrap());
        assert_eq!(
            Value::Text(String::from("text")),
            row.get::<_, Value>(1).unwrap()
        );
        assert_eq!(Value::Integer(1), row.get::<_, Value>(2).unwrap());
        match row.get::<_, Value>(3).unwrap() {
            Value::Real(val) => assert!((1.5 - val).abs() < EPSILON),
            x => panic!("Invalid Value {:?}", x),
        }
        assert_eq!(Value::Null, row.get::<_, Value>(4).unwrap());
    }

    use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
    use self::url::ParseError;

    fn checked_memory_handlexxx() -> Connection {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo (t TEXT, i INTEGER, f FLOAT, b BLOB)")
            .unwrap();
        db
    }

    #[wasm_bindgen_test]
    fn test_naive_date() {
        let db = checked_memory_handlexxx();
        let date = NaiveDate::from_ymd(2016, 2, 23);
        db.execute("INSERT INTO foo (t) VALUES (?)", &[&date])
            .unwrap();

        let s: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!("2016-02-23", s);
        let t: NaiveDate = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(date, t);
    }

    #[wasm_bindgen_test]
    fn test_naive_time() {
        let db = checked_memory_handlexxx();
        let time = NaiveTime::from_hms(23, 56, 4);
        db.execute("INSERT INTO foo (t) VALUES (?)", &[&time])
            .unwrap();

        let s: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!("23:56:04", s);
        let v: NaiveTime = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(time, v);
    }

    #[wasm_bindgen_test]
    fn test_naive_date_time() {
        let db = checked_memory_handlexxx();
        let date = NaiveDate::from_ymd(2016, 2, 23);
        let time = NaiveTime::from_hms(23, 56, 4);
        let dt = NaiveDateTime::new(date, time);

        db.execute("INSERT INTO foo (t) VALUES (?)", &[&dt])
            .unwrap();

        let s: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!("2016-02-23T23:56:04", s);
        let v: NaiveDateTime = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(dt, v);

        db.execute("UPDATE foo set b = datetime(t)", NO_PARAMS)
            .unwrap(); // "YYYY-MM-DD HH:MM:SS"
        let hms: NaiveDateTime = db
            .query_row("SELECT b FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(dt, hms);
    }

    #[wasm_bindgen_test]
    fn test_date_time_utc() {
        let db = checked_memory_handlexxx();
        let date = NaiveDate::from_ymd(2016, 2, 23);
        let time = NaiveTime::from_hms_milli(23, 56, 4, 789);
        let dt = NaiveDateTime::new(date, time);
        let utc = Utc.from_utc_datetime(&dt);

        db.execute("INSERT INTO foo (t) VALUES (?)", &[&utc])
            .unwrap();

        let s: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!("2016-02-23T23:56:04.789+00:00", s);

        let v1: DateTime<Utc> = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(utc, v1);

        let v2: DateTime<Utc> = db
            .query_row("SELECT '2016-02-23 23:56:04.789'", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(utc, v2);

        let v3: DateTime<Utc> = db
            .query_row("SELECT '2016-02-23 23:56:04'", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(utc - chrono::Duration::milliseconds(789), v3);

        let v4: DateTime<Utc> = db
            .query_row("SELECT '2016-02-23 23:56:04.789+00:00'", NO_PARAMS, |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(utc, v4);
    }

    #[wasm_bindgen_test]
    fn test_date_time_local() {
        let db = checked_memory_handlexxx();
        let date = NaiveDate::from_ymd(2016, 2, 23);
        let time = NaiveTime::from_hms_milli(23, 56, 4, 789);
        let dt = NaiveDateTime::new(date, time);
        let local = Local.from_local_datetime(&dt).single().unwrap();

        db.execute("INSERT INTO foo (t) VALUES (?)", &[&local])
            .unwrap();

        // Stored string should be in UTC
        let s: String = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert!(s.ends_with("+00:00"));

        let v: DateTime<Local> = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(local, v);
    }

    #[wasm_bindgen_test]
    fn test_sqlite_functions() {
        let db = checked_memory_handlexxx();
        let result: Result<NaiveTime> =
            db.query_row("SELECT CURRENT_TIME", NO_PARAMS, |r| r.get(0));
        assert!(result.is_ok());
        let result: Result<NaiveDate> =
            db.query_row("SELECT CURRENT_DATE", NO_PARAMS, |r| r.get(0));
        assert!(result.is_ok());
        let result: Result<NaiveDateTime> =
            db.query_row("SELECT CURRENT_TIMESTAMP", NO_PARAMS, |r| r.get(0));
        assert!(result.is_ok());
        let result: Result<DateTime<Utc>> =
            db.query_row("SELECT CURRENT_TIMESTAMP", NO_PARAMS, |r| r.get(0));
        assert!(result.is_ok());
    }


    fn checked_memory_handle_from_sql() -> Connection {
        Connection::open_in_memory().unwrap()
    }

    #[wasm_bindgen_test]
    fn test_integral_ranges() {
        let db = checked_memory_handle_from_sql();

        fn check_ranges<T>(db: &Connection, out_of_range: &[i64], in_range: &[i64])
            where
                T: Into<i64> + FromSql + ::std::fmt::Debug,
        {
            for n in out_of_range {
                let err = db
                    .query_row("SELECT ?", &[n], |r| r.get::<_, T>(0))
                    .unwrap_err();
                match err {
                    Error::IntegralValueOutOfRange(_, value) => assert_eq!(*n, value),
                    _ => panic!("unexpected error: {}", err),
                }
            }
            for n in in_range {
                assert_eq!(
                    *n,
                    db.query_row("SELECT ?", &[n], |r| r.get::<_, T>(0))
                        .unwrap()
                        .into()
                );
            }
        }

        check_ranges::<i8>(&db, &[-129, 128], &[-128, 0, 1, 127]);
        check_ranges::<i16>(&db, &[-32769, 32768], &[-32768, -1, 0, 1, 32767]);
        check_ranges::<i32>(
            &db,
            &[-2_147_483_649, 2_147_483_648],
            &[-2_147_483_648, -1, 0, 1, 2_147_483_647],
        );
        check_ranges::<u8>(&db, &[-2, -1, 256], &[0, 1, 255]);
        check_ranges::<u16>(&db, &[-2, -1, 65536], &[0, 1, 65535]);
        check_ranges::<u32>(&db, &[-2, -1, 4_294_967_296], &[0, 1, 4_294_967_295]);
    }

    extern crate serde_json;

    fn checked_memory_handle_serde_json() -> Connection {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo (t TEXT, b BLOB)")
            .unwrap();
        db
    }

    #[wasm_bindgen_test]
    fn test_json_value() {
        let db = checked_memory_handle_serde_json();

        let json = r#"{"foo": 13, "bar": "baz"}"#;
        let data: serde_json::Value = serde_json::from_str(json).unwrap();
        db.execute(
            "INSERT INTO foo (t, b) VALUES (?, ?)",
            &[&data as &dyn ToSql, &json.as_bytes()],
        )
            .unwrap();

        let t: serde_json::Value = db
            .query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(data, t);
        let b: serde_json::Value = db
            .query_row("SELECT b FROM foo", NO_PARAMS, |r| r.get(0))
            .unwrap();
        assert_eq!(data, b);
    }


    fn is_to_sql<T: ToSql>() {}

    #[wasm_bindgen_test]
    fn test_integral_types() {
        is_to_sql::<i8>();
        is_to_sql::<i16>();
        is_to_sql::<i32>();
        is_to_sql::<i64>();
        is_to_sql::<u8>();
        is_to_sql::<u16>();
        is_to_sql::<u32>();
    }

    #[wasm_bindgen_test]
    fn test_cow_str() {
        use std::borrow::Cow;
        let s = "str";
        let cow = Cow::Borrowed(s);
        let r = cow.to_sql();
        assert!(r.is_ok());
        let cow = Cow::Owned::<str>(String::from(s));
        let r = cow.to_sql();
        assert!(r.is_ok());
    }

    #[cfg(feature = "i128_blob")]
    #[wasm_bindgen_test]
    fn test_i128() {
        use crate::{Connection, NO_PARAMS};
        use std::i128;
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo (i128 BLOB, desc TEXT)")
            .unwrap();
        db.execute(
            "
            INSERT INTO foo(i128, desc) VALUES
                (?, 'zero'),
                (?, 'neg one'), (?, 'neg two'),
                (?, 'pos one'), (?, 'pos two'),
                (?, 'min'), (?, 'max')",
            &[0i128, -1i128, -2i128, 1i128, 2i128, i128::MIN, i128::MAX],
        )
            .unwrap();

        let mut stmt = db
            .prepare("SELECT i128, desc FROM foo ORDER BY i128 ASC")
            .unwrap();

        let res = stmt
            .query_map(NO_PARAMS, |row| {
                Ok((row.get::<_, i128>(0)?, row.get::<_, String>(1)?))
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(
            res,
            &[
                (i128::MIN, "min".to_owned()),
                (-2, "neg two".to_owned()),
                (-1, "neg one".to_owned()),
                (0, "zero".to_owned()),
                (1, "pos one".to_owned()),
                (2, "pos two".to_owned()),
                (i128::MAX, "max".to_owned()),
            ]
        );
    }

    #[cfg(feature = "uuid")]
    #[wasm_bindgen_test]
    fn test_uuid() {
        use crate::{params, Connection};
        use uuid::Uuid;

        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE foo (id BLOB CHECK(length(id) = 16), label TEXT);")
            .unwrap();

        let id = Uuid::new_v4();

        db.execute(
            "INSERT INTO foo (id, label) VALUES (?, ?)",
            params![id, "target"],
        )
            .unwrap();

        let mut stmt = db
            .prepare("SELECT id, label FROM foo WHERE id = ?")
            .unwrap();

        let mut rows = stmt.query(params![id]).unwrap();
        let row = rows.next().unwrap().unwrap();

        let found_id: Uuid = row.get_unwrap(0);
        let found_label: String = row.get_unwrap(1);

        assert_eq!(found_id, id);
        assert_eq!(found_label, "target");
    }

    extern crate url;

    fn checked_memory_handle_url() -> Connection {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch("CREATE TABLE urls (i INTEGER, v TEXT)")
            .unwrap();
        db
    }

    fn get_url(db: &Connection, id: i64) -> Result<url::Url> {
        db.query_row("SELECT v FROM urls WHERE i = ?", params![id], |r| r.get(0))
    }

    #[wasm_bindgen_test]
    fn test_sql_url() {
        let db = &checked_memory_handle_url();

        let url0 = url::Url::parse("http://www.example1.com").unwrap();
        let url1 = url::Url::parse("http://www.example1.com/👌").unwrap();
        let url2 = "http://www.example2.com/👌";

        db.execute(
            "INSERT INTO urls (i, v) VALUES (0, ?), (1, ?), (2, ?), (3, ?)",
            // also insert a non-hex encoded url (which might be present if it was
            // inserted separately)
            params![url0, url1, url2, "illegal"],
        )
            .unwrap();

        assert_eq!(get_url(db, 0).unwrap(), url0);

        assert_eq!(get_url(db, 1).unwrap(), url1);

        // Should successfully read it, even though it wasn't inserted as an
        // escaped url.
        let out_url2: url::Url = get_url(db, 2).unwrap();
        assert_eq!(out_url2, url::Url::parse(url2).unwrap());

        // Make sure the conversion error comes through correctly.
        let err = get_url(db, 3).unwrap_err();
        match err {
            Error::FromSqlConversionFailure(_, _, e) => {
                assert_eq!(
                    *e.downcast::<ParseError>().unwrap(),
                    ParseError::RelativeUrlWithoutBase,
                );
            }
            e => {
                panic!("Expected conversion failure, got {}", e);
            }
        }
    }

    use rusqlite::vtab::{dequote, parse_boolean, array};
    use std::rc::Rc;

    #[wasm_bindgen_test]
    fn test_dequote() {
        assert_eq!("", dequote(""));
        assert_eq!("'", dequote("'"));
        assert_eq!("\"", dequote("\""));
        assert_eq!("'\"", dequote("'\""));
        assert_eq!("", dequote("''"));
        assert_eq!("", dequote("\"\""));
        assert_eq!("x", dequote("'x'"));
        assert_eq!("x", dequote("\"x\""));
        assert_eq!("x", dequote("x"));
    }

    #[wasm_bindgen_test]
    fn test_parse_boolean() {
        assert_eq!(None, parse_boolean(""));
        assert_eq!(Some(true), parse_boolean("1"));
        assert_eq!(Some(true), parse_boolean("yes"));
        assert_eq!(Some(true), parse_boolean("on"));
        assert_eq!(Some(true), parse_boolean("true"));
        assert_eq!(Some(false), parse_boolean("0"));
        assert_eq!(Some(false), parse_boolean("no"));
        assert_eq!(Some(false), parse_boolean("off"));
        assert_eq!(Some(false), parse_boolean("false"));
    }


    #[wasm_bindgen_test]
    fn test_array_module() {
        let db = Connection::open_in_memory().unwrap();
        array::load_module(&db).unwrap();

        let v = vec![1i64, 2, 3, 4];
        let values = v.into_iter().map(Value::from).collect();
        let ptr = Rc::new(values);
        {
            let mut stmt = db.prepare("SELECT value from rarray(?);").unwrap();

            let rows = stmt.query_map(&[&ptr], |row| row.get::<_, i64>(0)).unwrap();
            assert_eq!(2, Rc::strong_count(&ptr));
            let mut count = 0;
            for (i, value) in rows.enumerate() {
                assert_eq!(i as i64, value.unwrap() - 1);
                count += 1;
            }
            assert_eq!(4, count);
        }
        assert_eq!(1, Rc::strong_count(&ptr));
    }
/*
    use rusqlite::vtab::csvtab;

    #[wasm_bindgen_test]
    #[ignore]
    fn test_csv_module() {
        let db = Connection::open_in_memory().unwrap();
        csvtab::load_module(&db).unwrap();
        db.execute_batch("CREATE VIRTUAL TABLE vtab USING csv(filename='test.csv', header=yes)")
            .unwrap();

        {
            let mut s = db.prepare("SELECT rowid, * FROM vtab").unwrap();
            {
                let headers = s.column_names();
                assert_eq!(vec!["rowid", "colA", "colB", "colC"], headers);
            }

            let ids: Result<Vec<i32>> = s
                .query(NO_PARAMS)
                .unwrap()
                .map(|row| row.get::<_, i32>(0))
                .collect();
            let sum = ids.unwrap().iter().sum::<i32>();
            assert_eq!(sum, 15);
        }
        db.execute_batch("DROP TABLE vtab").unwrap();
    }

    #[wasm_bindgen_test]
    #[ignore]
    fn test_csv_cursor() {
        let db = Connection::open_in_memory().unwrap();
        csvtab::load_module(&db).unwrap();
        db.execute_batch("CREATE VIRTUAL TABLE vtab USING csv(filename='test.csv', header=yes)")
            .unwrap();

        {
            let mut s = db
                .prepare(
                    "SELECT v1.rowid, v1.* FROM vtab v1 NATURAL JOIN vtab v2 WHERE \
                     v1.rowid < v2.rowid",
                )
                .unwrap();

            let mut rows = s.query(NO_PARAMS).unwrap();
            let row = rows.next().unwrap().unwrap();
            assert_eq!(row.get_unwrap::<_, i32>(0), 2);
        }
        db.execute_batch("DROP TABLE vtab").unwrap();
    }

*/
    use rusqlite::vtab::series;

    #[wasm_bindgen_test]
    fn test_series_module() {
        let version = unsafe { ffi::sqlite3_libversion_number() };
        if version < 3_008_012 {
            return;
        }

        let db = Connection::open_in_memory().unwrap();
        series::load_module(&db).unwrap();

        let mut s = db.prepare("SELECT * FROM generate_series(0,20,5)").unwrap();

        let series = s.query_map(NO_PARAMS, |row| row.get::<_, i32>(0)).unwrap();

        let mut expected = 0;
        for value in series {
            assert_eq!(expected, value.unwrap());
            expected += 5;
        }
    }
}
