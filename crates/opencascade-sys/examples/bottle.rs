use opencascade_sys::ffi::{new_arc_of_circle, new_point, new_segment};

// All dimensions are in millimeters.
pub fn main() {
    let height = 70.0;
    let width = 50.0;
    let thickness = 30.0;

    // Define the points making up the bottle's profile.
    let point_1 = new_point(-width / 2.0, 0.0, 0.0);
    let point_2 = new_point(-width / 2.0, -thickness / 4.0, 0.0);
    let point_3 = new_point(0.0, thickness / 2.0, 0.0);
    let point_4 = new_point(width / 2.0, thickness / 4.0, 0.0);
    let point_5 = new_point(width / 2.0, 0.0, 0.0);

    // Define the
    let segment_1 = new_segment(&point_1, &point_2);
    let segment_2 = new_segment(&point_4, &point_5);
    let arc = new_arc_of_circle(&point_2, &point_3, &point_4);
}
