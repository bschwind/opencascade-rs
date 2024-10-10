use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade::{
    primitives::{Direction, IntoShape, JoinType, Shape, Wire},
    workplane::Workplane,
};

// All units are in millimeters.
// The top/bottom/left/right conventions relate to 2D rectangles in
// cartesian coordinates (positive Y axis points up). Values with "_Z"
// in their name refer to the Z coordinate.
const PCB_THICKNESS: f64 = 1.6;
const PCB_WIDTH: f64 = 285.75;
const PCB_HEIGHT: f64 = 114.3;
const TOP_PLATE_THICKNESS: f64 = 1.6;

const PCB_FILLET_RADIUS: f64 = 2.4;

// "Inflate" the PCB dimensions by this much to create an easier fit.
const PCB_DIMENSION_TOLERANCE: f64 = 0.9;

// The origin point for this board is the top left corner
// of the PCB, on the top surface. The PCB rests on this
// shelf. Positive X values go to the right, positive Y
// values go up.
const ORIGIN: DVec3 = DVec3::new(0.0, 0.0, 0.0);

// Case
const CASE_WALL_THICKNESS: f64 = 3.5;
const CASE_LIP_HEIGHT: f64 = 1.0;

const CASE_TOP: f64 = PCB_TOP + CASE_WALL_THICKNESS;
const CASE_TOP_Z: f64 = TOP_PLATE_TOP_Z + CASE_LIP_HEIGHT;
const CASE_BOTTOM: f64 = PCB_BOTTOM - CASE_WALL_THICKNESS;
const CASE_LEFT: f64 = PCB_LEFT - CASE_WALL_THICKNESS;
const CASE_RIGHT: f64 = PCB_RIGHT + CASE_WALL_THICKNESS;
const CASE_FLOOR_Z: f64 = PCB_BOTTOM_Z - PCB_SHELF_HEIGHT;
const CASE_BOTTOM_Z: f64 = CASE_FLOOR_Z - CASE_WALL_THICKNESS;

const CASE_FOOT_THICKNESS: f64 = 2.1;

// PCB
const PCB_TOP: f64 = ORIGIN.y + PCB_DIMENSION_TOLERANCE;
const PCB_TOP_NO_TOLERANCE: f64 = ORIGIN.y;
const PCB_TOP_Z: f64 = ORIGIN.z;
const PCB_BOTTOM: f64 = PCB_TOP_NO_TOLERANCE - PCB_HEIGHT - PCB_DIMENSION_TOLERANCE;
const PCB_BOTTOM_NO_TOLERANCE: f64 = PCB_TOP_NO_TOLERANCE - PCB_HEIGHT;
const PCB_BOTTOM_Z: f64 = PCB_TOP_Z - PCB_THICKNESS;
const PCB_LEFT: f64 = ORIGIN.x - PCB_DIMENSION_TOLERANCE;
const PCB_LEFT_NO_TOLERANCE: f64 = ORIGIN.x;
const PCB_RIGHT: f64 = PCB_LEFT_NO_TOLERANCE + PCB_WIDTH + PCB_DIMENSION_TOLERANCE;
const PCB_RIGHT_NO_TOLERANCE: f64 = PCB_LEFT_NO_TOLERANCE + PCB_WIDTH;

// Top Plate
const TOP_PLATE_BOTTOM_Z: f64 = 3.4;
const TOP_PLATE_TOP_Z: f64 = TOP_PLATE_BOTTOM_Z + TOP_PLATE_THICKNESS;

// PCB Shelf
const PCB_SHELF_THICKNESS_TOP: f64 = 2.75;
const PCB_SHELF_THICKNESS_BOTTOM: f64 = 4.0;
const PCB_SHELF_HEIGHT: f64 = 4.0;

// Top plate support post locations
const SUPPORT_POST_RADIUS: f64 = 2.0;

// http://www.metrication.com/engineering/threads.html
const SUPPORT_POST_DRILL_RADIUS: f64 = 0.8;
const SUPPORT_POST_DIST_FROM_EDGE: f64 = 2.5;

#[allow(unused)]
enum PostDirection {
    Up,
    Down,
    Left,
    Right,
}

struct SupportPost {
    pos: DVec2,
    direction: PostDirection,
}

