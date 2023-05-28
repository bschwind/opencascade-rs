const LIBS: [&str; 18] = [
    "TKMath",
    "TKMath",
    "TKernel",
    "TKGeomBase",
    "TKG2d",
    "TKG3d",
    "TKTopAlgo",
    "TKGeomAlgo",
    "TKGeomBase",
    "TKBRep",
    "TKPrim",
    "TKSTL",
    "TKMesh",
    "TKShHealing",
    "TKFillet",
    "TKBool",
    "TKBO",
    "TKOffset",
];

fn main() {
    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");
    let is_dynamic = std::env::var("CARGO_FEATURE_DYNAMIC").is_ok();

    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    let mut build = cxx_build::bridge("src/lib.rs");

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", "TRUE");
    }

    if !is_dynamic {
        let opencascade_include = build_opencascade();
        build.include(opencascade_include);

        for lib in LIBS {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    } else {
        for lib in LIBS {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
        build.include("/usr/include/opencascade");
    }

    build
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include("include")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.hxx");
}

fn build_opencascade() -> String {
    let dst = cmake::Config::new("OCCT")
        .define("BUILD_LIBRARY_TYPE", "Static")
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
        .define("INSTALL_DIR_LIB", "lib")
        .define("INSTALL_DIR_INCLUDE", "include")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    format!("{}", dst.join("include").display())
}
