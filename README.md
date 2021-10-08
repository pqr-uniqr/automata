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

file structure

* / 
    * `main.go` - server entrypoint. 
    * static
        * `gorust-wasm-test.html` - sole html file, imports `main.wasm`, go wasm
          target (for qr code gen integration), and `rust-wasm.js`, javascript
          entrypoint for rust-wasm.
        * `go_wasm_shim.js` defines the APIs for operating go-generated wasm
          code (originally `wasm_exec.js`, found in standard go installations).

> ! go wasm doesn't do anything right now.

[article about go-wasm integration.](http://macias.info/entry/202003151900_go_wasm_js.md)

> don't we need these as dependencies in the package management system? whatever.

> shit why does the wasm.js file contain the Destroyer thing. isn't this a
> library? chill we can move it out.


https://github.com/naomiaro/waveform-playlist