impl SupportPost {
    fn shape(&self) -> Shape {
        let bottom_z = CASE_FLOOR_Z;
        let top_z = TOP_PLATE_BOTTOM_Z;

        let pos = DVec3::from((self.pos, CASE_FLOOR_Z));
        let cylinder = Shape::cylinder(pos, SUPPORT_POST_RADIUS, DVec3::Z, top_z - bottom_z);
        let m2_drill_hole =
            Shape::cylinder(pos, SUPPORT_POST_DRILL_RADIUS, DVec3::Z, top_z - bottom_z);

        let dist_from_edge = SUPPORT_POST_DIST_FROM_EDGE + PCB_DIMENSION_TOLERANCE;

        let box_part = match self.direction {
            PostDirection::Up => {
                let corner_1 =
                    DVec3::new(pos.x - SUPPORT_POST_RADIUS, pos.y + dist_from_edge, bottom_z);
                let corner_2 = DVec3::new(pos.x + SUPPORT_POST_RADIUS, pos.y, top_z);

                Shape::box_from_corners(corner_1, corner_2)
            },
            PostDirection::Down => {
                let corner_1 = DVec3::new(pos.x - SUPPORT_POST_RADIUS, pos.y, bottom_z);
                let corner_2 =
                    DVec3::new(pos.x + SUPPORT_POST_RADIUS, pos.y - dist_from_edge, top_z);

                Shape::box_from_corners(corner_1, corner_2)
            },
            PostDirection::Left => {
                let corner_1 =
                    DVec3::new(pos.x - dist_from_edge, pos.y - SUPPORT_POST_RADIUS, bottom_z);
                let corner_2 = DVec3::new(pos.x, pos.y + SUPPORT_POST_RADIUS, top_z);

                Shape::box_from_corners(corner_1, corner_2)
            },
            PostDirection::Right => {
                let corner_1 = DVec3::new(pos.x, pos.y - SUPPORT_POST_RADIUS, bottom_z);
                let corner_2 =
                    DVec3::new(pos.x + dist_from_edge, pos.y + SUPPORT_POST_RADIUS, top_z);

                Shape::box_from_corners(corner_1, corner_2)
            },
        };

        cylinder.union(&box_part).subtract(&m2_drill_hole).into()
    }
}

