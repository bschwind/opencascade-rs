use glam::dvec3;
use opencascade::{
    primitives::{Direction, Shape},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let tolerance = 0.3;

    // The origin of the coordinate system is the closest bottom left corner of
    // the PC box, when viewing its ports from behind.
    let case_thickness = 1.5;
    let case_width = 88.5;
    let case_width = case_width + tolerance;
    let case_height = 50.0; // Not measured, arbitrary value
    let case_depth = 38.5; // measured 38.05;
    let case_depth = case_depth + tolerance;

    let lip_thickness = 2.0;

    let hook_thickness = 3.0;
    let wire_gap = 2.6;

    let case_box = Shape::box_with_dimensions(case_width, case_height, case_depth);

    let back_face = case_box.faces().farthest(Direction::PosY);
    let case_box = case_box.hollow(case_thickness, [back_face]);

    let port_cutout = Workplane::xz()
        .sketch()
        .move_to(lip_thickness, lip_thickness)
        .line_to(lip_thickness, case_depth - lip_thickness)
        .line_to(case_width - lip_thickness, case_depth - lip_thickness)
        .line_to(case_width - lip_thickness, lip_thickness)
        .close()
        .to_face();

    let cutout = port_cutout.extrude(dvec3(0.0, -case_thickness, 0.0)).into();

    let mut case_box = case_box.subtract(&cutout);

    // Add the back hooks
    let bottom_face = case_box.faces().farthest(Direction::NegZ);

    for x_offset in [-25.0, 25.0] {
        let hook_vertical_offset_from_center = -10.0;
        let hook_width = 40.0;
        let hook_attachment_height = 20.0;

        let mut hook: Shape = bottom_face
            .workplane()
            .translated(dvec3(x_offset, hook_vertical_offset_from_center, 0.0))
            .rect(hook_width, hook_attachment_height)
            .to_face()
            .extrude(dvec3(0.0, 0.0, -(hook_thickness + wire_gap)))
            .into();

        let hook_bottom = hook.faces().farthest(Direction::NegY);

        let hook_descent = hook_bottom
            .workplane()
            .translated(dvec3(0.0, -(hook_thickness + wire_gap) / 2.0 + hook_thickness / 2.0, 0.0))
            .rect(hook_width, hook_thickness)
            .to_face()
            .extrude(dvec3(0.0, -hook_attachment_height, 0.0))
            .into();

        hook = hook.union(&hook_descent).into();
        // hook = hook.into();

        let bottom_hook_edges =
            hook.faces().farthest(Direction::NegY).edges().parallel_to(Direction::PosZ);
        hook = hook.fillet_edges(10.0, bottom_hook_edges);

        case_box = case_box.union(&hook);
    }

    // Punch some holes in the back for optional zipties
    for x_offset in [-100.0, -50.0, 0.0, 50.0, 100.0] {
        let ziptie_hole = bottom_face
            .workplane()
            .translated(dvec3(x_offset, -20.0, 0.0))
            .circle(0.0, 0.0, 2.25)
            .to_face()
            .extrude(dvec3(0.0, 0.0, 10.0))
            .into();

        case_box = case_box.subtract(&ziptie_hole)
    }

    // Cut out a circle on the front
    let radius_reduction = 10.0;

    let circle_cutout = case_box
        .faces()
        .farthest(Direction::PosZ)
        .workplane()
        .translated(dvec3(0.0, case_height / 2.0, 0.0))
        .circle(0.0, 0.0, (case_width - radius_reduction) / 2.0)
        .to_face();

    circle_cutout.subtractive_extrude(&case_box, -case_thickness)
}
