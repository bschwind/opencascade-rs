use glam::DVec3;

pub struct Vertex {
    _id: u32,
}

impl Vertex {
    pub fn new(p: DVec3) -> Self {
        unsafe {
            let id = bindings::new_vertex(p.x, p.y, p.z);

            Self { _id: id }
        }
    }
}

pub struct Edge {
    _id: u32,
}

impl Edge {
    pub fn segment(a: DVec3, b: DVec3) -> Self {
        unsafe {
            let id = bindings::new_segment(a.x, a.y, a.z, b.x, b.y, b.z);

            Self { _id: id }
        }
    }
}

mod bindings {
    extern "C" {
        pub fn new_vertex(x: f64, y: f64, z: f64) -> u32;
        pub fn new_segment(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> u32;
    }
}
