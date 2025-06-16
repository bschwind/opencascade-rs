use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct BoundingBox {
    pub(crate) inner: UniquePtr<ffi::Bnd_Box>,
}
impl BoundingBox {
    pub fn new() -> BoundingBox {
        Self { inner: ffi::Bnd_Box_ctor() }
    }

    pub fn is_void(self: &BoundingBox) -> bool {
        self.inner.IsVoid()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_bounding_box() {
        let bb = BoundingBox::new();
        assert!(bb.is_void());
        // let [mut x_min, mut y_min, mut z_min, mut x_max, mut y_max, mut z_max] = [0.; 6];
        // bb.inner.Get(&mut x_min, &mut y_min, &mut z_min, &mut x_max, &mut y_max, &mut z_max);
        // assert!(x_min > 0.);
        // assert!(x_max < 0.);
    }
}
