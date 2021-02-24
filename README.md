# What?
Server for web audio installations.

# Stack
Go web server, and the html client runs wasm code compiled from both rust and
go, plus obviously glue javascript. There's react integration as well. 

The go-wasm code will enable reusing of qr code generation code, rust-wasm will
be used to conduct audio activities by talking to Web Audio APIs using
`web-sys` bindings.

# Build instructions
`./make build` to build, `./make clean` to clean.
