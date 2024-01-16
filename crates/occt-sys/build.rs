use std::{env::var, path::Path};

fn main() {
    println!(
        "cargo:rustc-env=OCCT_SRC_DIR={}",
        Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("OCCT").to_string_lossy()
    );
    println!(
        "cargo:rustc-env=OCCT_PATCH_DIR={}",
        Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("patch").to_string_lossy()
    );
}
