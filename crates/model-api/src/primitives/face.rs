use crate::{primitives::Wire, wasm};

pub struct Face {
    pub(crate) inner: wasm::Face,
}

impl AsRef<Face> for Face {
    fn as_ref(&self) -> &Face {
        self
    }
}

impl Face {
    pub fn from_wire(wire: &Wire) -> Self {
        let host_face = wasm::Face::from_wire(&wire.inner);

        Self { inner: host_face }
    }

    pub fn fillet(&self, radius: f64) -> Self {
        let host_face = self.inner.fillet(radius);

        Self { inner: host_face }
    }

    pub fn outer_wire(&self) -> Wire {
        let host_wire = self.inner.outer_wire();

        Wire { inner: host_wire }
    }
}
