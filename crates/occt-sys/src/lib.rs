use std::{
    env::var,
    fs, io,
    path::{Path, PathBuf},
};

const LIB_DIR: &str = "lib";
const INCLUDE_DIR: &str = "include";

/// Get the path to the OCCT library installation directory to be
/// used in build scripts.
///
/// Only valid during build (`cargo clean` removes these files).
pub fn occt_path() -> PathBuf {
    // moves the output into target/TARGET/PROFILE/build/OCCT
    // this way its less likely to be rebuilt without a cargo clean
    Path::new(&var("OUT_DIR").expect("missing OUT_DIR")).join("../../OCCT")
}

/// Build the OCCT library.
pub fn build_occt() {
    if fs::read_dir(occt_path().join(LIB_DIR)).is_ok()
        && fs::read_dir(occt_path().join(INCLUDE_DIR)).is_ok()
    {
        // skip rebuild if it already exists
        return;
    }

    let dir = cmake::Config::new(Path::new(env!("OCCT_SRC_DIR")))
        .define("BUILD_PATCH", Path::new(env!("OCCT_PATCH_DIR")))
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

    copy_dir_all(dir.join(LIB_DIR), occt_path().join(LIB_DIR)).expect("failed to copy lib files");
    copy_dir_all(dir.join(INCLUDE_DIR), occt_path().join(INCLUDE_DIR))
        .expect("failed to copy include files");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
