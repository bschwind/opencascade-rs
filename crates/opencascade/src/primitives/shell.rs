use crate::primitives::Wire;
use cxx::UniquePtr;
use opencascade_sys as ffi;

pub struct Shell {
    pub(crate) inner: UniquePtr<ffi::topo_ds::TopoDS_Shell>,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

impl Shell {
    pub(crate) fn from_shell(shell: &ffi::topo_ds::TopoDS_Shell) -> Self {
        let inner = ffi::topo_ds::TopoDS_Shell_to_owned(shell);

        Self { inner }
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = false;
        let mut make_loft = ffi::b_rep_offset_api::BRepOffsetAPI_ThruSections_ctor(is_solid);

        for wire in wires.into_iter() {
            make_loft.pin_mut().AddWire(&wire.as_ref().inner);
        }

        // Set CheckCompatibility to `true` to avoid twisted results.
        make_loft.pin_mut().CheckCompatibility(true);

        let shape = make_loft.pin_mut().Shape();
        let shell = ffi::topo_ds::TopoDS_cast_to_shell(shape);

        Self::from_shell(shell)
    }
}
