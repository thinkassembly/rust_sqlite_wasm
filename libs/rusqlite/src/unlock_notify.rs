//! [Unlock Notification](http://sqlite.org/unlock_notify.html)

use std::os::raw::c_int;


use crate::ffi;


#[cfg(not(feature = "unlock_notify"))]
pub fn is_locked(_db: *mut ffi::sqlite3, _rc: c_int) -> bool {
    unreachable!()
}

#[cfg(not(feature = "unlock_notify"))]
pub fn wait_for_unlock_notify(_db: *mut ffi::sqlite3) -> c_int {
    unreachable!()
}
