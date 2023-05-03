use crate::primitives::{Edge, Wire};
use glam::{dvec3, DAffine3, DVec3};

#[derive(Debug, Copy, Clone)]
pub enum Plane {
    XY,
    YZ,
    ZX,
    XZ,
    YX,
    ZY,
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Custom { origin: (f64, f64, f64), x_dir: (f64, f64, f64), normal_dir: (f64, f64, f64) },
}

impl Plane {
    pub fn transform_point(&self, point: DVec3) -> DVec3 {
        self.transform().transform_point3(point)
    }

    pub fn transform(&self) -> DAffine3 {
        match self {
            Self::XY => DAffine3::from_cols(
                dvec3(1.0, 0.0, 0.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(0.0, 0.0, 1.0),
                dvec3(0.0, 0.0, 0.0),
            ),
            Self::YZ => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::ZX => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::XZ => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::YX => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::ZY => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Front => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Back => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Left => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Right => DAffine3::from_cols(
                dvec3(0.0, 0.0, -1.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(1.0, 0.0, 0.0),
                dvec3(0.0, 0.0, 0.0),
            ),
            Self::Top => DAffine3::from_cols(
                dvec3(1.0, 0.0, 0.0),
                dvec3(0.0, 0.0, -1.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(0.0, 0.0, 0.0),
            ),
            Self::Bottom => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Custom { origin, x_dir, normal_dir } => {
                let x_axis = dvec3(x_dir.0, x_dir.1, x_dir.2);
                let z_axis = dvec3(normal_dir.0, normal_dir.1, normal_dir.2);
                let y_axis = z_axis.cross(x_axis);

                DAffine3::from_cols(x_axis, y_axis, z_axis, dvec3(origin.0, origin.1, origin.2))
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Workplane {
    plane: Plane,
}

impl Workplane {
    pub fn xy() -> Self {
        Self { plane: Plane::XY }
    }

    pub fn to_world_pos(&self, pos: DVec3) -> DVec3 {
        self.plane.transform_point(pos)
    }

    pub fn rect(&self, width: f64, height: f64) -> Wire {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let p1 = self.to_world_pos(dvec3(-half_width, half_height, 0.0));
        let p2 = self.to_world_pos(dvec3(half_width, half_height, 0.0));
        let p3 = self.to_world_pos(dvec3(half_width, -half_height, 0.0));
        let p4 = self.to_world_pos(dvec3(-half_width, -half_height, 0.0));

        let top = Edge::segment(p1, p2);
        let right = Edge::segment(p2, p3);
        let bottom = Edge::segment(p3, p4);
        let left = Edge::segment(p4, p1);

        Wire::from_edges([&top, &right, &bottom, &left].into_iter())
    }

    pub fn sketch(&self) -> Sketch {
        Sketch {
            cursor: self.to_world_pos(DVec3::ZERO),
            workplane: self.clone(),
            edges: Vec::new(),
        }
    }
}

pub struct Sketch {
    cursor: DVec3,
    workplane: Workplane,
    edges: Vec<Edge>,
}

impl Sketch {
    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        self.cursor = self.workplane.to_world_pos(dvec3(x, y, 0.0));
        self
    }

    pub fn line_to(mut self, x: f64, y: f64) -> Self {
        let new_point = self.workplane.to_world_pos(dvec3(x, y, 0.0));
        let new_edge = Edge::segment(self.cursor, new_point);
        self.cursor = new_point;

        self.edges.push(new_edge);

        self
    }

    pub fn arc(mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64), (x3, y3): (f64, f64)) -> Self {
        let p1 = self.workplane.to_world_pos(dvec3(x1, y1, 0.0));
        let p2 = self.workplane.to_world_pos(dvec3(x2, y2, 0.0));
        let p3 = self.workplane.to_world_pos(dvec3(x3, y3, 0.0));

        let new_arc = Edge::arc(p1, p2, p3);

        self.cursor = p3;

        self.edges.push(new_arc);

        self
    }

    pub fn wire(self) -> Wire {
        Wire::from_edges(self.edges.iter())
    }
}
