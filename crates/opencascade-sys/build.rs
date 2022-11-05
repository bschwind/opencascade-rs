fn main() {
    let dst = cmake::Config::new("OCCT").define("BUILD_LIBRARY_TYPE", "Static").build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=TKMath");
    println!("cargo:rustc-link-lib=static=TKernel");
    println!("cargo:rustc-link-lib=static=TKGeomBase");
    println!("cargo:rustc-link-lib=static=TKG2d");
    println!("cargo:rustc-link-lib=static=TKG3d");
    println!("cargo:rustc-link-lib=static=TKTopAlgo");
    println!("cargo:rustc-link-lib=static=TKGeomAlgo");
    println!("cargo:rustc-link-lib=static=TKGeomBase");
    println!("cargo:rustc-link-lib=static=TKBRep");
    println!("cargo:rustc-link-lib=static=TKPrim");
    println!("cargo:rustc-link-lib=static=TKSTL");
    println!("cargo:rustc-link-lib=static=TKMesh");
    println!("cargo:rustc-link-lib=static=TKShHealing");
    println!("cargo:rustc-link-lib=static=TKFillet");
    println!("cargo:rustc-link-lib=static=TKBool");
    println!("cargo:rustc-link-lib=static=TKBO");
    println!("cargo:rustc-link-lib=static=TKOffset");

    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .include(format!("{}", dst.join("include").join("opencascade").display()))
        .include("include")
        .file("cpp/wrapper.cpp")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.hxx");
    println!("cargo:rerun-if-changed=cpp/wrapper.cpp");
}
