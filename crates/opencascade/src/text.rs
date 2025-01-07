use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct Font {
    pub(crate) inner: UniquePtr<ffi::Font_BRepFont>,
}
