use std::path::PathBuf;

const LIBS: &[&str] = &[
    "TKMath",
    "TKernel",
    "TKFeat",
    "TKGeomBase",
    "TKG2d",
    "TKG3d",
    "TKTopAlgo",
    "TKGeomAlgo",
    "TKGeomBase",
    "TKBRep",
    "TKPrim",
    "TKSTEP",
    "TKSTEPAttr",
    "TKSTEPBase",
    "TKSTEP209",
    "TKSTL",
    "TKMesh",
    "TKShHealing",
    "TKFillet",
    "TKBool",
    "TKBO",
    "TKOffset",
    "TKV3d",
    "TKXSBase"
];

#[cfg(feature = "dynamic")]
mod opencascade {
    use super::*;

    const LIB_NAME: &str = "opencascade";
    const LIB_VERSION: &str = "7.6.3";

    pub(super) fn get_build_paths() -> Result<(PathBuf, PathBuf), String> {
        let (pkgconfig_include_path, pkgconfig_lib_path) = find_pkgconfig_paths();

        let include_path = std::env::var("OPENCASCADE_INCLUDE")
            .ok()
            .map(|x| x.into())
            .or(pkgconfig_include_path);
        let lib_path = std::env::var("OPENCASCADE_LIB")
            .ok()
            .map(|x| x.into())
            .or(pkgconfig_lib_path);

        println!("{:?}, {:?}", include_path, lib_path);

        match (include_path, lib_path) {
            (Some(include_path), Some(lib_path)) => Ok((include_path, lib_path)),
            _ => {
                eprintln!("Couldn't find either header or lib files for {}.", LIB_NAME);
                eprintln!("See the crate README for installation instructions, or not use the 'dynamic' feature to statically compile bundled.");
                Err("Missing OpenCascade header or lib files".into())
            },
        }
    }

    pub(super) fn build_if_necessary() -> Result<(PathBuf, PathBuf), String> {
        get_build_paths()
    }

    // TODO this will not work on macOS Homebrew installed opencascade because
    // .pc is not installed for pkg-config to find. Use must supply
    // OPENCASCADE_INCLUDE=/opt/homebrew/include/opencascade and
    // OPENCASCADE_LIB=/opt/homebrew/lib. Not least because the Homebrew
    // version is newer anyway. Why no longer error but Option so that can
    // fallback to env vars
    fn find_pkgconfig_paths() -> (Option<PathBuf>, Option<PathBuf>) {
        match pkg_config::Config::new()
            // Be exact because API changes often break builds
            .exactly_version(LIB_VERSION)
            .probe(LIB_NAME) {
                Ok(mut libary) => {
                    let include_path = libary
                        .include_paths
                        .pop();
                    let lib_path = libary
                        .link_paths
                        .pop();
                    (include_path, lib_path)
                },
                Err(e) => {
                    eprintln!("Couldn't find {} via pkg-config: {}", LIB_NAME, e);
                    (None, None)
                }
            }
    }
}

#[cfg(not(feature = "dynamic"))]
mod opencascade {
    use super::*;

    pub(super) fn build_if_necessary() -> Result<(PathBuf, PathBuf), String> {
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

        Ok((dst.join("include"), dst.join("lib")))
    }
}

fn main() {
    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    let mut build = cxx_build::bridge("src/lib.rs");

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", "TRUE");
    }

    let (include_path, lib_path) = opencascade::build_if_necessary().unwrap_or_else(|e| {
        eprintln!("Failed to build OpenCascade: {}", e);
        std::process::exit(1);
    });

    build.include(include_path);

    for lib in LIBS {
        if cfg!(feature = "dynamic") {
            println!("cargo:rustc-link-lib=dylib={lib}");
        } else {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    }
    println!("cargo:rustc-link-search=native={}", lib_path.display());

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
