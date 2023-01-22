use glam::{DVec2, DVec3};
use opencascade::Shape;

// All units are in millimeters.
// The top/bottom/left/right conventions relate to 2D rectangles in screen
// coordinates. Values with "_Z" in their name refer to the Z coordinate.
const PCB_THICKNESS: f64 = 1.6;
const PCB_WIDTH: f64 = 285.75;
const PCB_HEIGHT: f64 = 114.3;
const TOP_PLATE_THICKNESS: f64 = 1.6;

// Case
const CASE_WALL_THICKNESS: f64 = 3.0;
const CASE_LIP_HEIGHT: f64 = 1.0;

const CASE_TOP: f64 = PCB_TOP - CASE_WALL_THICKNESS;
const CASE_TOP_Z: f64 = TOP_PLATE_TOP_Z + CASE_LIP_HEIGHT;
const CASE_BOTTOM: f64 = PCB_BOTTOM + CASE_WALL_THICKNESS;
const CASE_LEFT: f64 = PCB_LEFT - CASE_WALL_THICKNESS;
const CASE_RIGHT: f64 = PCB_RIGHT + CASE_WALL_THICKNESS;
const CASE_FLOOR_Z: f64 = PCB_BOTTOM_Z - PCB_SHELF_HEIGHT;
const CASE_BOTTOM_Z: f64 = CASE_FLOOR_Z - CASE_WALL_THICKNESS;

// The origin point for this board is the top left corner
// of the PCB, on the top surface. The PCB rests on this
// shelf. Positive X values go to the right, positive Y
// values go down (like screen coordinates).
const ORIGIN: DVec3 = DVec3::new(0.0, 0.0, 0.0);

const PCB_TOP: f64 = ORIGIN.y;
const PCB_TOP_Z: f64 = ORIGIN.z;
const PCB_BOTTOM: f64 = PCB_TOP + PCB_HEIGHT;
const PCB_BOTTOM_Z: f64 = PCB_TOP_Z - PCB_THICKNESS;
const PCB_LEFT: f64 = ORIGIN.x;
const PCB_RIGHT: f64 = PCB_LEFT + PCB_WIDTH;

const TOP_PLATE_BOTTOM_Z: f64 = 3.4;
const TOP_PLATE_TOP_Z: f64 = TOP_PLATE_BOTTOM_Z + TOP_PLATE_THICKNESS;

const PCB_SHELF_THICKNESS_TOP: f64 = 2.75;
const PCB_SHELF_THICKNESS_BOTTOM: f64 = 4.0;
const PCB_SHELF_HEIGHT: f64 = 4.0;

// Top plate support post locations
const SUPPORT_POST_RADIUS: f64 = 2.25;
const SUPPORT_POST_DIST_FROM_EDGE: f64 = 2.5;

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
        let mut cylinder = Shape::make_cylinder(pos, SUPPORT_POST_RADIUS, top_z - bottom_z);

        let box_part = match self.direction {
            PostDirection::Up => {
                let corner_1 = DVec3::new(
                    pos.x - SUPPORT_POST_RADIUS,
                    pos.y - SUPPORT_POST_DIST_FROM_EDGE,
                    bottom_z,
                );
                let corner_2 = DVec3::new(pos.x + SUPPORT_POST_RADIUS, pos.y, top_z);

                Shape::make_box_point_point(corner_1, corner_2)
            },
            PostDirection::Down => {
                let corner_1 = DVec3::new(pos.x - SUPPORT_POST_RADIUS, pos.y, bottom_z);
                let corner_2 = DVec3::new(
                    pos.x + SUPPORT_POST_RADIUS,
                    pos.y + SUPPORT_POST_DIST_FROM_EDGE,
                    top_z,
                );

                Shape::make_box_point_point(corner_1, corner_2)
            },
            PostDirection::Left => {
                let corner_1 = DVec3::new(
                    pos.x - SUPPORT_POST_DIST_FROM_EDGE,
                    pos.y - SUPPORT_POST_RADIUS,
                    bottom_z,
                );
                let corner_2 = DVec3::new(pos.x, pos.y + SUPPORT_POST_RADIUS, top_z);

                Shape::make_box_point_point(corner_1, corner_2)
            },
            PostDirection::Right => {
                let corner_1 = DVec3::new(pos.x, pos.y - SUPPORT_POST_RADIUS, bottom_z);
                let corner_2 = DVec3::new(
                    pos.x + SUPPORT_POST_DIST_FROM_EDGE,
                    pos.y + SUPPORT_POST_RADIUS,
                    top_z,
                );

                Shape::make_box_point_point(corner_1, corner_2)
            },
        };

        cylinder.union(&box_part);

        cylinder
    }
}

