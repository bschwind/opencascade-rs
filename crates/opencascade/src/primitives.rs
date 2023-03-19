use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi::{gp_Pnt, new_point};

pub struct Vertex {
    internal: UniquePtr<gp_Pnt>,
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        Self { internal: new_point(point.x, point.y, point.z) }
    }
}

pub struct Edge {}

pub struct Wire {}

pub struct Face {}

pub struct Shell {}

pub struct Solid {}

pub struct Shape {}
