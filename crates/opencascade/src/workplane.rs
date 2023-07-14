use crate::primitives::{Edge, Wire};
use glam::{dvec3, DAffine3, DMat3, DVec3, EulerRot};

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
    Custom { x_dir: (f64, f64, f64), normal_dir: (f64, f64, f64) },
}

impl Plane {
    pub fn transform_point(&self, point: DVec3) -> DVec3 {
        self.transform().transform_point3(point)
    }

    pub fn transform(&self) -> DAffine3 {
        match self {
            Self::XY => DAffine3::from_cols(DVec3::X, DVec3::Y, DVec3::Z, DVec3::ZERO),
            Self::YZ => DAffine3::from_cols(DVec3::Y, DVec3::Z, DVec3::X, DVec3::ZERO),
            Self::ZX => DAffine3::from_cols(DVec3::Z, DVec3::X, DVec3::Y, DVec3::ZERO),
            Self::XZ => DAffine3::from_cols(DVec3::X, DVec3::Z, DVec3::NEG_Y, DVec3::ZERO),
            Self::YX => DAffine3::from_cols(DVec3::Y, DVec3::X, DVec3::NEG_Z, DVec3::ZERO),
            Self::ZY => DAffine3::from_cols(DVec3::Z, DVec3::Y, DVec3::NEG_X, DVec3::ZERO),
            Self::Front => DAffine3::from_cols(DVec3::X, DVec3::Y, DVec3::Z, DVec3::ZERO),
            Self::Back => DAffine3::from_cols(DVec3::NEG_X, DVec3::Y, DVec3::NEG_Z, DVec3::ZERO),
            Self::Left => DAffine3::from_cols(DVec3::Z, DVec3::Y, DVec3::NEG_X, DVec3::ZERO),
            Self::Right => DAffine3::from_cols(DVec3::NEG_Z, DVec3::Y, DVec3::X, DVec3::ZERO),
            Self::Top => DAffine3::from_cols(DVec3::X, DVec3::NEG_Z, DVec3::Y, DVec3::ZERO),
            Self::Bottom => DAffine3::from_cols(DVec3::X, DVec3::Z, DVec3::NEG_Y, DVec3::ZERO),
            Self::Custom { x_dir, normal_dir } => {
                let x_axis = dvec3(x_dir.0, x_dir.1, x_dir.2).normalize();
                let z_axis = dvec3(normal_dir.0, normal_dir.1, normal_dir.2).normalize();
                let y_axis = z_axis.cross(x_axis).normalize();

                DAffine3::from_cols(x_axis, y_axis, z_axis, DVec3::ZERO)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Workplane {
    transform: DAffine3,
}

impl Workplane {
    pub fn new(x_dir: DVec3, normal_dir: DVec3) -> Self {
        Self {
            transform: Plane::Custom {
                x_dir: x_dir.normalize().into(),
                normal_dir: normal_dir.normalize().into(),
            }
            .transform(),
        }
    }

    pub fn xy() -> Self {
        Self { transform: Plane::XY.transform() }
    }

    pub fn yz() -> Self {
        Self { transform: Plane::YZ.transform() }
    }

    pub fn zx() -> Self {
        Self { transform: Plane::ZX.transform() }
    }

    pub fn xz() -> Self {
        Self { transform: Plane::XZ.transform() }
    }

    pub fn zy() -> Self {
        Self { transform: Plane::ZY.transform() }
    }

    pub fn yx() -> Self {
        Self { transform: Plane::YX.transform() }
    }

    pub fn origin(&self) -> DVec3 {
        self.transform.translation
    }

    pub fn normal(&self) -> DVec3 {
        self.transform.matrix3.z_axis
    }

    // TODO(bschwind) - Test this.
    pub fn set_rotation(&mut self, (rot_x, rot_y, rot_z): (f64, f64, f64)) {
        let rot_x = rot_x * std::f64::consts::PI / 180.0;
        let rot_y = rot_y * std::f64::consts::PI / 180.0;
        let rot_z = rot_z * std::f64::consts::PI / 180.0;
        let rotation_matrix = DMat3::from_euler(EulerRot::XYZ, rot_x, rot_y, rot_z);

        let translation = self.transform.translation;

        let x_dir = DVec3::X;
        let normal_dir = DVec3::Z;

        self.transform = Plane::Custom {
            x_dir: rotation_matrix.mul_vec3(x_dir).into(),
            normal_dir: rotation_matrix.mul_vec3(normal_dir).into(),
        }
        .transform();

        self.set_translation(translation);
    }

    pub fn rotate_by(&mut self, (rot_x, rot_y, rot_z): (f64, f64, f64)) {
        let rot_x = rot_x * std::f64::consts::PI / 180.0;
        let rot_y = rot_y * std::f64::consts::PI / 180.0;
        let rot_z = rot_z * std::f64::consts::PI / 180.0;
        let rotation_matrix = DMat3::from_euler(EulerRot::XYZ, rot_x, rot_y, rot_z);

        let translation = self.transform.translation;

        let x_dir = rotation_matrix.mul_vec3(DVec3::X);
        let normal_dir = rotation_matrix.mul_vec3(DVec3::Z);

        self.transform = Plane::Custom {
            x_dir: self.transform.transform_vector3(x_dir).into(),
            normal_dir: self.transform.transform_vector3(normal_dir).into(),
        }
        .transform();

        self.set_translation(translation);
    }

    pub fn set_translation(&mut self, pos: DVec3) {
        self.transform.translation = pos;
    }

    pub fn translate_by(&mut self, offset: DVec3) {
        self.transform.translation += offset;
    }

    pub fn transformed(&self, offset: DVec3, rotate: DVec3) -> Self {
        let mut new = self.clone();
        let new_origin = new.to_world_pos(offset);

        new.rotate_by((rotate.x, rotate.y, rotate.z));
        new.transform.translation = new_origin;

        new
    }

    pub fn translated(&self, offset: DVec3) -> Self {
        let mut new = self.clone();
        let new_origin = new.to_world_pos(offset);
        new.transform.translation = new_origin;

        new
    }

    pub fn rotated(&self, rotate: DVec3) -> Self {
        let mut new = self.clone();
        new.rotate_by((rotate.x, rotate.y, rotate.z));

        new
    }

    pub fn to_world_pos(&self, pos: DVec3) -> DVec3 {
        self.transform.transform_point3(pos)
    }

    pub fn to_local_pos(&self, pos: DVec3) -> DVec3 {
        self.transform.inverse().transform_point3(pos)
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

    pub fn circle(&self, x: f64, y: f64, radius: f64) -> Wire {
        let center = self.to_world_pos(dvec3(x, y, 0.0));

        let circle = Edge::circle(center, self.normal(), radius);

        Wire::from_edges([&circle].into_iter())
    }

    pub fn sketch(&self) -> Sketch {
        let cursor = self.to_world_pos(DVec3::ZERO);
        Sketch::new(cursor, self.clone())
    }
}

pub struct Sketch {
    first_point: Option<DVec3>,
    cursor: DVec3, // cursor is in global coordinates
    workplane: Workplane,
    edges: Vec<Edge>,
}

impl Sketch {
    fn new(cursor: DVec3, workplane: Workplane) -> Self {
        Self { first_point: None, cursor, workplane, edges: Vec::new() }
    }

    fn add_edge(&mut self, edge: Edge) {
        if self.first_point.is_none() {
            self.first_point = Some(edge.start_point());
        }

        self.edges.push(edge);
    }

    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        self.cursor = self.workplane.to_world_pos(dvec3(x, y, 0.0));
        self
    }

    pub fn line_to(mut self, x: f64, y: f64) -> Self {
        let new_point = self.workplane.to_world_pos(dvec3(x, y, 0.0));
        let new_edge = Edge::segment(self.cursor, new_point);
        self.cursor = new_point;

        self.add_edge(new_edge);

        self
    }

    pub fn arc(mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64), (x3, y3): (f64, f64)) -> Self {
        let p1 = self.workplane.to_world_pos(dvec3(x1, y1, 0.0));
        let p2 = self.workplane.to_world_pos(dvec3(x2, y2, 0.0));
        let p3 = self.workplane.to_world_pos(dvec3(x3, y3, 0.0));

        let new_arc = Edge::arc(p1, p2, p3);

        self.cursor = p3;

        self.add_edge(new_arc);

        self
    }

    pub fn three_point_arc(self, p2: (f64, f64), p3: (f64, f64)) -> Self {
        let cursor = self.workplane.to_local_pos(self.cursor);
        self.arc((cursor.x, cursor.y), p2, p3)
    }

    pub fn wire(self) -> Wire {
        Wire::from_edges(self.edges.iter())
    }

    pub fn close(mut self) -> Wire {
        let start_point = self.first_point.unwrap();

        let new_edge = Edge::segment(self.cursor, start_point);
        self.add_edge(new_edge);
        Wire::from_edges(self.edges.iter())
    }
}
