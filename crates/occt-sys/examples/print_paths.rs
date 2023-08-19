fn main() {
    println!("occt_path: {}", occt_sys::occt_path().to_str().unwrap());
}
