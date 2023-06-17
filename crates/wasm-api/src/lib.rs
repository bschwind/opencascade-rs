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

mod bindings {
    extern "C" {
        pub fn new_vertex(x: f64, y: f64, z: f64) -> u32;
    }
}
