use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::primitives::Wire;

pub type FontAspect = ffi::FontAspect;

pub struct Font {
    pub(crate) inner: UniquePtr<ffi::Font_BRepFont>,
}

impl Font {
    pub fn from_name(name: &str, aspect: FontAspect, size: f64) -> Self {
        let inner = ffi::Font_BRepFont_ctor_from_name(&name.to_owned(), aspect.repr, size);
        Self { inner }
    }

    pub fn render_glyph(&mut self, c: char) -> Wire {
        let shape = ffi::Font_BRepFont_RenderGlyph(self.inner.pin_mut(), c as u32);
        let wire = ffi::TopoDS_Wire_to_owned(ffi::TopoDS_cast_to_wire(&shape));
        Wire { inner: wire }
    }
}
