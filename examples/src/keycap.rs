// Keycap generator, referenced from
// https://github.com/cubiq/OPK/blob/53f9d6a4123b0f309f87158115c83d19811b3484/opk.py
use glam::dvec3;
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::{Direction, Face, Shape, Solid},
    workplane::Workplane,
};

const KEYCAP_PITCH: f64 = 19.05;

pub fn shape() -> Shape {
    let convex = false;
    let keycap_unit_size_x = 1.0;
    let keycap_unit_size_y = 1.0;
    let height = 16.0;
    let angle = 13.0.degrees();
    let depth: f64 = 2.8;
    let thickness: f64 = 1.5;
    let base = 18.2;
    let top = 13.2;
    let curve = 1.7;
    let bottom_fillet = 0.5;
    let top_fillet = 5.0;
    let tension = if convex { 0.4 } else { 1.0 };
    let pos = false; // Use POS-style stabilizers

    let top_diff = base - top;

    let bx = KEYCAP_PITCH * keycap_unit_size_x - (KEYCAP_PITCH - base);
    let by = KEYCAP_PITCH * keycap_unit_size_y - (KEYCAP_PITCH - base);

    let tx = bx - top_diff;
    let ty = by - top_diff;

    let base = Workplane::xy().rect(bx, by).fillet(bottom_fillet);

    let mid = Workplane::xy().rect(bx, by).fillet((top_fillet - bottom_fillet) / 3.0).transform(
        dvec3(0.0, 0.0, height / 4.0),
        dvec3(1.0, 0.0, 0.0),
        angle / 4.0,
    );

    // We should use `ConnectEdgesToWires` for `Wire::from_edges`, as it
    // likely puts these arcs in the order we want.
    let top_wire = Workplane::xy()
        .sketch()
        .arc((curve, curve * tension), (0.0, ty / 2.0), (curve, ty - curve * tension))
        .arc((curve, ty - curve * tension), (tx / 2.0, ty), (tx - curve, ty - curve * tension))
        .arc((tx - curve, ty - curve * tension), (tx, ty / 2.0), (tx - curve, curve * tension))
        .arc((tx - curve, curve * tension), (tx / 2.0, 0.0), (curve, curve * tension))
        .wire()
        .fillet(top_fillet)
        .translate(dvec3(-tx / 2.0, -ty / 2.0, 0.0))
        .transform(dvec3(0.0, 0.0, height), dvec3(1.0, 0.0, 0.0), angle);

    let keycap = Solid::loft([&base, &mid, &top_wire]);

    let scoop = if convex {
        let scoop = Workplane::yz()
            .transformed(dvec3(0.0, height - 2.1, -bx / 2.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0, -1.0)
            .three_point_arc((0.0, 2.0), (by / 2.0, -1.0))
            .line_to(by / 2.0, 10.0)
            .line_to(-by / 2.0, 10.0)
            .close();

        let scoop = Face::from_wire(&scoop);
        scoop.extrude(dvec3(bx, 0.0, 0.0))
    } else {
        let scoop_right = Workplane::yz()
            .transformed(dvec3(0.0, height, bx / 2.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0 + 2.0, 0.0)
            .three_point_arc((0.0, (-depth + 1.5).min(-0.1)), (by / 2.0 - 2.0, 0.0))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        let scoop_mid = Workplane::yz()
            .transformed(dvec3(0.0, height, 0.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0 - 2.0, -0.5)
            .three_point_arc((0.0, -depth), (by / 2.0 + 2.0, -0.5))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        let scoop_left = Workplane::yz()
            .transformed(dvec3(0.0, height, -bx / 2.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0 + 2.0, 0.0)
            .three_point_arc((0.0, (-depth + 1.5).min(-0.1)), (by / 2.0 - 2.0, 0.0))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        Solid::loft([&scoop_right, &scoop_mid, &scoop_left])
    };

    let keycap = keycap.subtract(&scoop).fillet_new_edges(0.6);

    let shell_bottom = Workplane::xy().rect(bx - thickness * 2.0, by - thickness * 2.0);

    let shell_mid = Workplane::xy()
        .translated(dvec3(0.0, 0.0, height / 4.0))
        .rect(bx - thickness * 3.0, by - thickness * 3.0);

    let shell_top = Workplane::xy()
        .transformed(dvec3(0.0, 0.0, (height / 4.0) + height - height / 4.0 - 4.5), RVec::x(angle))
        .rect(tx - thickness * 2.0 + 0.5, ty - thickness * 2.0 + 0.5);

    let shell: Shape = Solid::loft([&shell_bottom, &shell_mid, &shell_top]).into();

    let mut keycap = keycap.subtract(&shell);

    let temp_face =
        shell.faces().farthest(Direction::PosZ).workplane().rect(bx * 2.0, by * 2.0).to_face();

    let mut stem_points = vec![];
    let mut ribh_points = vec![];
    let mut ribv_points = vec![];

    if pos {
        let stem_num_x = keycap_unit_size_x.floor();
        let stem_num_y = keycap_unit_size_y.floor();

        let stem_start_x = round_digits(-KEYCAP_PITCH * (stem_num_x / 2.0) + KEYCAP_PITCH / 2.0, 6);
        let stem_start_y = round_digits(-KEYCAP_PITCH * (stem_num_y / 2.0) + KEYCAP_PITCH / 2.0, 6);

        for i in 0..(stem_num_y as usize) {
            ribh_points.push((0.0, stem_start_y + i as f64 * KEYCAP_PITCH));

            for l in 0..(stem_num_x as usize) {
                if i == 0 {
                    ribv_points.push((stem_start_x + l as f64 * KEYCAP_PITCH, 0.0));
                }

                stem_points.push((
                    stem_start_x + l as f64 * KEYCAP_PITCH,
                    stem_start_y + i as f64 * KEYCAP_PITCH,
                ));
            }
        }
    } else {
        stem_points.push((0.0, 0.0));

        if keycap_unit_size_y > keycap_unit_size_x {
            if keycap_unit_size_y > 2.75 {
                let dist = keycap_unit_size_y / 2.0 * KEYCAP_PITCH - KEYCAP_PITCH / 2.0;
                stem_points.extend_from_slice(&[(0.0, dist), (0.0, -dist)]);
            } else if keycap_unit_size_y > 1.75 {
                let dist = 2.25 / 2.0 * KEYCAP_PITCH - KEYCAP_PITCH / 2.0;
                stem_points.extend_from_slice(&[(0.0, -dist), (0.0, dist)]);
            }

            ribh_points.clone_from(&stem_points);
            ribv_points.push((0.0, 0.0));
        } else {
            if keycap_unit_size_x > 2.75 {
                let dist = keycap_unit_size_x / 2.0 * KEYCAP_PITCH - KEYCAP_PITCH / 2.0;
                stem_points.extend_from_slice(&[(dist, 0.0), (-dist, 0.0)]);
            } else if keycap_unit_size_x > 1.75 {
                let dist = 2.25 / 2.0 * KEYCAP_PITCH - KEYCAP_PITCH / 2.0;
                stem_points.extend_from_slice(&[(dist, 0.0), (-dist, 0.0)]);
            }

            ribh_points.push((0.0, 0.0));
            ribv_points.clone_from(&stem_points);
        }
    }

    let bottom_face = keycap.faces().farthest(Direction::NegZ);

    let bottom_workplane = bottom_face.workplane().translated(dvec3(0.0, 0.0, -4.5));

    for (x, y) in &stem_points {
        let circle = bottom_workplane.circle(*x, *y, 2.75).to_face();

        let post = circle.extrude_to_face(&keycap, &temp_face);

        keycap = keycap.union(&post);
    }

    for (x, y) in ribh_points {
        let rect = bottom_workplane.translated(dvec3(x, y, 0.0)).rect(tx, 0.8).to_face();

        let rib = rect.extrude_to_face(&keycap, &temp_face);

        keycap = keycap.union(&rib);
    }

    for (x, y) in ribv_points {
        let rect = bottom_workplane.translated(dvec3(x, y, 0.0)).rect(0.8, ty).to_face();

        let rib = rect.extrude_to_face(&keycap, &temp_face);

        keycap = keycap.union(&rib);
    }

    // TODO(bschwind) - This should probably be done after every union...
    let mut keycap = keycap.clean();

    for (x, y) in &stem_points {
        let bottom_face = keycap.faces().farthest(Direction::NegZ);
        let workplane = bottom_face.workplane().translated(dvec3(0.0, 0.0, -0.6));

        let circle = workplane.circle(*x, *y, 2.75).to_face();

        // TODO(bschwind) - Abstract all this into a "extrude_to_next_face" function.
        let origin = workplane.to_world_pos(dvec3(*x, *y, 0.0));
        let faces = keycap.faces_along_line(origin, workplane.normal());
        let nearest_hit = faces
            .iter()
            .min_by(|hit_a, hit_b| hit_a.t.partial_cmp(&hit_b.t).unwrap())
            .expect("We should have a face to extrude to");
        let post = circle.extrude_to_face(&keycap, &nearest_hit.face);

        keycap = keycap.union(&post).into();
    }

    let r1 = Face::from_wire(&Workplane::xy().rect(4.15, 1.27));
    let r2 = Face::from_wire(&Workplane::xy().rect(1.27, 4.15));

    let mut cross = r1.union(&r2).clean();

    for (x, y) in stem_points {
        cross.set_global_translation(dvec3(x, y, 0.0));
        let cross = cross.extrude(dvec3(0.0, 0.0, 4.6));

        keycap = keycap.subtract(&cross).chamfer_new_edges(0.2);
    }

    keycap
}

fn round_digits(num: f64, digits: i32) -> f64 {
    let multiplier = 10.0f64.powi(digits);
    (num * multiplier).round() / multiplier
}
