use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::primitives::CompoundFace;

pub type FontAspect = ffi::FontAspect;

pub struct Font {
    pub(crate) inner: UniquePtr<ffi::StdPrs_BRepFont>,
}

impl Font {
    pub fn from_name(name: &str, aspect: FontAspect, size: f64) -> Self {
        let inner = ffi::StdPrs_BRepFont_ctor_from_name(&name.to_owned(), aspect.repr, size);
        Self { inner }
    }

    pub fn render_glyph(&mut self, c: char) -> CompoundFace {
        let shape = ffi::StdPrs_BRepFont_RenderGlyph(self.inner.pin_mut(), c as u32);
        let face = ffi::TopoDS_Compound_to_owned(ffi::TopoDS_cast_to_compound(&shape));
        CompoundFace { inner: face }
    }
}
