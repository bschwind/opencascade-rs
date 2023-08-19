use std::path::Path;

/// Get the path to the OCCT library installation directory to be
/// used in build scripts.
///
/// Only valid during build (`cargo clean` removes these files).
pub fn occt_path() -> &'static Path {
    Path::new(env!("OCCT_PATH"))
}
