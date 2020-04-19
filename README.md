This repo is a POC using clang to compile sqlite and statically link with rust targeting wasm32-unknown-unknown and accessing it using the rusqlite crate interface.  

I borrowed heavily from several projects to get this working. 
* https://github.com/jfbastien/musl
* https://github.com/rusqlite/rusqlite
* https://github.com/DeMille/wasm-glue


**Goals**
* Use sqlite as an in-memory sql database from rust/wasm.
* No emscripten dependency.
* No manually written javascript shims. (let wasm-bindgen, web-sys, js-sys handle those bits)

**Things I don't care about.**
* Supporting all features of sqlite. Just the subset that makes sense in a browser based environment.
* A javascript API. 
* The libc implementation, I only care about the minimum needed to get this working. 
* Continued rusqlite compatibility, a lot of features don't make sense in the browser environment, I doubt any
of my modifications make sense to upstream.

**Building**

You will need the docker image from https://github.com/nullrocket/wasm-compiler 

```
docker pull docker.pkg.github.com/nullrocket/wasm-compiler/wasm_compiler:9.0.1
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