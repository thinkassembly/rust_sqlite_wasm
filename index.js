const js = import("./dist-browser/rust_sqlite_wasm");

js.then(js => {
  js.start();

});
