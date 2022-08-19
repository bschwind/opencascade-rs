fn main() {
    let dst = cmake::Config::new("OCCT").define("BUILD_LIBRARY_TYPE", "Static").build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=TKMath");

    let _build = cxx_build::bridge("src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
