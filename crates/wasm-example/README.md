# wasm-example

An example of model code which can be compiled to WASM and executed in the viewer app.

Use [cargo watch](https://crates.io/crates/cargo-watch) to listen for file changes and rebuild the model file whenever code is edited:

```
# From the project root
$ cargo watch -x "build -p wasm-example --release --target wasm32-unknown-unknown"
```

Run the viewer app with:

```
# From the project root, while the "cargo watch" command above is running.
$ cargo run --release --bin viewer -- --wasm-path target/wasm32-unknown-unknown/release/wasm_example.wasm
```

Edit `src/lib.rs` and save to see the model code get recompiled, and see the model in the viewer app get updated.
