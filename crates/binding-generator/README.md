# binding-generator

Parses C++ header files in the OCCT src directory and writes out Rust bindings for the parsed C++ types.

## Run the example

```
cargo run --release --example simple_binding -- ../occt-sys/OCCT/src StlAPI
```
