DIST_BROWSER := dist-browser
OUTFILES_BROWSER := rust_sqlite_wasm_bg.wasm rust_sqlite_wasm.js rust_sqlite_wasm_bg.d.ts
OUTPATHS_BROWSER := $(foreach file,$(OUTFILES_BROWSER),$(DIST_BROWSER)/$(file))
DIST_NODEJS := dist-nodejs
OUTFILES_NODEJS := $(OUTFILES_BROWSER) rust_sqlite_wasm.js
OUTPATHS_NODEJS := $(foreach file,$(OUTFILES_NODEJS),$(DIST_NODEJS)/$(file))
DOCKER_IMAGE_VERSION := 9.0.2
DOCKER_IMAGE := wasm_compiler:$(DOCKER_IMAGE_VERSION)
export C_LIB_DIR := /musl-sysroot/lib

DOCKER_RUN = docker run \
  --user $(shell id -u):$(shell id -g) \
   --volume $(CURDIR):/c:rw \
  --volume $(CURDIR)/target:/c/target:rw \
  --env PATH=/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/.cargo/bin:/usr/local/nvm/versions/node/v13.13.0/bin \
  --env C_LIB_DIR=$(C_LIB_DIR) \
  --env RUSTUP_HOME=/.rust \
  --env CARGO_HOME=/.cargo \
  --env LLVM_CONFIG_PATH=/clang/bin/llvm-config \
  --workdir /c \
  --interactive \
  --tty \
  --rm \
  $(DOCKER_IMAGE)
all: browser-release node-release

node-debug: src/* libs/libc-sys
	mkdir -p target
	$(DOCKER_RUN)  /bin/bash -c "/clang/bin/llvm-ar d /musl-sysroot/lib/libc.a memcpy.o memmove.o memset.o expf.o memcmp.o fmax.o fmin.o && /.cargo/bin/wasm-pack build --target nodejs  --out-dir dist-nodejs"
	$(DOCKER_RUN)  /bin/bash -c "wasm-opt /c/dist-nodejs/rust_sqlite_wasm_bg.wasm -o /c/dist-nodejs/rust_sqlite_wasm_bg.wasm"

browser-debug: src/* libs/libc-sys
	mkdir -p target
	$(DOCKER_RUN)  /bin/bash -c "/clang/bin/llvm-ar d /musl-sysroot/lib/libc.a memcpy.o memmove.o memset.o expf.o memcmp.o fmax.o fmin.o && /.cargo/bin/wasm-pack build --target browser  --out-dir dist-browser"
	$(DOCKER_RUN)  /bin/bash -c "wasm-opt /c/dist-browser/rust_sqlite_wasm_bg.wasm -o /c/dist-browser/rust_sqlite_wasm_bg.wasm"

node-release: src/* libs/libc-sys
	mkdir -p target
	$(DOCKER_RUN)  /bin/bash -c "/clang/bin/llvm-ar d /musl-sysroot/lib/libc.a memcpy.o memmove.o memset.o expf.o memcmp.o fmax.o fmin.o && /.cargo/bin/wasm-pack build --target nodejs --release --out-dir dist-nodejs"
	$(DOCKER_RUN)  /bin/bash -c "wasm-opt /c/dist-nodejs/rust_sqlite_wasm_bg.wasm -o /c/dist-nodejs/rust_sqlite_wasm_bg.wasm"

browser-release: src/* libs/libc-sys
	mkdir -p target
	$(DOCKER_RUN)  /bin/bash -c "/clang/bin/llvm-ar d /musl-sysroot/lib/libc.a memcpy.o memmove.o memset.o expf.o memcmp.o fmax.o fmin.o && /.cargo/bin/wasm-pack build --target browser --release --out-dir dist-browser"
	$(DOCKER_RUN)  /bin/bash -c "wasm-opt /c/dist-browser/rust_sqlite_wasm_bg.wasm -o /c/dist-browser/rust_sqlite_wasm_bg.wasm"


node-test: src/* libs/libc-sys
	mkdir -p target
	$(DOCKER_RUN)  /bin/bash -c "/clang/bin/llvm-ar d /musl-sysroot/lib/libc.a memcpy.o memmove.o memset.o expf.o memcmp.o fmax.o fmin.o && /.cargo/bin/wasm-pack test  --node -- --lib "

clean:
	cargo clean
	rm -rf $(DIST_BROWSER) $(DIST_NODEJS)

blankslate: clean
	rm -f Cargo.lock
	rm -rf node_modules
	rm -f package-lock.json

env:
	npm install

test: $(OUTPATHS_NODEJS)
	node run.js


.PHONY: all clean env blankslate test
