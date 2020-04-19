use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut cfg = cc::Build::new();
    cfg.file("sqlite3/sqlite3.c")
        .include("/musl-sysroot/include")
        .flag("--sysroot=/musl-sysroot")
        .flag("-nostdinc")
        .flag("--static")
        .flag("--target=wasm32-unknown-unknown")
        //.flag("-Wl,--import-memory")
        .flag("-Wno-unused-command-line-argument")
        .flag("-Wno-bitwise-op-parentheses")
        .flag("-Wno-shift-op-parentheses")
        .flag("-emit-llvm")
        .flag("-DLONGDOUBLE_TYPE=double")
        .flag("-DSQLITE_OMIT_LOAD_EXTENSION")
        .flag("-DSQLITE_DISABLE_LFS")
        .flag("-DSQLITE_ENABLE_FTS5")
        .flag("-DSQLITE_ENABLE_FTS5_PARENTHESIS")
        .flag("-DSQLITE_THREADSAFE=0");

    // Older versions of visual studio don't support c99 (including isnan), which
    // causes a build failure when the linker fails to find the `isnan`
    // function. `sqlite` provides its own implmentation, using the fact
    // that x != x when x is NaN.
    //
    // There may be other platforms that don't support `isnan`, they should be
    // tested for here.

    cfg.flag("-DSQLITE_HAVE_ISNAN");

    if cfg!(feature = "unlock_notify") {
        cfg.flag("-DSQLITE_ENABLE_UNLOCK_NOTIFY");
    }
    if cfg!(feature = "preupdate_hook") {
        cfg.flag("-DSQLITE_ENABLE_PREUPDATE_HOOK");
    }
    if cfg!(feature = "session") {
        cfg.flag("-DSQLITE_ENABLE_SESSION");
    }

    if let Ok(limit) = env::var("SQLITE_MAX_VARIABLE_NUMBER") {
        cfg.flag(&format!("-DSQLITE_MAX_VARIABLE_NUMBER={}", limit));
    }
    println!("cargo:rerun-if-env-changed=SQLITE_MAX_VARIABLE_NUMBER");

    if let Ok(limit) = env::var("SQLITE_MAX_EXPR_DEPTH") {
        cfg.flag(&format!("-DSQLITE_MAX_EXPR_DEPTH={}", limit));
    }
    println!("cargo:rerun-if-env-changed=SQLITE_MAX_EXPR_DEPTH");

    cfg.compile("sqlite3");
    println!("cargo:rustc-link-search=native={}", out_dir);

    Command::new("/clang/bin/llvm-ar").arg("rcv").arg("libsqlite3.a").arg("sqlite3.o").output().expect("Failed to execute command");
}


fn env_prefix() -> &'static str {
    "SQLITE3"
}

pub enum HeaderLocation {
    FromEnvironment,
    Wrapper,
    FromPath(String),
}

impl From<HeaderLocation> for String {
    fn from(header: HeaderLocation) -> String {
        match header {
            HeaderLocation::FromEnvironment => {
                let prefix = env_prefix();
                let mut header = env::var(format!("{}_INCLUDE_DIR", prefix)).unwrap_or_else(|_| {
                    panic!(
                        "{}_INCLUDE_DIR must be set if {}_LIB_DIR is set",
                        prefix, prefix
                    )
                });
                header.push_str("/sqlite3.h");
                header
            }
            HeaderLocation::Wrapper => "wrapper.h".into(),
            HeaderLocation::FromPath(path) => path,
        }
    }
}


#[derive(Debug)]
struct SqliteTypeChooser;
