// Keycap generator, referenced from
// https://github.com/cubiq/OPK/blob/53f9d6a4123b0f309f87158115c83d19811b3484/opk.py
use opencascade::{
    glam::dvec3,
    primitives::{Edge, Solid, Wire},
};

const KEYCAP_PITCH: f64 = 19.05;

pub fn main() {
    let convex = false;
    let keycap_unit_size_x = 1.0;
    let keycap_unit_size_y = 1.0;
    let _angle = 7.0;
    let base = 18.2;
    let top = 13.2;
    let curve = 1.7;
    let bottom_fillet = 0.5;
    let top_fillet = 5.0;
    let tension = if convex { 0.4 } else { 1.0 };

    let top_diff = base - top;

    let bx = KEYCAP_PITCH * keycap_unit_size_x - (KEYCAP_PITCH - base);
    let by = KEYCAP_PITCH * keycap_unit_size_y - (KEYCAP_PITCH - base);

    let tx = bx - top_diff;
    let ty = by - top_diff;

    let mut base = Wire::rect(bx, by);
    base.fillet(bottom_fillet);

    let mut mid = Wire::rect(bx, by);
    mid.fillet((top_fillet - bottom_fillet) / 3.0);

    let arc_1 = Edge::arc(
        dvec3(curve, curve * tension, 0.0),
        dvec3(0.0, ty / 2.0, 0.0),
        dvec3(curve, ty - curve * tension, 0.0),
    );
    let arc_2 = Edge::arc(
        dvec3(curve, ty - curve * tension, 0.0),
        dvec3(tx / 2.0, ty, 0.0),
        dvec3(tx - curve, ty - curve * tension, 0.0),
    );
    let arc_3 = Edge::arc(
        dvec3(tx - curve, ty - curve * tension, 0.0),
        dvec3(tx, ty / 2.0, 0.0),
        dvec3(tx - curve, curve * tension, 0.0),
    );
    let arc_4 = Edge::arc(
        dvec3(tx - curve, curve * tension, 0.0),
        dvec3(tx / 2.0, 0.0, 0.0),
        dvec3(curve, curve * tension, 0.0),
    );

    let mut top_wire = Wire::from_edges([&arc_1, &arc_2, &arc_3, &arc_4]);
    top_wire.fillet(top_fillet);

    // TODO - translate the mid and top wires to the correct height.
    let keycap = Solid::loft([&base, &mid, &top_wire].into_iter());

    keycap.write_stl("keycap.stl").unwrap();
}
