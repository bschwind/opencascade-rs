use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade::{
    primitives::{Direction, IntoShape, Shape, Solid},
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
const PCB_DIMENSION_TOLERANCE: f64 = 0.0;

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

// PCB
const PCB_TOP: f64 = ORIGIN.y + PCB_DIMENSION_TOLERANCE;
const PCB_TOP_Z: f64 = ORIGIN.z;
const PCB_BOTTOM: f64 = PCB_TOP - PCB_HEIGHT - PCB_DIMENSION_TOLERANCE;
const PCB_BOTTOM_Z: f64 = PCB_TOP_Z - PCB_THICKNESS;
const PCB_LEFT: f64 = ORIGIN.x - PCB_DIMENSION_TOLERANCE;
const PCB_RIGHT: f64 = PCB_LEFT + PCB_WIDTH + PCB_DIMENSION_TOLERANCE;

// Top Plate
const TOP_PLATE_BOTTOM_Z: f64 = 3.4;
const TOP_PLATE_TOP_Z: f64 = TOP_PLATE_BOTTOM_Z + TOP_PLATE_THICKNESS;

// PCB Shelf
const PCB_SHELF_THICKNESS_TOP: f64 = 2.75;
const PCB_SHELF_THICKNESS_BOTTOM: f64 = 4.0;
const PCB_SHELF_HEIGHT: f64 = 4.0;

// Top plate support post locations
const SUPPORT_POST_RADIUS: f64 = 2.25;

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

        let box_part = match self.direction {
            PostDirection::Up => {
                let corner_1 = DVec3::new(
                    pos.x - SUPPORT_POST_RADIUS,
                    pos.y + SUPPORT_POST_DIST_FROM_EDGE,
                    bottom_z,
                );
                let corner_2 = DVec3::new(pos.x + SUPPORT_POST_RADIUS, pos.y, top_z);

                Shape::box_from_corners(corner_1, corner_2)
            },
            PostDirection::Down => {
                let corner_1 = DVec3::new(pos.x - SUPPORT_POST_RADIUS, pos.y, bottom_z);
                let corner_2 = DVec3::new(
                    pos.x + SUPPORT_POST_RADIUS,
                    pos.y - SUPPORT_POST_DIST_FROM_EDGE,
                    top_z,
                );

                Shape::box_from_corners(corner_1, corner_2)
            },
            PostDirection::Left => {
                let corner_1 = DVec3::new(
                    pos.x - SUPPORT_POST_DIST_FROM_EDGE,
                    pos.y - SUPPORT_POST_RADIUS,
                    bottom_z,
                );
                let corner_2 = DVec3::new(pos.x, pos.y + SUPPORT_POST_RADIUS, top_z);

                Shape::box_from_corners(corner_1, corner_2)
            },
            PostDirection::Right => {
                let corner_1 = DVec3::new(pos.x, pos.y - SUPPORT_POST_RADIUS, bottom_z);
                let corner_2 = DVec3::new(
                    pos.x + SUPPORT_POST_DIST_FROM_EDGE,
                    pos.y + SUPPORT_POST_RADIUS,
                    top_z,
                );

                Shape::box_from_corners(corner_1, corner_2)
            },
        };

        cylinder.union(&box_part).subtract(&m2_drill_hole).into()
    }
}

