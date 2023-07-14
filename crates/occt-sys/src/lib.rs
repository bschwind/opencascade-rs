use std::path::Path;

/// Get path to OCCT library directory with built static libraries. To be used in build scripts
/// with the `cargo:rustc-link-search` command.
///
/// Only valid during build (`cargo clean` removes these files).
pub fn occt_lib_path() -> &'static Path {
    Path::new(env!("OCCT_LIB_PATH"))
}

/// Get path to OCCT header files.
///
/// Only valid during build (`cargo clean` removes these files).
pub fn occt_include_path() -> &'static Path {
    Path::new(env!("OCCT_INCLUDE_PATH"))
}
