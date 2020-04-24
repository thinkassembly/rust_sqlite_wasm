//! Run-Time Limits

use std::os::raw::c_int;

use crate::ffi;
pub use crate::ffi::Limit;

use crate::Connection;

impl Connection {
    /// Returns the current value of a limit.
    pub fn limit(&self, limit: Limit) -> i32 {
        let c = self.db.borrow();
        unsafe { ffi::sqlite3_limit(c.db(), limit as c_int, -1) }
    }

    /// Changes the limit to `new_val`, returning the prior value of the limit.
    pub fn set_limit(&self, limit: Limit, new_val: i32) -> i32 {
        let c = self.db.borrow_mut();
        unsafe { ffi::sqlite3_limit(c.db(), limit as c_int, new_val) }
    }
}
