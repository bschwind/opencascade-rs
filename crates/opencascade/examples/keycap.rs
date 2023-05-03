// Keycap generator, referenced from
// https://github.com/cubiq/OPK/blob/53f9d6a4123b0f309f87158115c83d19811b3484/opk.py
use opencascade::{
    glam::dvec3,
    primitives::{Face, Solid},
    workplane::Workplane,
};

const KEYCAP_PITCH: f64 = 19.05;

pub fn main() {
    let convex = true;
    let keycap_unit_size_x = 1.0;
    let keycap_unit_size_y = 1.0;
    let height = 13.0;
    let angle = 7.0;
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

    let mut base = Workplane::xy().rect(bx, by);
    base.fillet(bottom_fillet);

    let mut mid = Workplane::xy().rect(bx, by);
    mid.fillet((top_fillet - bottom_fillet) / 3.0);
    mid.transform(dvec3(0.0, 0.0, height / 4.0), dvec3(1.0, 0.0, 0.0), angle / 4.0);

    // We should use `ConnectEdgesToWires` for `Wire::from_edges`, as it
    // likely puts these arcs in the order we want.
    let mut top_wire = Workplane::xy()
        .sketch()
        .arc((curve, curve * tension), (0.0, ty / 2.0), (curve, ty - curve * tension))
        .arc((curve, ty - curve * tension), (tx / 2.0, ty), (tx - curve, ty - curve * tension))
        .arc((tx - curve, ty - curve * tension), (tx, ty / 2.0), (tx - curve, curve * tension))
        .arc((tx - curve, curve * tension), (tx / 2.0, 0.0), (curve, curve * tension))
        .wire();

    top_wire.fillet(top_fillet);
    top_wire.translate(dvec3(-tx / 2.0, -ty / 2.0, 0.0));
    top_wire.transform(dvec3(0.0, 0.0, height), dvec3(1.0, 0.0, 0.0), angle);

    let mut keycap = Solid::loft([&base, &mid, &top_wire].into_iter());

    let scoop = Workplane::yz()
        .transformed(dvec3(0.0, height - 2.1, -bx / 2.0), dvec3(0.0, 0.0, angle))
        .sketch()
        .move_to(-by / 2.0, -1.0)
        .three_point_arc((0.0, 2.0), (by / 2.0, -1.0))
        .line_to(by / 2.0, 10.0)
        .line_to(-by / 2.0, 10.0)
        .close();

    let scoop = Face::from_wire(&scoop);
    let scoop = scoop.extrude(dvec3(bx, 0.0, 0.0));

    let keycap = keycap.subtract(&scoop);

    keycap.write_stl("keycap.stl").unwrap();
}
