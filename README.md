This repo is a POC using clang to compile sqlite to Webassembly using clang and accessing it using the rusqlite interface.  

I borrowed heavily from several projects to get this working. 
* https://github.com/jfbastien/musl
* https://github.com/rusqlite/rusqlite

Goals
* Use sqlite as an in-memory sql database from rust/wasm.
* No emscripten dependency.
* No manually written javascript shims. (let wasm-bindgen, web-sys, js-sys handle those bits)

Things I don't care about.
* All features of sqlite. Just the subset I need and that make sense in the browser environment.
* A javascript api.
* The libc implementation, I hacked around a bit to get just the minimum I needed.
The project it is based on https://github.com/jfbastien/musl is archived and no longer maintained.
* Rusqlite compatibility, a lot of features don't make sense in the browser environment, I doubt any
of my modifications make sense to upstream.

 

