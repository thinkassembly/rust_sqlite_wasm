#![feature(set_stdio)]

use std::fmt;
use std::fmt::Write;
use std::panic;
use std::io;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;


fn trace(buf: &str){
    web_sys::console::trace_1(&JsValue::from(buf));

}

fn print(buf: &str) {
    web_sys::console::info_1(&JsValue::from(buf));

}

fn eprint(buf: &str) {
    web_sys::console::warn_1(&JsValue::from(buf));

}

fn _print(buf: &str) -> io::Result<()> {
 //   let cstring = CString::new(buf)?;

        print(buf);


    Ok(())
}

fn _eprint(buf: &str) -> io::Result<()> {
    //let cstring = CString::new(buf)?;


        eprint(buf);


    Ok(())
}

/// Used by the `print` macro
#[doc(hidden)]
pub fn _print_args(args: fmt::Arguments) {
    let mut buf = String::new();
    let _ = buf.write_fmt(args);
    let _ = _print(&buf);
}

/// Used by the `eprint` macro
#[doc(hidden)]
pub fn _eprint_args(args: fmt::Arguments) {
    let mut buf = String::new();
    let _ = buf.write_fmt(args);
    let _ = _eprint(&buf);
}

/// Overrides the default `print!` macro.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print_args(format_args!($($arg)*)));
}

/// Overrides the default `eprint!` macro.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::_eprint_args(format_args!($($arg)*)));
}


type PrintFn = fn(&str) -> io::Result<()>;

struct Printer {
    printfn: PrintFn,
    buffer: String,
    is_buffered: bool,
}

impl Printer {
    fn new(printfn: PrintFn, is_buffered: bool) -> Printer {
        Printer {
            buffer: String::new(),
            printfn,
            is_buffered,
        }
    }
}

impl io::Write for Printer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.push_str(&String::from_utf8_lossy(buf));

        if !self.is_buffered {
            (self.printfn)(&self.buffer)?;
            self.buffer.clear();

            return Ok(buf.len());
        }

        if let Some(i) = self.buffer.rfind('\n') {
            let buffered = {
                let (first, last) = self.buffer.split_at(i);
                (self.printfn)(first)?;

                String::from(&last[1..])
            };

            self.buffer.clear();
            self.buffer.push_str(&buffered);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        (self.printfn)(&self.buffer)?;
        self.buffer.clear();

        Ok(())
    }
}


/// Sets a line-buffered stdout, uses your JavaScript `print` function
pub fn set_stdout() {
    let printer = Printer::new(_print, true);
    io::set_print(Some(Box::new(printer)));
}

/// Sets an unbuffered stdout, uses your JavaScript `print` function
pub fn set_stdout_unbuffered() {
    let printer = Printer::new(_print, false);
    io::set_print(Some(Box::new(printer)));
}

/// Sets a line-buffered stderr, uses your JavaScript `eprint` function
pub fn set_stderr() {
    let eprinter = Printer::new(_eprint, true);
    io::set_panic(Some(Box::new(eprinter)));
}

/// Sets an unbuffered stderr, uses your JavaScript `eprint` function
pub fn set_stderr_unbuffered() {
    let eprinter = Printer::new(_eprint, false);
    io::set_panic(Some(Box::new(eprinter)));
}

/// Sets a custom panic hook, uses your JavaScript `trace` function
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            }
        };
            trace(&format!("Panicked at '{}', {}:{}:{}", msg, file, line, col));

    }));
}

/// Sets stdout, stderr, and a custom panic hook
pub fn hook() {
    set_stdout();
    set_stderr();
    set_panic_hook();
}
