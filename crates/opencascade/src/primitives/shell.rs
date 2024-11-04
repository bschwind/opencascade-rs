use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::primitives::Wire;

pub struct Shell {
    pub(crate) inner: UniquePtr<ffi::TopoDSShell>,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

impl Shell {
    pub(crate) fn from_shell(shell: &ffi::TopoDSShell) -> Self {
        let inner = ffi::TopoDSShell_to_owned(shell);

        Self { inner }
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = false;
        let mut make_loft = ffi::BRepOffsetAPIThruSections_ctor(is_solid);

        for wire in wires.into_iter() {
            make_loft.pin_mut().add_wire(&wire.as_ref().inner);
        }

        // Set CheckCompatibility to `true` to avoid twisted results.
        make_loft.pin_mut().check_compatibility(true);

        let shape = make_loft.pin_mut().shape();
        let shell = ffi::TopoDS_cast_to_shell(shape);

        Self::from_shell(shell)
    }
}
