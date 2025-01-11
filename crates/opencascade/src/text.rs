use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::primitives::Shape;

pub type FontAspect = ffi::FontAspect;

pub struct Font {
    pub(crate) inner: UniquePtr<ffi::StdPrs_BRepFont>,
}

impl Font {
    pub fn from_name(name: &str, aspect: FontAspect, size: f64) -> Self {
        let name: String = name.to_owned();
        let mut inner = ffi::StdPrs_BRepFont_ctor();
        let is_success =
            ffi::StdPrs_BRepFont_Init_with_name(inner.pin_mut(), &name, aspect.repr, size);
        assert!(is_success);
        Self { inner }
    }

    pub fn from_path(path: &str, size: f64) -> Self {
        let path: String = path.to_owned();
        let mut inner = ffi::StdPrs_BRepFont_ctor();
        let is_success = ffi::StdPrs_BRepFont_Init_with_path(inner.pin_mut(), &path, size);
        assert!(is_success);
        Self { inner }
    }

    pub fn render_glyph(&mut self, c: char) -> Shape {
        let inner = ffi::StdPrs_BRepFont_RenderGlyph(self.inner.pin_mut(), c as u32);
        Shape { inner }
    }
}
