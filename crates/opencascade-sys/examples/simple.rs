pub fn main() {
    let point = opencascade_sys::ffi::make_gp_Pnt();
    let y = point.Y();
    println!("The point's Y value is {y}");
}