const SUPPORT_POSTS: &[SupportPost] = &[
    SupportPost {
        pos: DVec2::new(119.075, PCB_TOP - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(204.75, PCB_TOP - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(80.95, PCB_BOTTOM + SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
    SupportPost {
        pos: DVec2::new(200.05, PCB_BOTTOM + SUPPORT_POST_DIST_FROM_EDGE),
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
const USB_CUTOUT_PADDING: f64 = 0.1;
const USB_WIDTH: f64 = 9.0;
const USB_HEIGHT: f64 = 7.45;
const USB_DEPTH: f64 = 3.26;
const USB_RADIUS: f64 = 1.43;

const USB_LEFT: f64 = 21.724;
const USB_RIGHT: f64 = USB_LEFT + USB_WIDTH;
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
    let corner_1 = DVec3::new(USB_LEFT - USB_CUTOUT_PADDING, 2.3, PCB_BOTTOM_Z - (USB_DEPTH / 2.0));
    let corner_2 = DVec3::new(
        USB_RIGHT + USB_CUTOUT_PADDING,
        USB_BOTTOM - USB_CUTOUT_PADDING,
        PCB_BOTTOM_Z + USB_CUTOUT_PADDING,
    );

    let squared_shape = Shape::box_from_corners(corner_1, corner_2);

    let mut usb_workplane = Workplane::xz();
    usb_workplane.set_translation(dvec3(
        USB_LEFT + (USB_WIDTH / 2.0),
        USB_BOTTOM,
        PCB_BOTTOM_Z - (USB_DEPTH / 2.0),
    ));
    usb_workplane
        .rect(USB_WIDTH, USB_DEPTH)
        .fillet(USB_RADIUS)
        .to_face()
        .extrude(dvec3(0.0, USB_HEIGHT + CASE_WALL_THICKNESS, 0.0))
        .into_shape()
        .union(&squared_shape)
        .into()
}

// This is the little trapezoidal PCB shape which helps the USB C connector
// extend forward into the case.
fn pcb_usb_overhang() -> Shape {
    let start = CASE_FLOOR_Z;
    Solid::extrude_polygon(
        [
            DVec3::new(19.05, 0.0, start),
            DVec3::new(USB_LEFT - 0.3, 2.381, start),
            DVec3::new(USB_RIGHT + 0.3, 2.381, start),
            DVec3::new(33.337, 0.0, start),
            DVec3::new(33.337, PCB_TOP - PCB_SHELF_THICKNESS_TOP, start),
            DVec3::new(19.05, PCB_TOP - PCB_SHELF_THICKNESS_TOP, start),
        ],
        PCB_TOP_Z - start,
    )
    .unwrap()
    .into()
}

fn case_foot(center: DVec2, pointing_down: bool) -> Shape {
    const FOOT_THICKNESS: f64 = 2.3;
    const HALF_FOOT_THICKNESS: f64 = FOOT_THICKNESS / 2.0;
    const FOOT_EXTENT: f64 = 15.0;

    let sketch = if pointing_down {
        Workplane::xy()
            .sketch()
            .move_to(center.x - HALF_FOOT_THICKNESS, center.y - HALF_FOOT_THICKNESS)
            .line_dx(-FOOT_EXTENT)
            .line_dy(FOOT_THICKNESS)
            .line_dx(FOOT_EXTENT * 2.0 + FOOT_THICKNESS)
            .line_dy(-FOOT_THICKNESS)
            .line_dx(-FOOT_EXTENT)
            .line_dy(-FOOT_EXTENT)
            .line_dx(-FOOT_THICKNESS)
            .close()
    } else {
        Workplane::xy()
            .sketch()
            .move_to(center.x + HALF_FOOT_THICKNESS, center.y + HALF_FOOT_THICKNESS)
            .line_dx(FOOT_EXTENT)
            .line_dy(-FOOT_THICKNESS)
            .line_dx(-(FOOT_EXTENT * 2.0 + FOOT_THICKNESS))
            .line_dy(FOOT_THICKNESS)
            .line_dx(FOOT_EXTENT)
            .line_dy(FOOT_EXTENT)
            .line_dx(FOOT_THICKNESS)
            .close()
    };

    sketch.fillet(0.7).to_face().extrude(dvec3(0.0, 0.0, -5.0)).into()
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

    // For exporting to smaller 3D printers
    // let corner_1 = DVec3::new(CASE_LEFT, CASE_BOTTOM, CASE_BOTTOM_Z);
    // let corner_2 = DVec3::new(CASE_RIGHT / 2.0, CASE_TOP, CASE_TOP_Z);
    // let left_half = Shape::box_from_corners(corner_1, corner_2);

    // let corner_1 = DVec3::new(CASE_RIGHT / 2.0, CASE_BOTTOM, CASE_BOTTOM_Z);
    // let corner_2 = DVec3::new(CASE_RIGHT, CASE_TOP, CASE_TOP_Z);
    // let right_half = Shape::box_from_corners(corner_1, corner_2);

    // let shape = shape.intersect(&left_half);

    // shape.write_stl("keyboard_half.stl").unwrap();

    // case.write_step("keyboard.step").unwrap();

    // case
    // let foot = case_foot(dvec2(0.0, 0.0), pointing_down);

    let pcb_center = dvec2(PCB_WIDTH / 2.0, PCB_HEIGHT / 2.0);

    let upper_left_foot_pos = pcb_center + dvec2(-90.0, 18.5);
    let bottom_left_foot_pos = upper_left_foot_pos + dvec2(14.0, -55.3);

    let upper_left_foot = case_foot(upper_left_foot_pos, true);
    let upper_right_foot = case_foot(upper_left_foot_pos + dvec2(171.1, 0.0), true);
    let bottom_left_foot = case_foot(bottom_left_foot_pos, false);
    let bottom_right_foot = case_foot(bottom_left_foot_pos + dvec2(152.1, 0.0), false);

    // Temporary plate to hold the feet
    let corner_1 = DVec3::new(pcb_center.x - 120.0, pcb_center.y - 35.0 - 10.0, 0.0);
    let corner_2 = DVec3::new(pcb_center.x + 120.0, pcb_center.y + 35.0 - 10.0, 0.5);
    let plate = Shape::box_from_corners(corner_1, corner_2);

    let feet = upper_left_foot
        .union(&upper_right_foot)
        .union(&bottom_left_foot)
        .union(&bottom_right_foot)
        .union(&plate)
        .into_shape();

    feet.write_step("keyboard_case_feet_plate_test.step").unwrap();

    feet
}
