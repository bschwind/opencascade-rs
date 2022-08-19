fn main() {
    let dst = cmake::Config::new("OCCT").define("BUILD_LIBRARY_TYPE", "Static").build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=TKMath");
    println!("cargo:rustc-link-lib=static=TKernel");

    let _build = cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .include(format!("{}", dst.join("include").join("opencascade").display()))
        .file("cpp/wrapper.cpp")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
