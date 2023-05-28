use opencascade_sys::ffi::new_point;

fn main() {
    let point = new_point(0.0, 0.0, 0.0);
    println!("x: {}, y: {}, z: {}", point.X(), point.Y(), point.Z());
}