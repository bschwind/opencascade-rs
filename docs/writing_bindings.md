# Binding to OpenCascade Classes and Functions

OpenCascade is a huge C++ project, spanning mulitple decades. It has an impressive amount of functionality packed in, though it is somewhat hidden behind a (in my opinion) ugly/intimidating API.

One goal of this project is to expose a more ergonomic and approachable API on top of the goodies within OpenCascade so more people can use it to build cool things. OpenCascade leans pretty hard into C++, and so we need to use something more than just `bindgen` in order to have Rust call it safely.

For that we use [CXX](https://cxx.rs/), by dtolnay. CXX itself is also somewhat intimidating and hard to figure out on first use, so this document will hopefully show how _this_ project uses it to bind to OpenCascade and expose useful functionality.

## Organization

### `build.rs`

At the very bottom, we have the [opencascade-sys crate](../crates/opencascade-sys), which includes the OpenCascade source code in the [OCCT directory](../crates/opencascade-sys/OCCT).

This sys crate has a [build.rs](../crates/opencascade-sys/build.rs) file which calls out to `cmake` to configure and build the project. I've tried to disable as many features as possible which are unrelated to model building (such as FFMPEG, OpenGL, Tcl, basically anything related to visualization or infrastructure as we'll be building that ourselves in Rust).

In the theme of keeping it minimal, the OpenCascade libraries we statically link to are all explicitly laid out with `cargo:rustc-link-lib=static=LIB_NAME_HERE` cargo directives.

### C++/Rust Bridge File

In order to expose C++ types to Rust, and vice-versa, we need to write what cxx.rs calls a "bridge" file or module. This project currently has [one giant bridge file](../crates/opencascade-sys/src/lib.rs), but in the interest of organization this will eventually be broken up into sensible modules.

At the very top of the bridge module we can define types that are visible to both C++ and Rust, such as simple enums which can be represented as `u32`, for example.

Pretty much everything else goes inside of an `unsafe extern "C++" {}` block. Inside of this block, we declare opaque C++ types that we want Rust to know about. It is enough to simply state `type SOME_CPP_TYPE_HERE;` to make this declaration. `cxx` will then search included headers and make sure that type exists in the C++ world. With just the type, you can't really do anything, so you also need to start declaring functions that _should_ exist in the C++ world which you want to use.

There are some rules to how we define these functions in our code:

* At no point can a function in the bridge return a bare, owned C++ type. It must be behind an immutable reference, or a smart pointer such as `UniquePtr<T>` or `SharedPtr<T>`.
* If you're binding to a class method (not a free-floating function), you must use the `self` keyword as the first argument to the function.
    * If the method is `const` on the C++ side (not modifying `self`), then `fn do_something(self: &TheCPPTypeHere)` is sufficient.
    * If the method is _not_ `const`, then the signature must be `fn do_something(self: Pin<&mut TheCPPTypeHere>)`
    * Getting this wrong will result in ugly C++ compile errors getting spewed out on the console.
* If you're binding to a free-floating function, then avoid using the special `self` keyword as the name for a function argument, and just use `&T` and `Pin<&mut T>` as appropriate.
* From what I can tell, generics don't work from Rust to C++ templates. If you have a C++ type called `Handle<Edge>`, you'll need to declare your own C++ type called `HandleEdge` or whatever you want, and alias it to the full type (ex:`typedef opencascade::handle<Edge> HandleEdge;`
* You can use `#[cxx_name = "SomeCPPFunctionName"]` to tell `cxx` the _real_ name of the C++ function you want to use, and `#[rust_name = "some_rust_fn_name"]` to control what the name of the function is exposed to the Rust side of things. If you don't use these attributes, the exported Rust function is exactly the same as the C++ function name.

#### `construct_unique`



### wrapper.hxx

## Example: Binding to the STEP File Import Functionality

In order to give a concrete "tutorial" on binding to new functionality in OpenCascade, I'll go over what was required to add STEP file import functionality to this crate. You can see all the changes for that functionality in [this PR](https://github.com/bschwind/opencascade-rs/pull/33).
