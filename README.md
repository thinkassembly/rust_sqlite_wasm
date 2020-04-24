This repo is a POC using clang to compile sqlite and statically link with rust targeting wasm32-unknown-unknown and accessing it using the rusqlite crate interface.

This branch of the repo is based on an unmodified copy of sqlite3 sources and more extensive modifications of rusqlite sources to enable testing.

Unfortunately I couldn't figure out how to get web-pack tests to correctly compile and pass through link options to tests in sub crates, so I moved tests into
the top level crate I'm using to test drive things. It's not ideal but I wanted to move on. I'll come back to this later.
  

I borrowed heavily from several projects to get this working. 
* https://github.com/wehlutyk/wasm-explorations
* https://github.com/jfbastien/musl
* https://github.com/rusqlite/rusqlite
* https://github.com/DeMille/wasm-glue


**Goals**
* Use sqlite as an in-memory sql database from rust/wasm.
* No emscripten dependency.
* No manually written javascript shims. Let wasm-bindgen, web-sys, js-sys handle those bits.


**Things I don't care about.**
* Supporting all features of sqlite. Just the subset that makes sense in a browser based environment.
* A javascript API. 
* The libc implementation, I only care about the minimum needed to get this working. 
* Continued rusqlite compatibility, a lot of features don't make sense in the browser environment, I doubt any
of my modifications make sense to upstream.


**Next up**
* Move tests back into their proper places.
* Figure out wasm-pack & hot reloading with the make+docker build. 
* The current make+docker build isn't very fun for fast iteration during development and can't be built using cargo alone.
Not sure yet what the solution is here. 
* This POC was all work in preparation for rewriting some of sqlite to rust. This may or may not happen, but it is fun to hack on.

**Build**

You will need the docker image from https://github.com/nullrocket/wasm-compiler 

```
docker pull docker.pkg.github.com/nullrocket/wasm-compiler/wasm_compiler:9.0.2
```
To build:
```
make env
make
```
To run in node:
```
node run
```

To run in the browser:
```
npm run serve
```