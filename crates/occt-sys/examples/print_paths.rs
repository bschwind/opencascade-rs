use occt_sys::{occt_include_path, occt_lib_path};

fn main() {
    println!("occt_lib_path: {}", occt_lib_path().to_str().unwrap());
    println!(
        "occt_include_path: {}",
        occt_include_path().to_str().unwrap()
    );
}
