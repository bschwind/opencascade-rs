# opencascade-rs

Rust bindings to OpenCascade. The code is currently a major work in progress.

## Major Goals
* Define 3D CAD models in ergonomic Rust code
* Code-first approach, but allow use of a GUI where it makes sense (2D sketches)
* Support fillets, chamfers, lofts, surface filling, pipes, extrusions, lathes, etc.
* Support quick compile times for faster iterations
* Ability to import/export STEP files, STL files, KiCAD files
* Easy to install the viewer app (provide pre-built binaries for major platforms)
* Easy to create and use user-authored libraries (via crates.io) for quick and easy code-sharing
* Pretty visualizations of created parts
* Ability to specify assemblies of parts, and constraints between assembled parts

## Dependencies

* Rust Toolchain (https://rustup.rs/)
* CMake (https://cmake.org/)
* A C++ compiler with C++11 support

## Building

* The `OCCT` codebase is included as a git submodule. Clone the repo with the `--recursive` flag, or use `git submodule update --init` to fetch the submodule.
* `cargo build --release`

## Run Examples

* `cargo run --release --example bottle`

### Lower Level
There are low level examples which are more or less directly calling OpenCascade functions, such as the classic OpenCascade [bottle](./crates/opencascade-sys/examples/bottle.rs) example, or a [simpler](./crates/opencascade-sys/examples/simple.rs) one.

### Higher Level
The [higher level examples](./crates/opencascade/examples) use more ergonomic Rust APIs, though the exact API is still in flux and subject ot change.

## Viewer Application
There is currently an experimental viewer application based on WGPU, which will probably become the "main" way people use this crate. It currently visualizes a hardcoded model produced in Rust code, but will expand to be capable of loading Rust model code compiled to WASM, allowing faster compile times and more interactive inspection of the sketches and models.

You can run the current viewer app with

```
$ cargo run --release --bin viewer
```

## Code Formatting

```
$ cargo +nightly fmt
```
