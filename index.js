const js = import("./dist-browser/wasm_sqlite_demo");

js.then(js => {
  js.start();

});
