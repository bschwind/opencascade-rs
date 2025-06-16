use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

pub struct BoundingBox {
    pub(crate) inner: UniquePtr<ffi::Bnd_Box>,
    pub(crate) min: DVec3,
    pub(crate) max: DVec3,
}
impl BoundingBox {
    pub fn new() -> BoundingBox {
        let mut bnd_box = ffi::Bnd_Box_ctor();
        let p = ffi::new_point(0., 0., 0.);
        bnd_box.pin_mut().Set(&p);
        Self { inner: bnd_box, min: DVec3::ZERO, max: DVec3::ZERO }
    }

    pub fn is_void(&self) -> bool {
        self.inner.IsVoid()
    }

    pub fn min(&mut self) -> DVec3 {
        self.inner.Get(
            &mut self.min.x,
            &mut self.min.y,
            &mut self.min.z,
            &mut self.max.x,
            &mut self.max.y,
            &mut self.max.z,
        );
        self.min
    }

    pub fn max(&mut self) -> DVec3 {
        self.inner.Get(
            &mut self.min.x,
            &mut self.min.y,
            &mut self.min.z,
            &mut self.max.x,
            &mut self.max.y,
            &mut self.max.z,
        );
        self.max
    }
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
}
