use glam::DVec3;
use opencascade::primitives::{Edge, Face, IntoShape, Shape, Wire};

type Contour = Vec<Vec<(i32, i32)>>;

pub fn shape() -> Shape {
    let outer = contour_to_face(outer_contour()).extrude(DVec3::new(0.0, 0.0, 10.0));
    let inner = contour_to_face(inner_contour()).extrude(DVec3::new(0.0, 0.0, 10.0));
    outer.subtract(&inner).into_shape()
}

const SCALE_FACTOR: f64 = 1.0 / 16.0;
const CENTER_X: i32 = 256;
const CENTER_Y: i32 = 256;

fn contour_to_face(contour: Contour) -> Face {
    let edges: Vec<Edge> = contour
        .into_iter()
        .map(|segment_points| {
            let points = segment_points.into_iter().map(|(x, y)| {
                let x = (x - CENTER_X) as f64 * SCALE_FACTOR;
                let y = (y - CENTER_Y) as f64 * SCALE_FACTOR;
                DVec3::new(x, y, 0.0)
            });
            Edge::bezier(points)
        })
        .collect();
    let wire = Wire::from_edges(&edges);
    Face::from_wire(&wire)
}

fn outer_contour() -> Contour {
    vec![
        vec![(450, 123), (450, 91), (472, 60), (494, 52)],
        vec![(494, 52), (473, -12)],
        vec![(473, -12), (432, -7), (407, 11)],
        vec![(407, 11), (382, 29), (370, 67)],
        vec![(370, 67), (317, -12), (213, -12)],
        vec![(213, -12), (135, -12), (90, 32)],
        vec![(90, 32), (45, 76), (45, 147)],
        vec![(45, 147), (45, 231), (105, 276)],
        vec![(105, 276), (166, 321), (277, 321)],
        vec![(277, 321), (358, 321)],
        vec![(358, 321), (358, 360)],
        vec![(358, 360), (358, 416), (331, 440)],
        vec![(331, 440), (304, 464), (248, 464)],
        vec![(248, 464), (190, 464), (106, 436)],
        vec![(106, 436), (83, 503)],
        vec![(83, 503), (181, 539), (265, 539)],
        vec![(265, 539), (358, 539), (404, 493)],
        vec![(404, 493), (450, 448), (450, 364)],
        vec![(450, 364), (450, 123)],
    ]
}

fn inner_contour() -> Contour {
    vec![
        vec![(234, 57), (313, 57), (358, 139)],
        vec![(358, 139), (358, 260)],
        vec![(358, 260), (289, 260)],
        vec![(289, 260), (143, 260), (143, 152)],
        vec![(143, 152), (143, 105), (166, 81)],
        vec![(166, 81), (189, 57), (234, 57)],
    ]
}
