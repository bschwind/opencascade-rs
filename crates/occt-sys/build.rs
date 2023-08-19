const LIB_DIR: &str = "lib";
const INCLUDE_DIR: &str = "include";

fn main() {
    let current_dir = std::env::current_dir().expect("Should have a 'current' directory");
    let patch_dir = current_dir.join("patch");

    let dst = cmake::Config::new("OCCT")
        .define("BUILD_PATCH", patch_dir)
        .define("BUILD_LIBRARY_TYPE", "Static")
        .define("BUILD_MODULE_ApplicationFramework", "FALSE")
        .define("BUILD_MODULE_Draw", "FALSE")
        .define("USE_D3D", "FALSE")
        .define("USE_DRACO", "FALSE")
        .define("USE_EIGEN", "FALSE")
        .define("USE_FFMPEG", "FALSE")
        .define("USE_FREEIMAGE", "FALSE")
        .define("USE_FREETYPE", "FALSE")
        .define("USE_GLES2", "FALSE")
        .define("USE_OPENGL", "FALSE")
        .define("USE_OPENVR", "FALSE")
        .define("USE_RAPIDJSON", "FALSE")
        .define("USE_TBB", "FALSE")
        .define("USE_TCL", "FALSE")
        .define("USE_TK", "FALSE")
        .define("USE_VTK", "FALSE")
        .define("USE_XLIB", "FALSE")
        .define("INSTALL_DIR_LIB", LIB_DIR)
        .define("INSTALL_DIR_INCLUDE", INCLUDE_DIR)
        .build();

    println!("cargo:rustc-env=OCCT_PATH={}", dst.to_str().expect("path is valid Unicode"));
}
