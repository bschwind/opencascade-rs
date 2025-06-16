use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

use crate::primitives::Shape;

pub struct BoundingBox {
    pub(crate) inner: UniquePtr<ffi::Bnd_Box>,
}
impl BoundingBox {
    /// Create a new, valid bounding box with zero dimensions and volume.
    pub fn new() -> BoundingBox {
        let mut inner = ffi::Bnd_Box_ctor();
        let p = ffi::new_point(0., 0., 0.);
        inner.pin_mut().Set(&p);
        Self { inner }
    }

    pub fn is_void(&self) -> bool {
        self.inner.IsVoid()
    }

    pub fn get_gap(&self) -> f64 {
        self.inner.GetGap()
    }

    pub fn min(&self) -> DVec3 {
        let p = ffi::Bnd_Box_CornerMin(self.inner.as_ref().unwrap());
        glam::dvec3(p.X(), p.Y(), p.Z())
    }

    pub fn max(&self) -> DVec3 {
        let p = ffi::Bnd_Box_CornerMax(self.inner.as_ref().unwrap());
        glam::dvec3(p.X(), p.Y(), p.Z())
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
        let bb = BoundingBox::new();
        let min = bb.min();
        let max = bb.max();
        assert!(min.x == 0.0 && max.x == 0.0);
        assert!(min.y == 0.0 && max.y == 0.0);
        assert!(min.z == 0.0 && max.z == 0.0);
    }

    #[test]
    fn get_bounding_box_of_sphere() {
        let s = Shape::sphere(1.).build();

        let bb = aabb(&s);

        assert_eq!(bb.min(), glam::dvec3(-1., -1., -1.) + glam::DVec3::NEG_ONE * bb.get_gap());
        assert_eq!(bb.max(), glam::dvec3(1., 1., 1.) + glam::DVec3::ONE * bb.get_gap());
    }

    #[test]
    fn get_bounding_box_of_sphere_transformed() {
        let s = Shape::sphere(1.).at(glam::dvec3(1., 2., 3.)).build();

        let bb = aabb(&s);
        let gap = glam::DVec3::ONE * bb.get_gap();
        assert_eq!(bb.min(), glam::dvec3(0., 1., 2.) - gap);
        assert_eq!(bb.max(), glam::dvec3(2., 3., 4.) + gap);
    }
}
