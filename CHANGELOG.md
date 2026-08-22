# Changelog

## v0.3 (2026-08-22)

(sorry this is basically a paste of auto-generated github release notes)

* Update occt-sys to version 0.3 by @bschwind in https://github.com/bschwind/opencascade-rs/pull/125
* Using installed version OpenCASCADE by @katyo in https://github.com/bschwind/opencascade-rs/pull/87
* Small fixups by @bschwind in https://github.com/bschwind/opencascade-rs/pull/128
* Support creating wires out of unordered edges by @bschwind in https://github.com/bschwind/opencascade-rs/pull/131
* Make tolerance of meshing operations configurable by @julianschuler in https://github.com/bschwind/opencascade-rs/pull/121
* Variable Fillets by @bschwind in https://github.com/bschwind/opencascade-rs/pull/130
* Add creating wire from ordered points by @julianschuler in https://github.com/bschwind/opencascade-rs/pull/134
* Support loading a STEP file from the viewer app args by @bschwind in https://github.com/bschwind/opencascade-rs/pull/133
* Add an edge_type() function to Edge by @bschwind in https://github.com/bschwind/opencascade-rs/pull/138
* Add example: turners's cube by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/139
* Add a surface_area() function to Face by @bschwind in https://github.com/bschwind/opencascade-rs/pull/140
* Add Shell::loft(wires) and underlying machinery by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/142
* More Primitive Shapes by @bschwind in https://github.com/bschwind/opencascade-rs/pull/141
* Extend Shape::faces_along_ray to return a LineFaceHitPoint with more info by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/147
* Re-add the intersect function by @bschwind in https://github.com/bschwind/opencascade-rs/pull/150
* Offset for Face and Wire by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/145
* Update simple-game/wgpu all those dependencies by @bschwind in https://github.com/bschwind/opencascade-rs/pull/154
* Sweep Faces and Wires along a path by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/155
* Support pan, rotate and zoom camera operations by @skywhale in https://github.com/bschwind/opencascade-rs/pull/108
* Add BRepBuilderAPI_MakeShapeOnMesh by @DSchroer in https://github.com/bschwind/opencascade-rs/pull/156
* Add the beginning of a very simplistic kicad board file parser by @bschwind in https://github.com/bschwind/opencascade-rs/pull/94
* Add make solid and shell constructors by @DSchroer in https://github.com/bschwind/opencascade-rs/pull/157
* Rerun build script if env var changes by @DSchroer in https://github.com/bschwind/opencascade-rs/pull/158
* fix: double free segfault when using Handle_Poly_Triangulation_ctor by @DSchroer in https://github.com/bschwind/opencascade-rs/pull/159
* fix: emscripten unable to find OCCT by @DSchroer in https://github.com/bschwind/opencascade-rs/pull/161
* Fix the problem that the windows-gnu environment failed to find wrapper.hxx by @yk0n9 in https://github.com/bschwind/opencascade-rs/pull/113
* Toggle hidden-line drawing with the X key in the viewer app by @bschwind in https://github.com/bschwind/opencascade-rs/pull/162
* Add a cable bracket example by @bschwind in https://github.com/bschwind/opencascade-rs/pull/167
* feat: write example model to STEP or STL file & refactor by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/174
* feat: variable radius pipe by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/175
* First pass at using wit-bindgen/wasmtime and WIT interfaces by @bschwind in https://github.com/bschwind/opencascade-rs/pull/173
* feat: construct Edge as B-spline interpolating points by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/177
* feat: gtransform API by @DSchroer in https://github.com/bschwind/opencascade-rs/pull/181
* Add a bracket for flat ethernet cables by @bschwind in https://github.com/bschwind/opencascade-rs/pull/168
* Zotac ZBOX Mount by @bschwind in https://github.com/bschwind/opencascade-rs/pull/169
* fix: allow cross compilation under builtin flag v2 by @fidoriel in https://github.com/bschwind/opencascade-rs/pull/185
* Keyboard Case V2 by @bschwind in https://github.com/bschwind/opencascade-rs/pull/170
* Add iges import and export by @fidoriel in https://github.com/bschwind/opencascade-rs/pull/186
* Update occt sys to 0.6 by @fidoriel in https://github.com/bschwind/opencascade-rs/pull/190
* Documentation: Link to OpenCascade website by @schubart in https://github.com/bschwind/opencascade-rs/pull/192
* Construct edge as a Bezier curve from control points by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/191
* More boolean ops for Face and CompoundFace by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/195
* Handle name conflict to fix Windows build by @bschwind in https://github.com/bschwind/opencascade-rs/pull/196
* Ability to cast a Face to a CompoundFace by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/197
* Small wgpu update by @bschwind in https://github.com/bschwind/opencascade-rs/pull/198
* [kicad-parser] Refactor data types by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/200
* Reduce Render Passes by @bschwind in https://github.com/bschwind/opencascade-rs/pull/203
* Render instanced line segments instead of line strips by @bschwind in https://github.com/bschwind/opencascade-rs/pull/204
* Update dependencies used for WASM execution by @bschwind in https://github.com/bschwind/opencascade-rs/pull/208
* Add section API by @tymokvo in https://github.com/bschwind/opencascade-rs/pull/207
* Add bounding box support by @tymokvo in https://github.com/bschwind/opencascade-rs/pull/212
* feat: `Shape::empty()` by @mkovaxx in https://github.com/bschwind/opencascade-rs/pull/215
* Add gp_Dir.Transform to opencascade-sys by @gmorenz in https://github.com/bschwind/opencascade-rs/pull/210
* Add BRep I/O support (Text and Binary) by @lzpel in https://github.com/bschwind/opencascade-rs/pull/218
* Split C++ bridge into multiple files by @bschwind in https://github.com/bschwind/opencascade-rs/pull/199
* Add translate/rotate/scale/mirror transform operations to Shape by @torusJKL in https://github.com/bschwind/opencascade-rs/pull/224
* Add safe downcasts from Shape to Wire/Face/Solid by @torusJKL in https://github.com/bschwind/opencascade-rs/pull/225
* Write multiple shapes to one STEP file by @torusJKL in https://github.com/bschwind/opencascade-rs/pull/226
* Expose BRepBuilderAPI_MakeFace::Add for faces with holes by @torusJKL in https://github.com/bschwind/opencascade-rs/pull/227
* Add a basic under-the-desk power brick bracket example by @bschwind in https://github.com/bschwind/opencascade-rs/pull/229

## v0.2 (2023-08-16)

The current version existing on crates.io when this CHANGELOG file was created. This version contains hand-written bindings to the C++ code, and the bindings were mostly made as they were needed, so many things are missing. You can create basic primitives and do things like chamfer/fillet edges, perform boolean operations on shapes, and export STL or STEP files.

There is a viewer app which lets you view your generated 3D model in a basic way, and there is a (very limited) WebAssembly hot reloading "engine" which lets you write model code and see it update when you save.
