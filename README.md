# opencascade-rs

Rust bindings to OpenCascade. The code is currently a major work in progress.

I currently work a full-time job and work on this in my spare time, so please adjust timing expectations accordingly :)

## Major Goals
* Define 3D CAD models suitable for 3D printing or machining, in ergonomic Rust code
* Code-first approach, but allow use of a GUI where it makes sense (2D sketches)
* Support fillets, chamfers, lofts, surface filling, pipes, extrusions, revolutions, etc.
* Support quick compile times for faster iterations
* Ability to import/export STEP files, STL files, SVG, DXF, KiCAD files, and hopefully more!
* Easy to install the viewer app (provide pre-built binaries for major platforms)
* Easy to create and use user-authored libraries (via crates.io) for quick and easy code-sharing
* Pretty visualizations of created parts
* Ability to specify assemblies of parts, and constraints between assembled parts

## Rationale

This project was born out of me designing [my own keyboard](https://github.com/bschwind/key-ripper) and wanting to make a 3D-printed or CNCed aluminum case for it. In typical over-engineering fashion, I didn't want to just use Fusion360 and call it a day. I wanted a fully parameterized, fully open-source, code-based approach so I can easily make changes, and store the model code in version control. I also want to be fairly confident I can build these models any time in the future given I have a C++/Rust toolchain available.

So I researched what kernels are out there, learned that OpenCascade is one of the few open-source B-Rep (boundary representation) kernels available, and started writing bindings to it with cxx.rs to see if usage of the kernel is feasible. Turns out it is!

### Why Rust?

At this point I'm most comfortable with Rust, so most tools I build will be with Rust. I also don't find any joy in creating my own language or forcing people to learn one I created. Rust is a far better language than I could ever make myself, and contains pretty much every facility I would want for defining 3D models in code. Ultimately it's a hobby project and when you run a hobby project, you get to pick whatever you want :)

There are other benefits:

* Easy to install the Rust toolchain
* Strong type system can inform you what you can do with say, a `Wire` or a `Shape`
* Great generated documentation
* Good cross-platform support
* Excellent library ecosystem on crates.io, making parts sharing a breeze
* High level Rust can be ergonomic, with iterators, closures, operator overloading, and enums
* Rust's unique (`&mut T`) and shared (`&T`) references and function type signatures inform you when an operation modifies a shape vs. creating a new one

## Dependencies

* Rust Toolchain (https://rustup.rs/)
* CMake (https://cmake.org/)
* A C++ compiler with C++11 support

## Building

* The `OCCT` codebase is included as a git submodule. Clone the repo with the `--recursive` flag, or use `git submodule update --init` to fetch the submodule.
* `cargo build --release`

### Using pre-installed OpenCASCADE

If you have the `OCCT` library already installed via a package manager, you can dynamically link to it which will significantly decrease build times. By default, the `builtin` feature is enabled which means compiling OCCT from source. You can disable it via the command line:

`cargo build --no-default-features`

or by specifying `default-features = false` in your `Cargo.toml`:

```
[dependencies]
opencascade = { version = "0.2", default-features = false }
```

NOTE: If you have installed `OCCT` manually you may need specify the path to it via the `DEP_OCCT_ROOT` environment variable. The specified root directory usually contains `include` and `lib` directories.

## Run Examples

* `cargo run --release --example bottle`

The program will output `bottle.stl` in the current working directory.

### Lower Level

There are low level examples which are more or less directly calling OpenCascade functions, such as the classic OpenCascade [bottle](./crates/opencascade-sys/examples/bottle.rs) example, or a [simpler](./crates/opencascade-sys/examples/simple.rs) one.

### Higher Level

The [higher level examples](./crates/opencascade/examples) use more ergonomic Rust APIs, though the exact API is still in flux and subject to change.

## Viewer Application

There is currently an experimental viewer application based on WGPU, which will probably become the "main" way people use this crate. It currently visualizes one of the examples, but will expand to be capable of loading Rust model code compiled to WASM, allowing faster compile times and more interactive inspection of the sketches and models.

To e.g. visualize the keycap example, you can run the current viewer app with

```
$ cargo run --release --bin viewer -- --example keycap
```

To view a STEP file:

```
$ cargo run --release --bin viewer -- --step-file SOME_FILE.step
```

## Example Model Writer

You can write an example model to a file using the `write_model` binary in `examples`.

For more information, run the following command:

```
cargo run --bin write_model -- --help
```

## Code Formatting

### Rust Code
```
$ cargo +nightly fmt
```

### C++ Code
```
$ clang-format -i crates/opencascade-sys/include/wrapper.hxx
```

## Comparison to other tools

### OpenCascade C++ API

This is probably an obvious one, but I use Rust in order to avoid using C++ when possible. You can use OpenCascade directly in its native language, C++, and some people do! I don't have the patience or mental fortitude for it, though. This method of course gives you the full power of OpenCascade without having to write bindings or higher-level wrappers for it.

### [OpenSCAD](https://openscad.org/)

OpenSCAD is how I started with code-based CAD, and it's still a nice tool with lots of community projects and libraries invested into it. To me though, there are several downsides:

* The language is clumsy and limited compared to modern programming languages
* The CAD kernel is CGAL, which is mesh-based. There is less semantic information about geometry, and parts end up just being a soup of triangles.
* Fillets, chamfers, and curves in general end up being more of a pain compared to a B-Rep (Boundary Representation) CAD kernel
* No ability to export STEP files


### [CadQuery](https://cadquery.readthedocs.io/en/latest/)

This project is extremely similar to CadQuery, and owes a lot of its inspiration to it. I mostly like CadQuery, except:

* It's a Python tool, and managing Python dependencies and installations just isn't fun
* The usage of the "fluent" API produces code that is hard to visualize, you have to keep a lot on your mental stack to understand what a given snippet is doing.

These are small complaints, and to the second point, I'm pretty sure you can write more imperative CadQuery code which spells out more obviously what is going on.

I'd say CadQuery is an _excellent_ tool, and likely the most fully-featured code-based CAD tool out there that I'm aware of.

So if you like Python and have patience to deal with Python installations and such, absolutely go with CadQuery. It'll take this project quite awhile to reach feature parity with it.

### [Build123d](https://github.com/gumyr/build123d)

Build123d seems to be an evolution of CadQuery. Still in Python, it replaces the "fluent" API with stateful context managers using `with` blocks. It's still an early project and I haven't looked closely at it, but I do wonder if the context-manager approach will lead to lots of rightward drift in code. Aside from that, it seems like a reasonable syntax approach for CAD modeling.

Still has the same downsides of managing a Python installation and managing how you distribute that.

### [Cascade Studio](https://github.com/zalo/CascadeStudio)

Like CadQuery, Cascade Studio is also based on the OpenCascade kernel. It's quite nice as well, and has an [incredible manual](https://github.com/raydeleu/CascadeStudioManual) with tons of detail. I was mainly turned off by the fact that you have to use the GUI to discover edge indices, which you then pass to the `FilletEdges()` function as a list of numbers. These indices can change as you modify the shape, and it all feels a bit unstable and relies too much on mouse picking from the GUI.

But its web browser support and relatively simple JavaScript API make it a nice, approachable tool if you want to create models quickly.

### [DeclaraCAD](https://declaracad.com/docs/introduction/)

Also based on OpenCascade, DeclaraCAD aims to allow you to write a declarative tree which represents all the operations you perform to create a shape. It seems to have quite rich support for sketches, part modeling, and part assembly. It is distributed as a Qt application and is fully offline and driven by user text files - nice! I would personally worry about the rightward drift of code for non-trivial models, and my brain doesn't really think in a tree the way the code is structured, but if you're a LISPer this is probably perfect!

### [Fornjot](https://github.com/hannobraun/fornjot)

Fornjot is an early-stage B-Rep kernel, written in Rust. I think the project has a lot of potential, but of course being an early-stage project, it's not nearly as featureful as something like OpenCascade, which has had decades of development behind it.

At the same time, I think Rust gives you the power to take on large ambitious projects and keep things organized, so if the maintainer can keep momentum and build a community of contributors behind the project, we may have a nice, pure-Rust solution to code-based CAD.

For now though, I'd rather build a nice Rust API on top of OpenCascade, and then perhaps add Fornjot as a backend to that API when the project is farther along.
