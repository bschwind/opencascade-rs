use crate::{
    primitives::{EdgeIterator, Solid, Wire},
    wasm,
};
use glam::DVec3;

pub struct Face {
    pub(crate) inner: wasm::Face,
}

impl AsRef<Face> for Face {
    fn as_ref(&self) -> &Face {
        self
    }
}

impl Face {
    #[must_use]
    pub fn from_wire(wire: &Wire) -> Self {
        let host_face = wasm::Face::from_wire(&wire.inner);

        Self { inner: host_face }
    }

    #[must_use]
    pub fn fillet(&self, radius: f64) -> Self {
        let host_face = self.inner.fillet(radius);

        Self { inner: host_face }
    }

    #[must_use]
    pub fn extrude(&self, dir: DVec3) -> Solid {
        let host_solid = self.inner.extrude(dir.into());

        Solid { inner: host_solid }
    }

    pub fn outer_wire(&self) -> Wire {
        let host_wire = self.inner.outer_wire();

        Wire { inner: host_wire }
    }

    pub fn center_of_mass(&self) -> DVec3 {
        self.inner.center_of_mass().into()
    }

    pub fn edges(&self) -> EdgeIterator {
        EdgeIterator::new(self)
    }
}