const SUPPORT_POSTS: &[SupportPost] = &[
    SupportPost {
        pos: DVec2::new(119.075, PCB_TOP_NO_TOLERANCE - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(204.75, PCB_TOP_NO_TOLERANCE - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(80.95, PCB_BOTTOM_NO_TOLERANCE + SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
    SupportPost {
        pos: DVec2::new(200.05, PCB_BOTTOM_NO_TOLERANCE + SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
];

// The radius of the holes to drill in the bottom of the case to allow
// for threaded M3 rubber feet to pass through and attach to the
// threaded inserts on the PCB.
const BOTTOM_CUTOUT_RADIUS: f64 = 2.1;

const FEET_CUTOUTS: &[DVec2] = &[
    DVec2::new(19.05, -18.25),
    DVec2::new(266.65, -18.25),
    DVec2::new(19.05, -96.45),
    DVec2::new(266.65, -96.45),
];

// USB C Connector Cutout
const USB_CONNECTOR_PADDING: f64 = 1.0;
const USB_WIDTH: f64 = 9.0 + USB_CONNECTOR_PADDING;
const USB_HEIGHT: f64 = 7.45;
const USB_DEPTH_NO_PADDING: f64 = 3.26;
const USB_DEPTH: f64 = 3.26 + USB_CONNECTOR_PADDING;
const USB_RADIUS: f64 = 1.43;

const USB_MIDDLE_X: f64 = 26.194;
const USB_LEFT: f64 = USB_MIDDLE_X - (USB_WIDTH / 2.0);
const USB_RIGHT: f64 = USB_MIDDLE_X + (USB_WIDTH / 2.0);
const USB_TOP: f64 = 3.04;
const USB_BOTTOM: f64 = USB_TOP - USB_HEIGHT;

// Keep out zones for space bar stabilizer
const STABILIZER_SCREW_KEEPOUT_WIDTH: f64 = 8.0;

// A list of the left sides of each stabilizer screw "zones"
// where the PCB shelf should be cut away to accomodate
// stabilizer screws on the bottom of the PCB.
const STABILIZER_KEEPOUTS: &[f64] = &[87.0, 187.24];

// Pinholes for BOOT + reset buttons
const PINHOLE_BUTTON_RADIUS: f64 = 1.1;

const PINHOLE_LOCATIONS: &[DVec2] = &[DVec2::new(35.85, -53.95), DVec2::new(8.425, -86.075)];

fn case_outer_box() -> Shape {
    let mut workplane = Workplane::xy();
    workplane.set_translation(dvec3(0.0, 0.0, CASE_BOTTOM_Z));

    let outer_box = workplane
        .sketch()
        .move_to(CASE_LEFT, CASE_TOP)
        .line_to(CASE_RIGHT, CASE_TOP)
        .line_to(CASE_RIGHT, CASE_BOTTOM)
        .line_to(CASE_LEFT, CASE_BOTTOM)
        .close()
        .fillet(PCB_FILLET_RADIUS + CASE_WALL_THICKNESS)
        .to_face()
        .extrude(dvec3(0.0, 0.0, CASE_TOP_Z - CASE_BOTTOM_Z))
        .into_shape();

    let top_edges = outer_box.faces().farthest(Direction::PosZ).edges();
    let bottom_edges = outer_box.faces().farthest(Direction::NegZ).edges();

    outer_box.chamfer_edges(1.5, top_edges.chain(bottom_edges))
}

fn case_inner_box() -> Shape {
    let mut workplane = Workplane::xy();
    workplane.set_translation(dvec3(0.0, 0.0, CASE_FLOOR_Z));

    workplane
        .sketch()
        .move_to(PCB_LEFT, PCB_TOP)
        .line_to(PCB_RIGHT, PCB_TOP)
        .line_to(PCB_RIGHT, PCB_BOTTOM)
        .line_to(PCB_LEFT, PCB_BOTTOM)
        .close()
        .fillet(PCB_FILLET_RADIUS)
        .to_face()
        // 0.1 is a fudge factor to retrieve edges from boolean subtraction
        .extrude(dvec3(0.0, 0.0, CASE_TOP_Z + 0.1 - CASE_FLOOR_Z))
        .into_shape()
}

fn pcb_top_shelf() -> Shape {
    let corner_1 = DVec3::new(PCB_LEFT, PCB_TOP, CASE_FLOOR_Z);
    let corner_2 = DVec3::new(PCB_RIGHT, PCB_TOP - PCB_SHELF_THICKNESS_TOP, PCB_BOTTOM_Z);

    Shape::box_from_corners(corner_1, corner_2)
}

fn pcb_bottom_shelf() -> Shape {
    let corner_1 = DVec3::new(PCB_LEFT, PCB_BOTTOM + PCB_SHELF_THICKNESS_BOTTOM, CASE_FLOOR_Z);
    let corner_2 = DVec3::new(PCB_RIGHT, PCB_BOTTOM, CASE_FLOOR_Z + PCB_SHELF_HEIGHT);

    let mut bottom_shelf = Shape::box_from_corners(corner_1, corner_2);

    // Cut out gaps for the space bar stabilizer.

    for keepout_left in STABILIZER_KEEPOUTS {
        let corner_1 = DVec3::new(*keepout_left, corner_1.y, corner_1.z);
        let corner_2 =
            DVec3::new(keepout_left + STABILIZER_SCREW_KEEPOUT_WIDTH, corner_2.y, corner_2.z);

        let cutout_box = Shape::box_from_corners(corner_1, corner_2);

        bottom_shelf = bottom_shelf.subtract(&cutout_box).into();
    }

    bottom_shelf
}

fn usb_connector_cutout() -> Shape {
    let mut usb_workplane = Workplane::xz();
    usb_workplane.set_translation(dvec3(
        USB_MIDDLE_X,
        USB_BOTTOM,
        PCB_BOTTOM_Z - (USB_DEPTH_NO_PADDING / 2.0) - 0.01, // TODO - why does it segfault without this fudge factor?
    ));

    usb_workplane
        .rect(USB_WIDTH, USB_DEPTH)
        .fillet(USB_RADIUS)
        .to_face()
        .extrude(dvec3(0.0, USB_HEIGHT + CASE_WALL_THICKNESS, 0.0))
        .into_shape()
}

// This is the little trapezoidal PCB shape which helps the USB C connector
// extend forward into the case.
fn pcb_usb_overhang() -> Shape {
    const VERTICAL_PADDING: f64 = 1.0;
    const TRAPEZOID_PADDING: f64 = 1.0;

    let start = CASE_FLOOR_Z;
    let points = [
        DVec3::new(19.05, 0.0, start),
        DVec3::new(USB_LEFT - 0.3, 2.381, start),
        DVec3::new(USB_RIGHT + 0.3, 2.381, start),
        DVec3::new(33.337, 0.0, start),
        DVec3::new(33.337, PCB_TOP - PCB_SHELF_THICKNESS_TOP, start),
        DVec3::new(19.05, PCB_TOP - PCB_SHELF_THICKNESS_TOP, start),
    ];

    let wire = Wire::from_ordered_points(points).unwrap().offset(TRAPEZOID_PADDING, JoinType::Arc);
    wire.to_face().extrude(dvec3(0.0, 0.0, PCB_TOP_Z - start + VERTICAL_PADDING)).into()
}

#[allow(unused)]
enum FootStyle {
    PointingUp,
    PointingDown,
    Line,
}

fn case_foot(center: DVec2, foot_style: FootStyle, foot_thickness: f64, z_extrude: f64) -> Shape {
    const FOOT_EXTENT: f64 = 15.0;

    let half_foot_thickness = foot_thickness / 2.0;

    let mut workplane = Workplane::xy();
    workplane.set_translation(dvec3(0.0, 0.0, CASE_BOTTOM_Z));

    let sketch = match foot_style {
        FootStyle::PointingUp => workplane
            .sketch()
            .move_to(center.x + half_foot_thickness, center.y + half_foot_thickness)
            .line_dx(FOOT_EXTENT)
            .line_dy(-foot_thickness)
            .line_dx(-(FOOT_EXTENT * 2.0 + foot_thickness))
            .line_dy(foot_thickness)
            .line_dx(FOOT_EXTENT)
            .line_dy(FOOT_EXTENT)
            .line_dx(foot_thickness)
            .close(),
        FootStyle::PointingDown => workplane
            .sketch()
            .move_to(center.x - half_foot_thickness, center.y - half_foot_thickness)
            .line_dx(-FOOT_EXTENT)
            .line_dy(foot_thickness)
            .line_dx(FOOT_EXTENT * 2.0 + foot_thickness)
            .line_dy(-foot_thickness)
            .line_dx(-FOOT_EXTENT)
            .line_dy(-FOOT_EXTENT)
            .line_dx(-foot_thickness)
            .close(),
        FootStyle::Line => workplane
            .sketch()
            .move_to(center.x - FOOT_EXTENT, center.y + half_foot_thickness)
            .line_dx(FOOT_EXTENT * 2.0)
            .line_dy(-foot_thickness)
            .line_dx(-FOOT_EXTENT * 2.0)
            .close(),
    };

    sketch.fillet(0.7).to_face().extrude(dvec3(0.0, 0.0, z_extrude)).into()
}

fn case_feet(foot_thickness: f64, z_extrude: f64) -> Shape {
    // The vertical distance between the gaps in between rows of keycaps.
    // Moving this amount will take you from the center of one gap to the one
    // above or below it.
    const KEY_ROW_VERTICAL_PITCH: f64 = 18.4333;

    // A center point "origin" for the feet locations.
    // TODO(bschwind) - Add comments or const values for these magic numbers.
    let reference_point = dvec2(PCB_WIDTH / 2.0, ((CASE_BOTTOM - CASE_TOP) / 2.0) + 0.8 + 5.0);
    let upper_left_foot_pos = reference_point + dvec2(-90.0, 18.5 + KEY_ROW_VERTICAL_PITCH + 0.1);
    let bottom_left_foot_pos = upper_left_foot_pos + dvec2(14.0, -(55.3 + KEY_ROW_VERTICAL_PITCH));

    let upper_left_foot =
        case_foot(upper_left_foot_pos, FootStyle::Line, foot_thickness, z_extrude);
    let upper_right_foot = case_foot(
        upper_left_foot_pos + dvec2(171.1, 0.0),
        FootStyle::Line,
        foot_thickness,
        z_extrude,
    );
    let bottom_left_foot =
        case_foot(bottom_left_foot_pos, FootStyle::PointingUp, foot_thickness, z_extrude);
    let bottom_right_foot = case_foot(
        bottom_left_foot_pos + dvec2(152.1, 0.0),
        FootStyle::PointingUp,
        foot_thickness,
        z_extrude,
    );

    upper_left_foot
        .union(&upper_right_foot)
        .union(&bottom_left_foot)
        .union(&bottom_right_foot)
        .into()
}

const PUSH_SLOT_LOCATIONS: &[DVec2] = &[
    DVec2::new(PCB_LEFT + 58.0, PCB_BOTTOM + 8.0),
    DVec2::new(PCB_RIGHT - 58.0, PCB_BOTTOM + 8.0),
];

// A slot to help remove the PCB by pushing through the bottom of the case.
fn push_slot(center: DVec2) -> Shape {
    let mut cutout_workplane = Workplane::xy();
    cutout_workplane.set_translation(dvec3(center.x, center.y, CASE_FLOOR_Z));

    cutout_workplane
        .rect(10.0, 3.0)
        .fillet(1.0)
        .to_face()
        .extrude(dvec3(0.0, 0.0, -CASE_WALL_THICKNESS))
        .into()
}

pub fn shape() -> Shape {
    let inner_box = case_inner_box();
    let top_shelf = pcb_top_shelf();
    let bottom_shelf = pcb_bottom_shelf();
    let usb_cutout = usb_connector_cutout();

    let case = case_outer_box()
        .subtract(&inner_box)
        .fillet_new_edges(0.3)
        .union(&top_shelf)
        .union(&bottom_shelf)
        .subtract(&usb_cutout);

    let new_edges: Vec<_> = case
        .new_edges()
        .filter(|e| e.start_point().y > 0.0) // Only chamfer edges on the exterior of the case
        .collect();

    let case = case.chamfer_edges(1.0, new_edges);

    let mut case = case.into_shape();

    for support_post in SUPPORT_POSTS {
        case = case.union(&support_post.shape()).into();
    }

    let usb_overhang = pcb_usb_overhang();

    case = case.subtract(&usb_overhang).into();

    for feet_cutout in FEET_CUTOUTS {
        let pos = DVec3::from((*feet_cutout, CASE_FLOOR_Z));
        let dir = DVec3::new(0.0, 0.0, -1.0);

        case = case.drill_hole(pos, dir, BOTTOM_CUTOUT_RADIUS);
    }

    for pinhole_pos in PINHOLE_LOCATIONS {
        let pos = DVec3::from((*pinhole_pos, CASE_FLOOR_Z));
        let dir = DVec3::new(0.0, 0.0, -1.0);

        case = case.drill_hole(pos, dir, PINHOLE_BUTTON_RADIUS);
    }

    for slot_center in PUSH_SLOT_LOCATIONS {
        let slot = push_slot(*slot_center);
        case = case.subtract(&slot).into();
    }

    // For exporting to smaller 3D printers
    // let corner_1 = DVec3::new(CASE_LEFT, CASE_BOTTOM, CASE_BOTTOM_Z);
    // let corner_2 = DVec3::new(CASE_RIGHT / 2.0, CASE_TOP, CASE_TOP_Z);
    // let left_half = Shape::box_from_corners(corner_1, corner_2);

    // let corner_1 = DVec3::new(CASE_RIGHT / 2.0, CASE_BOTTOM, CASE_BOTTOM_Z);
    // let corner_2 = DVec3::new(CASE_RIGHT, CASE_TOP, CASE_TOP_Z);
    // let right_half = Shape::box_from_corners(corner_1, corner_2);

    // let shape = shape.intersect(&left_half);

    // shape.write_stl("keyboard_half.stl").unwrap();

    let foot_z_extrude = 2.5;
    let feet_indentation = case_feet(CASE_FOOT_THICKNESS + 0.15, foot_z_extrude);
    let feet = case_feet(CASE_FOOT_THICKNESS, -foot_z_extrude);

    let case = case.subtract(&feet_indentation).into_shape();
    // let case = case.union(&feet).into_shape();

    // let pcb_center = dvec2(PCB_WIDTH / 2.0, ((CASE_BOTTOM - CASE_TOP) / 2.0) + 0.8);
    // Temporary plate to hold the feet
    // let corner_1 = DVec3::new(pcb_center.x - 120.0, pcb_center.y - 45.0, CASE_BOTTOM_Z);
    // let corner_2 = DVec3::new(pcb_center.x + 120.0, pcb_center.y + 45.0, CASE_BOTTOM_Z + 0.5);
    // let plate = Shape::box_from_corners(corner_1, corner_2);

    // let test_plate = plate.union(&feet);

    // test_plate.write_step("feet_plate.step").unwrap();

    // test_plate.into()

    case.write_step("keyboard.step").unwrap();
    feet.write_step("case_feet.step").unwrap();
    case
}
