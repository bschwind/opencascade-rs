use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

use crate::primitives::Shape;

pub struct BoundingBox {
    pub(crate) inner: UniquePtr<ffi::Bnd_Box>,
    min: DVec3,
    max: DVec3,
}
impl BoundingBox {
    /// Create a new, valid bounding box with zero dimensions and volume.
    pub fn new() -> BoundingBox {
        let mut bnd_box = ffi::Bnd_Box_ctor();
        let p = ffi::new_point(0., 0., 0.);
        bnd_box.pin_mut().Set(&p);
        Self { inner: bnd_box, min: DVec3::ZERO, max: DVec3::ZERO }
    }

    pub fn is_void(&self) -> bool {
        self.inner.IsVoid()
    }

    /// The underlying OCC API uses mutable pointers to get values out of the
    /// `Bnd_Box`. We wrap this method to write those values straight into
    /// `DVec3`s to return in the Rust API.
    fn get(&mut self) {
        self.inner.Get(
            &mut self.min.x,
            &mut self.min.y,
            &mut self.min.z,
            &mut self.max.x,
            &mut self.max.y,
            &mut self.max.z,
        );
    }

    pub fn min(&mut self) -> DVec3 {
        self.get();
        self.min
    }

    pub fn max(&mut self) -> DVec3 {
        self.get();
        self.max
    }
}

/// Compute the axis-aligned bounding box of `shape` using the `BRepBndLib`
/// package.
pub fn aabb(shape: &Shape) -> BoundingBox {
    let mut bb = BoundingBox::new();
    ffi::BRepBndLib_Add(
        shape.inner.as_ref().expect("Input shape ref was null"),
        bb.inner.pin_mut(),
        true,
    );
    bb
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_bounding_box() {
        let bb = BoundingBox::new();
        assert!(!bb.is_void());
    }

    #[test]
    fn get_min_max_of_new_box() {
        let mut bb = BoundingBox::new();
        let min = bb.min();
        let max = bb.max();
        assert!(min.x == 0.0 && max.x == 0.0);
        assert!(min.y == 0.0 && max.y == 0.0);
        assert!(min.z == 0.0 && max.z == 0.0);
    }

    #[test]
    fn get_bounding_box_of_sphere() {
        let s = Shape::sphere(1.).build();

        let mut bb = aabb(&s);

        assert_eq!(bb.min(), glam::dvec3(-1., -1., -1.));
        assert_eq!(bb.max(), glam::dvec3(1., 1., 1.));
    }
}
