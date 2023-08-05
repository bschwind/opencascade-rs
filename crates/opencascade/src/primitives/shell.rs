use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct Shell {
    pub(crate) _inner: UniquePtr<ffi::TopoDS_Shell>,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}
