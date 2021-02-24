import("./pkg/wasm.js").then((js) => {
    js.default().then((_) => {
        js.greet("WebAssembly");
    })
});
