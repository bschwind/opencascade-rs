const LIB_DIR: &str = "lib";
const INCLUDE_DIR: &str = "include";

fn main() {
    let mut cfg = cmake::Config::new("OCCT");

    cfg.define("BUILD_LIBRARY_TYPE", "Static")
        .define("BUILD_MODULE_Draw", "FALSE")
        .define("USE_OPENGL", "FALSE")
        .define("USE_GLES2", "FALSE")
        .define("USE_D3D", "FALSE")
        .define("USE_VTK", "FALSE")
        .define("USE_TCL", "FALSE")
        .define("USE_XLIB", "FALSE")
        .define("USE_FREETYPE", "FALSE")
        .define("USE_FREEIMAGE", "FALSE")
        .define("USE_OPENVR", "FALSE")
        .define("USE_FFMPEG", "FALSE")
        .define("INSTALL_DIR_LIB", LIB_DIR)
        .define("INSTALL_DIR_INCLUDE", INCLUDE_DIR);

    if let Ok(ccache) = which::which("sccache").or_else(|_| which::which("ccache")) {
        cfg.define("CMAKE_C_COMPILER_LAUNCHER", ccache.as_os_str())
            .define("CMAKE_CXX_COMPILER_LAUNCHER", ccache.as_os_str());
    }

    let dst = cfg.build();

    println!(
        "cargo:rustc-env=OCCT_LIB_PATH={}",
        dst.join(LIB_DIR).to_str().expect("path is valid Unicode")
    );
    println!(
        "cargo:rustc-env=OCCT_INCLUDE_PATH={}",
        dst.join(INCLUDE_DIR).to_str().expect("path is valid Unicode")
    );
}
