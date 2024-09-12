use std::path::Path;

const SRC_DIR: &str = "shaders";

fn main() {
    println!("cargo:rerun-if-changed={}", SRC_DIR);

    for entry in std::fs::read_dir(SRC_DIR).expect("Shaders directory should exist") {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(extension) = path.extension().and_then(|os_str| os_str.to_str()) {
            if extension.to_ascii_lowercase().as_str() == "wgsl" {
                println!("cargo:rerun-if-changed={}", path.to_string_lossy());
                compile_shader(path);
            }
        }
    }
}

fn compile_shader<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    let shader_source = std::fs::read_to_string(path).expect("Shader source should be available");

    let module = naga::front::wgsl::parse_str(&shader_source)
        .inspect_err(|e| {
            let msg = e.emit_to_string(&shader_source);
            println!("{msg}");
        })
        .expect("Shader compilation failed");

    let _info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .expect("Shader validation failed");
}