const SUPPORT_POSTS: &[SupportPost] = &[
    SupportPost {
        pos: DVec2::new(117.7, SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(204.75, SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(SUPPORT_POST_DIST_FROM_EDGE, 57.15),
        direction: PostDirection::Left,
    },
    SupportPost {
        pos: DVec2::new(PCB_RIGHT - SUPPORT_POST_DIST_FROM_EDGE, 57.15),
        direction: PostDirection::Right,
    },
    SupportPost {
        pos: DVec2::new(80.95, PCB_HEIGHT - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
    SupportPost {
        pos: DVec2::new(200.05, PCB_HEIGHT - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
];

// The radius of the holes to drill in the bottom of the case to allow
// for threaded M3 rubber feet to pass through and attach to the
// threaded inserts on the PCB.
const BOTTOM_CUTOUT_RADIUS: f64 = 2.1;

const FEET_CUTOUTS: &[DVec2] = &[
    DVec2::new(19.05, 18.25),
    DVec2::new(266.65, 18.25),
    DVec2::new(19.05, 96.45),
    DVec2::new(266.65, 96.45),
];

// USB C Connector Cutout
const USB_CUTOUT_PADDING: f64 = PCB_THICKNESS;
const USB_WIDTH: f64 = 9.0;
const USB_HEIGHT: f64 = 7.45;
const USB_DEPTH: f64 = 3.26;

const USB_LEFT: f64 = 21.724;
const USB_RIGHT: f64 = USB_LEFT + USB_WIDTH;
const USB_TOP: f64 = -3.04;
const USB_BOTTOM: f64 = USB_TOP + USB_HEIGHT;

// Keep out zones for space bar stabilizer
const STABILIZER_SCREW_KEEPOUT_WIDTH: f64 = 8.0;

// A list of the left sides of each stabilizer screw "zones"
// where the PCB shelf should be cut away to accomodate
// stabilizer screws on the bottom of the PCB.
const STABILIZER_KEEPOUTS: &[f64] = &[87.0, 187.24];

// Pinholes for BOOT + reset buttons
const PINHOLE_BUTTON_RADIUS: f64 = 1.1;

const PINHOLE_LOCATIONS: &[DVec2] = &[DVec2::new(35.85, 53.95), DVec2::new(8.425, 86.075)];

fn main() {
    let mut outer_box = case_outer_box();
    let inner_box = case_inner_box();
    let top_shelf = pcb_top_shelf();
    let bottom_shelf = pcb_bottom_shelf();
    let usb_cutout = usb_connector_cutout();

    outer_box.subtract(&inner_box);
    outer_box.union(&top_shelf);
    outer_box.union(&bottom_shelf);
    outer_box.subtract(&usb_cutout);

    for support_post in SUPPORT_POSTS {
        outer_box.union(&support_post.shape());
    }

    // outer_box.fillet_edges(1.0);

    outer_box.write_stl("keyboard_case.stl");
}

fn case_outer_box() -> Shape {
    let corner_1 = DVec3::new(CASE_LEFT, CASE_TOP, CASE_BOTTOM_Z);
    let corner_2 = DVec3::new(CASE_RIGHT, CASE_BOTTOM, CASE_TOP_Z);

    Shape::make_box_point_point(corner_1, corner_2)
}

fn case_inner_box() -> Shape {
    let corner_1 = DVec3::new(PCB_LEFT, PCB_TOP, CASE_FLOOR_Z);
    let corner_2 = DVec3::new(PCB_RIGHT, PCB_BOTTOM, CASE_TOP_Z);

    Shape::make_box_point_point(corner_1, corner_2)
}

fn pcb_top_shelf() -> Shape {
    let corner_1 = DVec3::new(PCB_LEFT, PCB_TOP, CASE_FLOOR_Z);
    let corner_2 = DVec3::new(PCB_RIGHT, PCB_SHELF_THICKNESS_TOP, PCB_BOTTOM_Z);

    Shape::make_box_point_point(corner_1, corner_2)
}

fn pcb_bottom_shelf() -> Shape {
    let corner_1 = DVec3::new(PCB_LEFT, PCB_BOTTOM - PCB_SHELF_THICKNESS_BOTTOM, CASE_FLOOR_Z);
    let corner_2 = DVec3::new(PCB_RIGHT, PCB_BOTTOM, CASE_FLOOR_Z + PCB_SHELF_HEIGHT);

    Shape::make_box_point_point(corner_1, corner_2)
}

fn usb_connector_cutout() -> Shape {
    let corner_1 = DVec3::new(
        USB_LEFT - USB_CUTOUT_PADDING,
        CASE_TOP - USB_CUTOUT_PADDING,
        PCB_BOTTOM_Z - USB_DEPTH - USB_CUTOUT_PADDING,
    );
    let corner_2 = DVec3::new(
        USB_RIGHT + USB_CUTOUT_PADDING,
        USB_BOTTOM + USB_CUTOUT_PADDING,
        PCB_BOTTOM_Z + USB_CUTOUT_PADDING,
    );

    let mut shape = Shape::make_box_point_point(corner_1, corner_2);

    shape.fillet_edges(1.0);
    shape
}
