pub fn main() {
    let point = opencascade_sys::ffi::new_point(10.0, 7.0, 23.5);
    let y = point.Y();
    println!("The point's Y value is {y}");
}
