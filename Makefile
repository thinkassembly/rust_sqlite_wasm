DIST_BROWSER := dist-browser
OUTFILES_BROWSER := foo_bg.wasm foo.js foo.d.ts
OUTPATHS_BROWSER := $(foreach file,$(OUTFILES_BROWSER),$(DIST_BROWSER)/$(file))
DIST_NODEJS := dist-nodejs
OUTFILES_NODEJS := $(OUTFILES_BROWSER) foo_bg.js
OUTPATHS_NODEJS := $(foreach file,$(OUTFILES_NODEJS),$(DIST_NODEJS)/$(file))
DOCKER_IMAGE_VERSION := 9.0.1
DOCKER_IMAGE := wasm_compiler:$(DOCKER_IMAGE_VERSION)
export C_LIB_DIR := /musl-sysroot/lib
DOCKER_RUN = sudo docker run \
  --user $(shell id -u):$(shell id -g) \
   --volume $(CURDIR):/c:rw \
  --volume $(CURDIR)/target:/c/target \
  --volume $(HOME)/.cargo:/cargo \
  --volume $(shell rustc +nightly --print sysroot):/rust:ro \
  --env CARGO_HOME=/cargo \
  --env PATH=/rust/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
  --env C_LIB_DIR=$(C_LIB_DIR) \
  --env LLVM_CONFIG_PATH=/clang/bin/llvm-config \
  --workdir /c \
  --interactive \
  --tty \
  --rm \
  $(DOCKER_IMAGE)

all: browser node

browser: target/wasm32-unknown-unknown/release/wasm_sqlite_demo.wasm
	mkdir -p $(DIST_BROWSER)
	wasm-bindgen target/wasm32-unknown-unknown/release/wasm_sqlite_demo.wasm --out-dir $(DIST_BROWSER) --browser

node: target/wasm32-unknown-unknown/release/wasm_sqlite_demo.wasm
	mkdir -p $(DIST_NODEJS)
	wasm-bindgen target/wasm32-unknown-unknown/release/wasm_sqlite_demo.wasm --out-dir $(DIST_NODEJS) --nodejs

target/wasm32-unknown-unknown/release/wasm_sqlite_demo.wasm: src/* libs/libc-sys
	mkdir -p target
	$(DOCKER_RUN)  /bin/bash -c "/clang/bin/llvm-ar d /musl-sysroot/lib/libc.a memcpy.o memmove.o memset.o expf.o memcmp.o   && cargo build --target=wasm32-unknown-unknown --release"

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
