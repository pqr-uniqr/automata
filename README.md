# What?
Server for web audio installations.

# Stack
Go web server, and the html client runs wasm code compiled from both rust and
go, plus obviously glue javascript. There's react integration as well. 

The go-wasm code will enable reusing of qr code generation code, rust-wasm will
be used to conduct audio activities by talking to Web Audio APIs using
`web-sys` bindings.

# Build instructions
Place this repository at the go source directory. For example:
`/Users/pqhwan/go/src/pqr`

`./make build` to build, `./make clean` to clean.


* http://localhost:8080/.info prints go info. 
* http://localhost:8080/main.html or http://localhost:8080/qr/draw for qr code
generator page.

# gorust-wasm-test.html

http://localhost:8080/gorust-wasm-test.html 

picking this back up after a while, trying to figure out how it works.

`main.go` is the entry point into the server. Sets up various endpoints. it
serves `gosrust-wasm-test.html`, which is where eveyrthing is.  

the html's got a body with html elements and head with some javascript. it does
some unfathomable things to check for webassembly support, and then starts it by
feeding it the `main.wasm` file. Well it tells Go to run it, as well. what's the
`Go`?

i think it comes from the `go_wasm_shim.js`. that's a piece of work, too. it was
originally called `wasm_exec.js` and can be found in Go installations. i don't
know what role it plays exactly though. i can find out.

`main.wasm` is a compiled version of what's inside the `go-wasm/` directory.
oh. it just does `log.console`.

[article about go-wasm integration.](http://macias.info/entry/202003151900_go_wasm_js.md)

and of course we bring in a rust-wasm.js script, which has my code with the
worst naming ever. but it imports a wasm.js file, which i think is the rust wasm
module.

don't we need these as dependencies in the package management system? whatever.

shit why does the wasm.js file contain the Destroyer thing. isn't this a
library? chill we can move it out.

so finally we arrive at the rust file, `lib.rs`. 

## javascript debugger
