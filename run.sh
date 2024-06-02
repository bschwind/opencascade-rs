cd crates/wasm-example
cargo build -p wasm-example --release --target wasm32-unknown-unknown
cd ../..
wasm-tools component new ./target/wasm32-unknown-unknown/release/wasm_example.wasm -o my-component.wasm
cargo run --release --no-default-features --bin viewer
