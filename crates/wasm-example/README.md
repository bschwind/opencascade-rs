# wasm-example

An example of model code which can be compiled to WASM and executed in the viewer app.

Use [cargo watch](https://crates.io/crates/cargo-watch) to listen for file changes and rebuild the model file whenever code is edited:

```
$ cargo watch -x "build -p wasm-example --release --target wasm32-unknown-unknown"
```

Run the viewer app with:

```
$ cargo run --release --no-default-features --bin viewer
```
