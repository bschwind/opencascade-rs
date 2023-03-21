// Keycap generator, referenced from
// https://github.com/cubiq/OPK/blob/53f9d6a4123b0f309f87158115c83d19811b3484/opk.py

const KEYCAP_PITCH: f64 = 19.05;

use opencascade::{
    glam::dvec3,
    primitives::{Edge, Face, Wire},
};

pub fn main() {
    let keycap_unit_size_x = 1.0;
    let keycap_unit_size_y = 1.0;
    let base = 18.2;
    let top = 13.2;
    let bottom_fillet = 0.5;
    let top_fillet = 5.0;

    let top_diff = base - top;

    let bx = KEYCAP_PITCH * keycap_unit_size_x - (KEYCAP_PITCH - base);
    let by = KEYCAP_PITCH * keycap_unit_size_y - (KEYCAP_PITCH - base);

    let mut base = Wire::rect(bx, by);
    base.fillet(bottom_fillet);
    let face = Face::from_wire(&base);

    let body = face.extrude(dvec3(0.0, 0.0, 10.0));

    body.write_stl("keycap.stl").unwrap();
}
